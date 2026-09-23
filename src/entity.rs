
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity(pub(crate) u64);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityParts {
    pub id: u64,
    pub generation: u64,
}

impl Entity {
    const GENERATION_ID_BITS: u32 = 26;
    const INDEX_BITS: u32 = 64 - Self::GENERATION_ID_BITS;

    const GENERATION_ID_MASK: u64 = u64::MAX << Self::INDEX_BITS;
    const INDEX_MASK: u64 = u64::MAX >> Self::GENERATION_ID_BITS;
    const GENERATION_ID_MAX: u64 = Self::GENERATION_ID_MASK >> Self::INDEX_BITS;
    const INDEX_MAX: u64 = Self::INDEX_MASK;

    const GENERATION_INCR: u64 = 1 << Self::INDEX_BITS;
    const INDEX_INCR: u64 = 1;
    
    #[must_use]
    #[inline(always)]
    pub(crate) const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    #[inline(always)]
    pub(crate) const fn new(index: u64, generation: u64) -> Self {
        debug_assert!(index <= Self::INDEX_MAX && generation <= Self::GENERATION_ID_MAX);
        Self(index | (generation << Self::INDEX_BITS))
    }

    #[must_use]
    #[inline(always)]
    pub const fn split(self) -> EntityParts {
        EntityParts {
            id: self.0 & Self::INDEX_MASK,
            generation: self.0 >> Self::INDEX_BITS,
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn index(self) -> u64 {
        self.0 & Self::INDEX_MASK
    }

    #[must_use]
    #[inline(always)]
    pub const fn generation(self) -> u64 {
        self.0 >> Self::INDEX_BITS
    }

    #[must_use]
    #[inline(always)]
    pub(crate) const fn incr_index(self) -> Self {
        Self(self.0 + Self::INDEX_INCR)
    }

    #[must_use]
    #[inline(always)]
    pub(crate) const fn incr_generation(self) -> Self {
        Self(self.0 + Self::GENERATION_INCR)
    }
}

pub struct EntityData {
    
}
