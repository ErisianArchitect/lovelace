
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

#[repr(C)]
#[derive(Clone, Copy)]
struct Dropper {
    ptr: NonNull<()>,
    drop: fn(NonNull<()>),
    count: usize,
}

impl Dropper {
    fn dropper<T: Sized>(ptr: NonNull<()>) {
        let ptr = ptr.cast::<T>();
        unsafe { ptr.drop_in_place(); }
    }
    
    pub fn new<T: Sized>(ptr: NonNull<T>) -> Self {
        Self {
            ptr: ptr.cast(),
            drop: Dropper::dropper::<T>,
        }
    }
}



pub struct Arena {
    arena: NonNull<()>,
    drop_list: NonNull<Dropper>,
    alloc_size: usize,
    arena_top: UnsafeCell<NonNull<()>>,
    drop_list_len: UnsafeCell<usize>,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ABox<'arena, T: 'arena + Sized> {
    ptr: NonNull<T>,
    _phantom: PhantomData<(&'arena T,)>,
}

impl<'a, T: 'a + Sized> ABox<'a, T> {
    #[must_use]
    #[inline(always)]
    fn new(ptr: NonNull<T>) -> Self {
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ABoxImmut<'arena, T: 'arena + Sized> {
    ptr: NonNull<T>,
    _phantom: PhantomData<(&'arena T,)>,
}

impl<'a, T: 'a + Sized> ABoxImmut<'a, T> {
    fn new(ptr: NonNull<T>) -> Self {
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
}

impl<'a, T: 'a + Sized> std::ops::Deref for ABoxImmut<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Arena {
    pub fn new(alloc_size: usize) -> Self {
        let arena = unsafe { aligned_alloc(Alignment::KiB4, alloc_size) };
        Self {
            arena,
            drop_list: unsafe { aligned_alloc(Alignment::KiB4, alloc_size).cast() },
            alloc_size,
            arena_top: UnsafeCell::new(arena),
            drop_list_len: UnsafeCell::new(0),
        }
    }

    fn add_dropper<'a, T: 'a + Sized>(&'a self, ptr: NonNull<T>) {
        unsafe {
            let add_offset = self.drop_list.offset(self.get_drop_list_len() as isize);
            add_offset.write(Dropper::new(ptr));
            self.set_drop_list_len(self.get_drop_list_len() + 1);
        }
    }

    pub fn add<'a, 'b: 'a, T: 'b + Sized>(&'a self, value: T) -> ABox<'b, T> {
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
        if const { ::core::mem::needs_drop::<T>() } {
            self.add_dropper(aligned_ptr);
        }
        ABox::new(aligned_ptr)
    }

    #[must_use]
    #[inline(always)]
    fn get_drop_list_len(&self) -> usize {
        unsafe { self.drop_list_len.get().read() }
    }

    #[inline(always)]
    fn set_drop_list_len(&self, value: usize) {
        unsafe { self.drop_list_len.get().write(value); }
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

    fn clear_noreset(&self) {
        let drop_list_len = self.get_drop_list_len();
        if drop_list_len == 0 {
            return;
        }
        let mut i = (drop_list_len - 1) as isize;
        loop {
            let mut ptr = unsafe { self.drop_list.offset(i) };
            let freer = unsafe { ptr.as_mut() };
            (freer.drop)(freer.ptr);
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    pub fn clear(&self, retain: bool) {
        self.clear_noreset();
        self.set_drop_list_len(0);
        let top = self.get_top();
        let begin = self.arena;
        self.set_top(self.arena);
        if !retain {
            let free_size = top.addr().get() - begin.addr().get();
            unsafe { dont_need(self.arena, free_size); }
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        self.clear_noreset();
        unsafe {
            dealloc(self.arena, self.alloc_size);
            dealloc(self.drop_list.cast(), self.alloc_size);
        }
    }
}
