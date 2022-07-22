pub mod cvec;
pub use cvec::CVec;
mod cstring;
pub use cstring::CString;



pub(crate) struct MakeMaybeUninit<T, const N: usize>(std::marker::PhantomData<fn() -> T>);
impl<T, const N: usize> MakeMaybeUninit<T, N> {
    pub(crate) const VALUE: std::mem::MaybeUninit<T> = std::mem::MaybeUninit::uninit();
    pub(crate) const ARRAY: [std::mem::MaybeUninit<T>; N] = [Self::VALUE; N];
}


pub(crate) trait ICVec {
    type Item;
    const CAPACITY: usize;

    fn len(&self) -> usize;

    unsafe fn set_len(&mut self, new_len: usize);

    fn as_slice(&self) -> &[Self::Item] {
        let len = self.len();
        unsafe {
            std::slice::from_raw_parts(self.as_ptr(), len)
        }
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        let len = self.len();
        unsafe {
            std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
        }
    }

    fn as_ptr(&self) -> *const Self::Item;

    fn as_mut_ptr(&mut self) -> *mut Self::Item;

    fn push(&mut self, element: Self::Item) {
        self.try_push(element).unwrap()
    }

    fn try_push(&mut self, element: Self::Item) -> Result<(), crate::errors::CapacityError<Self::Item>> {
        if self.len() < Self::CAPACITY {
            unsafe {
                self.push_unchecked(element);
            }
            Ok(())
        } else {
            Err(crate::errors::CapacityError::new(element))
        }
    }

    unsafe fn push_unchecked(&mut self, element: Self::Item) {
        let len = self.len();
        debug_assert!(len < Self::CAPACITY);
        std::ptr::write(self.as_mut_ptr().add(len), element);
        self.set_len(len + 1);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }
        unsafe {
            let new_len = self.len() - 1;
            self.set_len(new_len);
            Some(std::ptr::read(self.as_ptr().add(new_len)))
        }
    }

    fn clear(&mut self) {
        self.truncate(0)
    }

    fn truncate(&mut self, new_len: usize) {
        unsafe {
            let len = self.len();
            if new_len < len {
                self.set_len(new_len);
                let tail = std::slice::from_raw_parts_mut(self.as_mut_ptr().add(new_len), len - new_len);
                std::ptr::drop_in_place(tail);
            }
        }
    }
}
pub(crate) unsafe fn encode_utf8(ch: char, ptr: *mut u8, len: usize) -> Result<usize,crate::nulls::Undefined> {
    let code = ch as u32;
    if code < 0x80 && len >= 1 {
        ptr.add(0).write(code as u8);
        return Ok(1);
    } else if code < 0x800 && len >= 2 {
        ptr.add(0).write((code >> 6 & 0x1F) as u8 | 0b1100_0000);
        ptr.add(1).write((code & 0x3F) as u8 | 0b1000_0000);
        return Ok(2);
    } else if code < 0x10000 && len >= 3 {
        ptr.add(0).write((code >> 12 & 0x0F) as u8 | 0b1110_0000);
        ptr.add(1).write((code >>  6 & 0x3F) as u8 | 0b1000_0000);
        ptr.add(2).write((code & 0x3F) as u8 | 0b1000_0000);
        return Ok(3);
    } else if len >= 4 {
        ptr.add(0).write((code >> 18 & 0x07) as u8 | 0b1111_0000);
        ptr.add(1).write((code >> 12 & 0x3F) as u8 | 0b1000_0000);
        ptr.add(2).write((code >>  6 & 0x3F) as u8 | 0b1000_0000);
        ptr.add(3).write((code & 0x3F) as u8 | 0b1000_0000);
        return Ok(4);
    };
    Err(crate::nulls::Undefined)
}