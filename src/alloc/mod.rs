
pub mod arena;

cfg_select!{
    unix => {
        mod unix_alloc;
        pub use unix_alloc::*;
    }
    windows => {
        compile_error!("Not yet implemented for Windows.");
    }
}

#[must_use]
#[inline(always)]
const fn kib(n: u32) -> usize {
    1024 * n as usize
}

#[must_use]
#[inline(always)]
const fn mib(n: u32) -> usize {
    const MIB: usize = 1024*1024;
    MIB * n as usize
}

#[must_use]
#[inline(always)]
const fn gib(n: u32) -> usize {
    const GIB: usize = 1024*1024*1024;
    GIB * n as usize
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Alignment {
    KiB4 = kib(4),
    KiB8 = kib(8),
    KiB16 = kib(16),
    KiB32 = kib(32),
    KiB64 = kib(64),
    KiB128 = kib(128),
    KiB256 = kib(256),
    KiB512 = kib(512),
    MiB1 = mib(1),
    MiB2 = mib(2),
    MiB4 = mib(4),
    MiB8 = mib(8),
    MiB16 = mib(16),
    MiB32 = mib(32),
    MiB64 = mib(64),
    MiB128 = mib(128),
    MiB256 = mib(256),
    MiB512 = mib(512),
    GiB1 = gib(1),
    GiB2 = gib(2),
    GiB4 = gib(4),
    GiB8 = gib(8),
}

impl Alignment {
    const MIN: usize = Alignment::KiB4 as usize;
    const MAX: usize = Alignment::GiB8 as usize;

    #[must_use]
    pub const fn from_usize(alignment: usize) -> Option<Self> {
        if alignment > Self::MAX
        || alignment < Self::MIN
        || !alignment.is_power_of_two() {
            return None;
        }
        Some(unsafe { ::core::mem::transmute(alignment) })
    }
    #[must_use]
    #[inline(always)]
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}
