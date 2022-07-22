use crate::{
    ctools::MakeMaybeUninit,
    errors::CapacityError
};
use std::{
    mem::MaybeUninit,
    borrow::Borrow,
    hash::{
        Hasher,
        Hash
    },
    str::{
        Utf8Error,
        FromStr,
        self
    },
    ops::{
        DerefMut,
        Deref
    },
    slice,
    cmp,
    fmt,
    ptr
};






#[derive(Copy)]
pub struct CString<const LEN: usize> {
    dat: [MaybeUninit<u8>; LEN],
    len: u32
}
impl<const LEN: usize> Default for CString<LEN> {
    fn default() -> CString<LEN> {
        CString::new()
    }
}

impl<const LEN: usize> CString<LEN>{
    pub fn new() -> CString<LEN> {
        unsafe {
            CString { dat: MaybeUninit::uninit().assume_init(), len: 0 }
        }
    }

    pub const fn new_const() -> CString<LEN> {
        CString { dat: MakeMaybeUninit::ARRAY, len: 0 }
    }

    pub const fn len(&self) -> usize { self.len as usize }

    pub const fn is_empty(&self) -> bool { self.len() == 0 }

    pub fn from(s: &str) -> Result<Self, CapacityError<&str>> {
        let mut arraystr = Self::new();
        arraystr.try_push_str(s)?;
        Ok(arraystr)
    }

    pub fn from_byte_string(b: &[u8; LEN]) -> Result<Self, Utf8Error> {
        let len = str::from_utf8(b)?.len();
        debug_assert_eq!(len, LEN);
        let mut vec = Self::new();
        unsafe {
            (b as *const [u8; LEN] as *const [MaybeUninit<u8>; LEN])
                .copy_to_nonoverlapping(&mut vec.dat as *mut [MaybeUninit<u8>; LEN], 1);
            vec.set_len(LEN);
        }
        Ok(vec)
    }

    pub fn zero_filled() -> Self {
        unsafe {
            CString {
                dat: MaybeUninit::zeroed().assume_init(),
                len: LEN as _
            }
        }
    }

    pub const fn capacity(&self) -> usize { LEN }

    pub const fn is_full(&self) -> bool { self.len() == self.capacity() }

    pub const fn remaining_capacity(&self) -> usize {
        self.capacity() - self.len()
    }

    pub fn push(&mut self, c: char) {
        self.try_push(c).unwrap();
    }

    fn try_push(&mut self, c: char) -> Result<(), CapacityError<char>> {
        let len = self.len();
        unsafe {
            let ptr = self.as_mut_ptr().add(len);
            let remaining_capacity = self.capacity() - len;
            match crate::ctools::encode_utf8(c, ptr, remaining_capacity) {
                Ok(n) => {
                    self.set_len(len + n);
                    Ok(())
                }
                Err(_) => Err(CapacityError::new(c)),
            }
        }
    }

    pub fn push_str(&mut self, s: &str) {
        self.try_push_str(s).unwrap()
    }

    fn try_push_str<'a>(&mut self, s: &'a str) -> Result<(), CapacityError<&'a str>> {
        if s.len() > self.capacity() - self.len() {
            return Err(CapacityError::new(s));
        }
        unsafe {
            let dst = self.as_mut_ptr().add(self.len());
            let src = s.as_ptr();
            ptr::copy_nonoverlapping(src, dst, s.len());
            let newl = self.len() + s.len();
            self.set_len(newl);
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Option<char> {
        let ch = match self.chars().rev().next() {
            Some(ch) => ch,
            None => return None,
        };
        let new_len = self.len() - ch.len_utf8();
        unsafe {
            self.set_len(new_len);
        }
        Some(ch)
    }

    pub fn truncate(&mut self, new_len: usize) {
        if new_len <= self.len() {
            assert!(self.is_char_boundary(new_len));
            unsafe { 
                self.set_len(new_len);
            }
        }
    }
    pub fn remove(&mut self, idx: usize) -> char {
        let ch = match self[idx..].chars().next() {
            Some(ch) => ch,
            None => panic!("cannot remove a char from the end of a string"),
        };

        let next = idx + ch.len_utf8();
        let len = self.len();
        unsafe {
            ptr::copy(self.as_ptr().add(next),
                    self.as_mut_ptr().add(idx),
                    len - next);
            self.set_len(len - (next - idx));
        }
        ch
    }

    pub fn clear(&mut self) {
        unsafe {
            self.set_len(0);
        }
    }

    pub unsafe fn set_len(&mut self, length: usize) {
        self.len = length as u32;
    }

    pub fn as_str(&self) -> &str {
        self
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        self
    }

    fn as_ptr(&self) -> *const u8 {
        self.dat.as_ptr() as *const u8
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.dat.as_mut_ptr() as *mut u8
    }
}

impl<const LEN: usize> Deref for CString<LEN> {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe {
            let sl = slice::from_raw_parts(self.as_ptr(), self.len());
            str::from_utf8_unchecked(sl)
        }
    }
}

impl<const LEN: usize> DerefMut for CString<LEN> {

    fn deref_mut(&mut self) -> &mut str {
        unsafe {
            let len = self.len();
            let sl = slice::from_raw_parts_mut(self.as_mut_ptr(), len);
            str::from_utf8_unchecked_mut(sl)
        }
    }
}

impl<const LEN: usize> PartialEq for CString<LEN> {
    fn eq(&self, rhs: &Self) -> bool {
        **self == **rhs
    }
}

impl<const LEN: usize> PartialEq<str> for CString<LEN> {
    fn eq(&self, rhs: &str) -> bool {
        &**self == rhs
    }
}

impl<const LEN: usize> PartialEq<CString<LEN>> for str {
    fn eq(&self, rhs: &CString<LEN>) -> bool {
        self == &**rhs
    }
}

impl<const LEN: usize> Eq for CString<LEN> {}

impl<const LEN: usize> Hash for CString<LEN> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        (**self).hash(h)
    }
}

impl<const LEN: usize> Borrow<str> for CString<LEN> {
    fn borrow(&self) -> &str { self }
}

impl<const LEN: usize> AsRef<str> for CString<LEN> {
    fn as_ref(&self) -> &str { self }
}

impl<const LEN: usize> fmt::Debug for CString<LEN> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { (**self).fmt(f) }
}

impl<const LEN: usize> fmt::Display for CString<LEN> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { (**self).fmt(f) }
}

impl<const LEN: usize> fmt::Write for CString<LEN> {
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.try_push(c).map_err(|_| fmt::Error)
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.try_push_str(s).map_err(|_| fmt::Error)
    }
}

impl<const LEN: usize> Clone for CString<LEN> {
    fn clone(&self) -> CString<LEN> {
        *self
    }
    fn clone_from(&mut self, rhs: &Self) {
        self.clear();
        self.try_push_str(rhs).ok();
    }
}

impl<const LEN: usize> PartialOrd for CString<LEN> {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        (**self).partial_cmp(&**rhs)
    }
    fn lt(&self, rhs: &Self) -> bool { **self < **rhs }
    fn le(&self, rhs: &Self) -> bool { **self <= **rhs }
    fn gt(&self, rhs: &Self) -> bool { **self > **rhs }
    fn ge(&self, rhs: &Self) -> bool { **self >= **rhs }
}

impl<const LEN: usize> PartialOrd<str> for CString<LEN> {
    fn partial_cmp(&self, rhs: &str) -> Option<cmp::Ordering> {
        (**self).partial_cmp(rhs)
    }
    fn lt(&self, rhs: &str) -> bool { &**self < rhs }
    fn le(&self, rhs: &str) -> bool { &**self <= rhs }
    fn gt(&self, rhs: &str) -> bool { &**self > rhs }
    fn ge(&self, rhs: &str) -> bool { &**self >= rhs }
}

impl<const LEN: usize> PartialOrd<CString<LEN>> for str {
    fn partial_cmp(&self, rhs: &CString<LEN>) -> Option<cmp::Ordering> {
        self.partial_cmp(&**rhs)
    }
    fn lt(&self, rhs: &CString<LEN>) -> bool { self < &**rhs }
    fn le(&self, rhs: &CString<LEN>) -> bool { self <= &**rhs }
    fn gt(&self, rhs: &CString<LEN>) -> bool { self > &**rhs }
    fn ge(&self, rhs: &CString<LEN>) -> bool { self >= &**rhs }
}

impl<const LEN: usize> Ord for CString<LEN> {
    fn cmp(&self, rhs: &Self) -> cmp::Ordering {
        (**self).cmp(&**rhs)
    }
}

impl<const LEN: usize> FromStr for CString<LEN> {
    type Err = CapacityError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from(s).map_err(CapacityError::simplify)
    }
}