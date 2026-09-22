
use ::std::{
    any::{
        Any,
    },
};

#[must_use]
#[inline(always)]
pub fn downcast_ref<T: Any>(any: &dyn Any) -> Option<&T> {
    any.downcast_ref()
}

#[must_use]
#[inline(always)]
pub fn downcast_mut<T: Any>(any: &mut dyn Any) -> Option<&mut T> {
    any.downcast_mut()
}
