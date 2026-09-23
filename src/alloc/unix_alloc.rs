
use ::core::{
    ptr::{
        NonNull,
    },
    num::{
        NonZeroUsize,
    },
};

use ::std::{
    sync::{
        LazyLock,
    },
};

use ::rustix::{
    param::{
        page_size,
    },
    mm::{
        mmap,
        mmap_anonymous,
        munmap,
        madvise,
        ProtFlags,
        MapFlags,
        Advice,
    },
};

use super::{
    Alignment,
};

fn page_normalize(alignment: Alignment, size: usize) -> (usize, usize) {
    let page_size = page_size();
    let align = (alignment as usize).next_multiple_of(page_size);
    (align, size.next_multiple_of(page_size))
}

pub unsafe fn aligned_alloc(alignment: Alignment, size: usize) -> NonNull<()> {
    let (align, size) = page_normalize(alignment, size);
    let adj_size = if align == page_size() {
        size
    } else {
        size + align
    };
    let result = unsafe { mmap_anonymous(
        ::core::ptr::null_mut(),
        adj_size,
        ProtFlags::READ | ProtFlags::WRITE,
        MapFlags::PRIVATE,
    ) };
    match result {
        Ok(ptr) => {
            if ptr.is_null() {
                panic!("Out of memory.");
            }
            let aligned_ptr = ptr.with_addr(ptr.addr().next_multiple_of(align));
            let pre_align_size = aligned_ptr.addr() - ptr.addr();
            let post_align_size = adj_size - pre_align_size - size;
            let post_offset = aligned_ptr.addr() + size;
            let post_addr = aligned_ptr.with_addr(post_offset);
            if pre_align_size != 0 {
                let unmap_result = unsafe { munmap(ptr, pre_align_size) };
                if let Err(errno) = unmap_result {
                    panic!("Error: {errno}");
                }
            }
            if post_align_size != 0 {
                let unmap_result = unsafe { munmap(post_addr, post_align_size) };
                if let Err(errno) = unmap_result {
                    panic!("Error: {errno}");
                }
            }
            unsafe { NonNull::new_unchecked(aligned_ptr.cast()) }
        }
        Err(errno) => {
            panic!("Error: {errno}");
        }
    }
}

pub unsafe fn dealloc(ptr: NonNull<()>, size: usize) {
    match unsafe { munmap(ptr.as_ptr().cast(), size) } {
        Ok(_) => {},
        Err(errno) => {
            panic!("Error: {errno}");
        }
    }
}

pub unsafe fn dont_need(ptr: NonNull<()>, size: usize) {
    match unsafe { madvise(ptr.as_ptr().cast(), size, Advice::LinuxFree) } {
        Ok(_) => {},
        Err(errno) => {
            panic!("Error: {errno}");
        }
    }
}
