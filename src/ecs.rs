
use ::core::{
    any::{
        TypeId,
        Any,
    },
    sync::{
        atomic::{
            AtomicU32,
            Ordering as AtomicOrdering,
        },
    },
    num::{
        NonZeroU32,
    },
};

use ::std::{
    collections::{
        HashMap,
    },
};

trait ComponentList {
}

trait ResourceList {
}

struct SystemQuery<
    C: ComponentList,
    R: ResourceList,
> {
    components: C,
    resources: R,
}

impl SystemQuery

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EcsTypeId(pub(crate) NonZeroU32);

pub struct Ecs {
    entities: Vec<()>,
    type_ids: HashMap<TypeId, EcsTypeId>,
}

impl Ecs {
    pub fn register_type<T: Any>(&mut self) -> EcsTypeId {
        static ID_COUNTER: AtomicU32 = AtomicU32::new(1);
        let entry = self.type_ids.entry(TypeId::of::<T>()).or_insert_with(|| {
            EcsTypeId(unsafe { NonZeroU32::new_unchecked(ID_COUNTER.fetch_add(1, AtomicOrdering::Relaxed)) })
        });
        *entry
    }

    pub fn get_type_id<T: Any>(&self) -> Option<EcsTypeId> {
        self.type_ids.get(&TypeId::of::<T>()).copied()
    }
}
