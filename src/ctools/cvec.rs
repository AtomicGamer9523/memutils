use crate::{
    errors::CapacityError,
    ctools::{
        MakeMaybeUninit,
        ICVec
    }
};
use std::{
    borrow::{
        BorrowMut,
        Borrow
    },
    hash::{
        Hasher,
        Hash
    },
    mem::{
        ManuallyDrop,
        MaybeUninit,
        self
    },
    ops::{
        RangeBounds,
        DerefMut,
        Deref,
        Bound
    },
    iter,
    cmp,
    ptr,
    fmt,
    io
};




pub struct CVec<T, const LEN: usize> {
    dat: [MaybeUninit<T>; LEN],
    len: u32,
}

impl<T, const LEN: usize> Drop for CVec<T, LEN> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const LEN: usize> CVec<T, LEN> {
    const CAPACITY: usize = LEN;

    pub fn new() -> CVec<T, LEN> {
        unsafe {
            CVec { dat: MaybeUninit::uninit().assume_init(), len: 0 }
        }
    }

    pub const fn new_const() -> CVec<T, LEN> {
        CVec { dat: MakeMaybeUninit::ARRAY, len: 0 }
    }

    pub const fn len(&self) -> usize { self.len as usize }

    pub const fn is_empty(&self) -> bool { self.len() == 0 }

    pub const fn capacity(&self) -> usize { LEN }

    pub const fn is_full(&self) -> bool { self.len() == self.capacity() }

    pub const fn remaining_capacity(&self) -> usize {
        self.capacity() - self.len()
    }

    pub fn push(&mut self, element: T) {
        ICVec::push(self, element)
    }

    pub fn try_push(&mut self, element: T) -> Result<(), CapacityError<T>> {
        ICVec::try_push(self, element)
    }

    pub unsafe fn push_unchecked(&mut self, element: T) {
        ICVec::push_unchecked(self, element)
    }

    pub fn truncate(&mut self, new_len: usize) {
        ICVec::truncate(self, new_len)
    }

    pub fn clear(&mut self) {
        ICVec::clear(self)
    }

    unsafe fn get_unchecked_ptr(&mut self, index: usize) -> *mut T {
        self.as_mut_ptr().add(index)
    }

    pub fn insert(&mut self, index: usize, element: T) {
        self.try_insert(index, element).unwrap()
    }

    fn try_insert(&mut self, index: usize, element: T) -> Result<(), CapacityError<T>> {
        if index > self.len() {
            crate::cap_panic!("try_insert", index, self.len())
        }
        if self.len() == self.capacity() {
            return Err(CapacityError::new(element));
        }
        let len = self.len();
        unsafe {
            {
                let p: *mut _ = self.get_unchecked_ptr(index);
                ptr::copy(p, p.offset(1), len - index);
                ptr::write(p, element);
            }
            self.set_len(len + 1);
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        ICVec::pop(self)
    }

    pub fn swap_remove(&mut self, index: usize) -> T {
        self.swap_pop(index)
        .unwrap_or_else(|| {
            crate::cap_panic!("swap_remove", index, self.len())
        })
    }

    fn swap_pop(&mut self, index: usize) -> Option<T> {
        let len = self.len();
        if index >= len {
            return None;
        }
        self.swap(index, len - 1);
        self.pop()
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.pop_at(index)
        .unwrap_or_else(|| {
            crate::cap_panic!("remove", index, self.len())
        })
    }

    fn pop_at(&mut self, index: usize) -> Option<T> {
        if index >= self.len() {
            None
        } else {
            self.drain(index..index + 1).next()
        }
    }

    pub fn retain<F: FnMut(&mut T) -> bool>(&mut self, mut f: F) {
        let original_len = self.len();
        unsafe {
            self.set_len(0)
        };
        struct BackshiftOnDrop<'a, T, const LEN: usize> {
            v: &'a mut CVec<T, LEN>,
            processed_len: usize,
            deleted_cnt: usize,
            original_len: usize,
        }
        impl<T, const LEN: usize> Drop for BackshiftOnDrop<'_, T, LEN> {
            fn drop(&mut self) {
                if self.deleted_cnt > 0 {
                    unsafe {
                        ptr::copy(
                            self.v.as_ptr().add(self.processed_len),
                            self.v.as_mut_ptr().add(self.processed_len - self.deleted_cnt),
                            self.original_len - self.processed_len
                        );
                    }
                }
                unsafe {
                    self.v.set_len(self.original_len - self.deleted_cnt);
                }
            }
        }
        let mut g = BackshiftOnDrop { v: self, processed_len: 0, deleted_cnt: 0, original_len };
        fn process_one<F: FnMut(&mut T) -> bool, T, const LEN: usize, const DELETED: bool>(
            f: &mut F,
            g: &mut BackshiftOnDrop<'_, T, LEN>
        ) -> bool {
            let cur = unsafe { g.v.as_mut_ptr().add(g.processed_len) };
            if !f(unsafe { &mut *cur }) {
                g.processed_len += 1;
                g.deleted_cnt += 1;
                unsafe { ptr::drop_in_place(cur) };
                return false;
            }
            if DELETED {
                unsafe {
                    let hole_slot = g.v.as_mut_ptr().add(g.processed_len - g.deleted_cnt);
                    ptr::copy_nonoverlapping(cur, hole_slot, 1);
                }
            }
            g.processed_len += 1;
            true
        }
        while g.processed_len != original_len {
            if !process_one::<F, T, LEN, false>(&mut f, &mut g) {
                break;
            }
        }
        while g.processed_len != original_len {
            process_one::<F, T, LEN, true>(&mut f, &mut g);
        }

        drop(g);
    }

    unsafe fn set_len(&mut self, length: usize) {
        self.len = length as u32;
    }

    pub fn try_extend_from_slice(&mut self, other: &[T]) -> Result<(), CapacityError>
        where T: Copy,
    {
        if self.remaining_capacity() < other.len() {
            return Err(CapacityError::new(()));
        }

        let self_len = self.len();
        let other_len = other.len();

        unsafe {
            let dst = self.get_unchecked_ptr(self_len);
            ptr::copy_nonoverlapping(other.as_ptr(), dst, other_len);
            self.set_len(self_len + other_len);
        }
        Ok(())
    }

    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<T, LEN> {
        let len = self.len();
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i.saturating_add(1),
        };
        let end = match range.end_bound() {
            Bound::Excluded(&j) => j,
            Bound::Included(&j) => j.saturating_add(1),
            Bound::Unbounded => len,
        };
        self.drain_range(start, end)
    }

    fn drain_range(&mut self, start: usize, end: usize) -> Drain<T, LEN> {
        let len = self.len();
        let range_slice: *const _ = &self[start..end];
        self.len = start as u32;
        unsafe {
            Drain {
                tail_start: end,
                tail_len: len - end,
                iter: (*range_slice).iter(),
                vec: self as *mut _,
            }
        }
    }
    pub fn into_inner(self) -> Result<[T; LEN], Self> {
        if self.len() < self.capacity() {
            Err(self)
        } else {
            unsafe { Ok(self.into_inner_unchecked()) }
        }
    }
    pub unsafe fn into_inner_unchecked(self) -> [T; LEN] {
        debug_assert_eq!(self.len(), self.capacity());
        let self_ = ManuallyDrop::new(self);
        let array = ptr::read(self_.as_ptr() as *const [T; LEN]);
        array
    }
    pub fn take(&mut self) -> Self  {
        mem::replace(self, Self::new())
    }

    pub fn as_slice(&self) -> &[T] {
        ICVec::as_slice(self)
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        ICVec::as_mut_slice(self)
    }

    pub fn as_ptr(&self) -> *const T {
        ICVec::as_ptr(self)
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        ICVec::as_mut_ptr(self)
    }
}

impl<T, const LEN: usize> ICVec for CVec<T, LEN> {
    type Item = T;
    const CAPACITY: usize = LEN;
    fn len(&self) -> usize { self.len() }
    unsafe fn set_len(&mut self, length: usize) {
        debug_assert!(length <= LEN);
        self.len = length as u32;
    }
    fn as_ptr(&self) -> *const Self::Item {
        self.dat.as_ptr() as _
    }
    fn as_mut_ptr(&mut self) -> *mut Self::Item {
        self.dat.as_mut_ptr() as _
    }
}

impl<T, const LEN: usize> Deref for CVec<T, LEN> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const LEN: usize> DerefMut for CVec<T, LEN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T, const LEN: usize> From<[T; LEN]> for CVec<T, LEN> {
    fn from(array: [T; LEN]) -> Self {
        let array = ManuallyDrop::new(array);
        let mut vec = <CVec<T, LEN>>::new();
        unsafe {
            (&*array as *const [T; LEN] as *const [MaybeUninit<T>; LEN])
                .copy_to_nonoverlapping(&mut vec.dat as *mut [MaybeUninit<T>; LEN], 1);
            vec.set_len(LEN);
        }
        vec
    }
}

impl<T: Clone, const LEN: usize> std::convert::TryFrom<&[T]> for CVec<T, LEN> {
    type Error = CapacityError;
    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        if Self::CAPACITY < slice.len() {
            Err(CapacityError::new(()))
        } else {
            let mut array = Self::new();
            array.extend_from_slice(slice);
            Ok(array)
        }
    }
}

impl<'a, T: 'a, const LEN: usize> IntoIterator for &'a CVec<T, LEN> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a, T: 'a, const LEN: usize> IntoIterator for &'a mut CVec<T, LEN> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}

impl<T, const LEN: usize> IntoIterator for CVec<T, LEN> {
    type Item = T;
    type IntoIter = IntoIter<T, LEN>;
    fn into_iter(self) -> IntoIter<T, LEN> {
        IntoIter { index: 0, v: self, }
    }
}

pub struct IntoIter<T, const LEN: usize> {
    index: usize,
    v: CVec<T, LEN>,
}

impl<T, const LEN: usize> Iterator for IntoIter<T, LEN> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.v.len() {
            None
        } else {
            unsafe {
                let index = self.index;
                self.index = index + 1;
                Some(ptr::read(self.v.get_unchecked_ptr(index)))
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.v.len() - self.index;
        (len, Some(len))
    }
}

impl<T, const LEN: usize> DoubleEndedIterator for IntoIter<T, LEN> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == self.v.len() {
            None
        } else {
            unsafe {
                let new_len = self.v.len() - 1;
                self.v.set_len(new_len);
                Some(ptr::read(self.v.get_unchecked_ptr(new_len)))
            }
        }
    }
}

impl<T, const LEN: usize> ExactSizeIterator for IntoIter<T, LEN> {}

impl<T, const LEN: usize> Drop for IntoIter<T, LEN> {
    fn drop(&mut self) {
        let index = self.index;
        let len = self.v.len();
        unsafe {
            self.v.set_len(0);
            let elements = std::slice::from_raw_parts_mut(
                self.v.get_unchecked_ptr(index),
                len - index);
            ptr::drop_in_place(elements);
        }
    }
}

impl<T: Clone, const LEN: usize> Clone for IntoIter<T, LEN> {
    fn clone(&self) -> IntoIter<T, LEN> {
        let mut v = CVec::new();
        v.extend_from_slice(&self.v[self.index..]);
        v.into_iter()
    }
}

impl<T: fmt::Debug, const LEN: usize> fmt::Debug for IntoIter<T, LEN> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list()
        .entries(&self.v[self.index..])
        .finish()
    }
}

pub struct Drain<'a, T: 'a, const LEN: usize> {
    tail_start: usize,
    tail_len: usize,
    iter: std::slice::Iter<'a, T>,
    vec: *mut CVec<T, LEN>,
}

unsafe impl<'a, T: Sync, const LEN: usize> Sync for Drain<'a, T, LEN> {}
unsafe impl<'a, T: Send, const LEN: usize> Send for Drain<'a, T, LEN> {}

impl<'a, T: 'a, const LEN: usize> Iterator for Drain<'a, T, LEN> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|elt|
            unsafe {
                ptr::read(elt as *const _)
            }
        )
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T: 'a, const LEN: usize> DoubleEndedIterator for Drain<'a, T, LEN> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|elt|
            unsafe {
                ptr::read(elt as *const _)
            }
        )
    }
}

impl<'a, T: 'a, const LEN: usize> ExactSizeIterator for Drain<'a, T, LEN> {}

impl<'a, T: 'a, const LEN: usize> Drop for Drain<'a, T, LEN> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
        if self.tail_len > 0 {
            unsafe {
                let source_vec = &mut *self.vec;
                let start = source_vec.len();
                let tail = self.tail_start;
                let src = source_vec.as_ptr().add(tail);
                let dst = source_vec.as_mut_ptr().add(start);
                ptr::copy(src, dst, self.tail_len);
                source_vec.set_len(start + self.tail_len);
            }
        }
    }
}

struct ScopeExitGuard<T, Data, F> where F: FnMut(&Data, &mut T) {
    value: T,
    data: Data,
    f: F,
}

impl<T, Data, F> Drop for ScopeExitGuard<T, Data, F> where F: FnMut(&Data, &mut T) {
    fn drop(&mut self) {
        (self.f)(&self.data, &mut self.value)
    }
}

impl<T, const LEN: usize> Extend<T> for CVec<T, LEN> {
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        unsafe {
            self.extend_from_iter::<_, true>(iter)
        }
    }
}

#[inline(never)]
#[cold]
fn extend_panic() {
    panic!("CVec: capacity exceeded in extend/from_iter");
}

impl<T, const LEN: usize> CVec<T, LEN> {
    pub(crate) unsafe fn extend_from_iter<I: IntoIterator<Item = T>, const CHECK: bool>(&mut self, iterable: I) {
        let take = self.capacity() - self.len();
        let len = self.len();
        let mut ptr = raw_ptr_add(self.as_mut_ptr(), len);
        let end_ptr = raw_ptr_add(ptr, take);
        let mut guard = ScopeExitGuard {
            value: &mut self.len,
            data: len,
            f: move |&len, self_len| {
                **self_len = len as u32;
            }
        };
        let mut iter = iterable.into_iter();
        loop {
            if let Some(elt) = iter.next() {
                if ptr == end_ptr && CHECK { extend_panic(); }
                debug_assert_ne!(ptr, end_ptr);
                ptr.write(elt);
                ptr = raw_ptr_add(ptr, 1);
                guard.data += 1;
            } else {
                return;
            }
        }
    }
    pub(crate) fn extend_from_slice(&mut self, slice: &[T]) where T: Clone {
        let take = self.capacity() - self.len();
        unsafe {
            let slice = if take < slice.len() { &slice[..take] } else { slice };
            self.extend_from_iter::<_, false>(slice.iter().cloned());
        }
    }
}
unsafe fn raw_ptr_add<T>(ptr: *mut T, offset: usize) -> *mut T {
    if mem::size_of::<T>() == 0 {
        (ptr as usize).wrapping_add(offset) as _
    } else {
        ptr.add(offset)
    }
}
impl<T, const LEN: usize> iter::FromIterator<T> for CVec<T, LEN> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut array = CVec::new();
        array.extend(iter);
        array
    }
}
impl<T: Clone, const LEN: usize> Clone for CVec<T, LEN> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
    fn clone_from(&mut self, rhs: &Self) {
        let prefix = cmp::min(self.len(), rhs.len());
        self[..prefix].clone_from_slice(&rhs[..prefix]);

        if prefix < self.len() {
            self.truncate(prefix);
        } else {
            let rhs_elems = &rhs[self.len()..];
            self.extend_from_slice(rhs_elems);
        }
    }
}

impl<T: Hash, const LEN: usize> Hash for CVec<T, LEN> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<T: PartialEq, const LEN: usize> PartialEq for CVec<T, LEN> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T: PartialEq, const LEN: usize> PartialEq<[T]> for CVec<T, LEN> {
    fn eq(&self, other: &[T]) -> bool {
        **self == *other
    }
}
impl<T, const LEN: usize> Eq for CVec<T, LEN> where T: Eq {}
impl<T, const LEN: usize> Borrow<[T]> for CVec<T, LEN> {
    fn borrow(&self) -> &[T] { self }
}
impl<T, const LEN: usize> BorrowMut<[T]> for CVec<T, LEN> {
    fn borrow_mut(&mut self) -> &mut [T] { self }
}
impl<T, const LEN: usize> AsRef<[T]> for CVec<T, LEN> {
    fn as_ref(&self) -> &[T] { self }
}
impl<T, const LEN: usize> AsMut<[T]> for CVec<T, LEN> {
    fn as_mut(&mut self) -> &mut [T] { self }
}
impl<T, const LEN: usize> fmt::Debug for CVec<T, LEN> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { (**self).fmt(f) }
}
impl<T, const LEN: usize> Default for CVec<T, LEN> {
    fn default() -> CVec<T, LEN> {
        CVec::new()
    }
}
impl<T, const LEN: usize> PartialOrd for CVec<T, LEN> where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        (**self).partial_cmp(other)
    }
    fn lt(&self, other: &Self) -> bool {
        (**self).lt(other)
    }
    fn le(&self, other: &Self) -> bool {
        (**self).le(other)
    }
    fn ge(&self, other: &Self) -> bool {
        (**self).ge(other)
    }
    fn gt(&self, other: &Self) -> bool {
        (**self).gt(other)
    }
}

impl<T, const LEN: usize> Ord for CVec<T, LEN> where T: Ord {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (**self).cmp(other)
    }
}
impl<const LEN: usize> io::Write for CVec<u8, LEN> {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let len = cmp::min(self.remaining_capacity(), data.len());
        let _result = self.try_extend_from_slice(&data[..len]);
        Ok(len)
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}