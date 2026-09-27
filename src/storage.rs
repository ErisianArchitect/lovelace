
use ::core::{
    ptr::{
        NonNull,
    },
    marker::{
        PhantomData,
    },
};

use crate::{
    alloc::{
        Alignment,
        aligned_alloc,
        dealloc,
        dont_need,
    },
};

#[repr(C)]
#[derive(Debug)]
pub struct Storage<'a, T: 'a + Sized> {
    pub(crate) ptr: NonNull<T>,
    pub(crate) len: usize,
    pub(crate) capacity: usize,
    pub(crate) _phantom: PhantomData<(&'a T,)>,
}

impl<'a, T: 'a + Sized> Storage<'a, T> {
    #[must_use]
    #[inline(always)]
    pub(crate) const unsafe fn from_raw_parts(ptr: NonNull<T>, len: usize, capacity: usize) -> Self {
        Self {
            ptr,
            len,
            capacity,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn new(capacity: usize) -> Self {
        let size = capacity * size_of::<T>();
        let ptr = unsafe { aligned_alloc(Alignment::KiB4, size) };
        Self {
            ptr: ptr.cast(),
            len: 0usize,
            capacity,
            _phantom: PhantomData,
        }
    }

    /// Push `value` to the storage, returning the index that it was added to.
    ///
    /// If the storage is already at capacity, this will return an error with the `value` passed.
    pub fn push(&mut self, value: T) -> Result<usize, T> {
        if self.len >= self.capacity {
            return Err(value);
        }
        let offset = self.len as isize;
        unsafe {
            let ptr = self.ptr.offset(offset);
            ptr.write(value);
        }
        self.len += 1;
        Ok(offset as usize)
    }
}

impl<'a, T: 'a + Sized> Drop for Storage<'a, T> {
    fn drop(&mut self) {
        if const { ::core::mem::needs_drop::<T>() } && self.len > 0 {
            let mut i = (self.len as isize) - 1;
            loop {
                unsafe {
                    let ptr = self.ptr.offset(i);
                    ptr.drop_in_place();
                }
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
        unsafe {
            dealloc(self.ptr.cast(), self.capacity * size_of::<T>());
        }
    }
}
