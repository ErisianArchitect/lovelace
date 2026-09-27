
use ::core::{
    ptr::{
        NonNull,
    },
    marker::{
        PhantomData,
    },
    alloc::{
        Layout,
    },
    num::{
        NonZeroUsize,
    },
    sync::{
        atomic::{
            AtomicU32,
            Ordering as AtomicOrdering,
        }
    },
    cell::{
        UnsafeCell,
    }
};

use crate::{
    alloc::{Alignment, aligned_alloc, dealloc, dont_need},
};



#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ABox<'arena, T: 'arena + Sized> {
    ptr: NonNull<T>,
    _phantom: PhantomData<(&'arena T,)>,
}

impl<'a, T: 'a + Sized> ABox<'a, T> {
    #[must_use]
    #[inline(always)]
    pub(crate) fn new(ptr: NonNull<T>) -> Self {
        Self {
            ptr,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ptr(&self) -> NonNull<T> {
        self.ptr
    }

    #[must_use]
    #[inline(always)]
    pub fn as_ref(&self) -> &'a T {
        unsafe { self.ptr.as_ref() }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_mut(&mut self) -> &'a mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<'a, T: 'a + Sized> std::ops::Deref for ABox<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T: 'a + Sized> std::ops::DerefMut for ABox<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<'a, T: 'a + Sized> Drop for ABox<'a, T> {
    fn drop(&mut self) {
        if const { ::core::mem::needs_drop::<T>() } {
            unsafe { self.ptr.drop_in_place(); }
        }
    }
}

pub struct Arena {
    arena: NonNull<()>,
    alloc_size: usize,
    arena_top: UnsafeCell<NonNull<()>>,
}

pub struct ArenaCheckout<'a> {
    arena: &'a Arena,
}

impl Arena {
    pub fn new(alloc_size: usize) -> Self {
        let arena = unsafe { aligned_alloc(Alignment::KiB4, alloc_size) };
        Self {
            arena,
            alloc_size,
            arena_top: UnsafeCell::new(arena),
        }
    }

    fn add<'a, T: 'a + Sized>(&'a self, value: T) -> ABox<'a, T> {
        let arena_top = self.get_top();
        let offset = arena_top.addr().get();
        let aligned_offset = offset.next_multiple_of(align_of::<T>());
        let end_offset = aligned_offset + size_of::<T>();
        if end_offset > (self.arena.addr().get() + self.alloc_size) {
            panic!("Arena got too big :(");
        }
        let aligned_ptr = arena_top.with_addr(unsafe { NonZeroUsize::new_unchecked(aligned_offset) }).cast::<T>();
        self.set_top(arena_top.with_addr(unsafe { NonZeroUsize::new_unchecked(end_offset) }));
        unsafe {
            aligned_ptr.write(value);
        }
        ABox::new(aligned_ptr)
    }

    #[must_use]
    #[inline(always)]
    fn get_top(&self) -> NonNull<()> {
        unsafe { self.arena_top.get().read() }
    }

    #[inline(always)]
    fn set_top(&self, value: NonNull<()>) {
        unsafe { self.arena_top.get().write(value) }
    }

    fn clear(&self) {
        self.set_top(self.arena);
    }

    #[must_use]
    #[inline(always)]
    pub fn checkout<'a: 'b, 'b>(&'b mut self) -> ArenaCheckout<'b> {
        ArenaCheckout { arena: self }
    }
}

impl<'a> ArenaCheckout<'a> {
    pub fn add<T: 'a + Sized>(&'a self, object: T) -> ABox<'a, T> {
        self.arena.add(object)
    }
}

impl<'a> Drop for ArenaCheckout<'a> {
    fn drop(&mut self) {
        self.arena.clear();
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.arena, self.alloc_size);
        }
    }
}
