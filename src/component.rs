
use ::std::{
    any::{
        Any,
    },
    collections::{
        HashMap,
    },
};

use crate::{
    ecs::{Ecs, EcsTypeId},
    entity::{
        Entity,
    },
};

pub trait Component: 'static + Sized + Any {
    fn on_add(&mut self, ecs: &mut Ecs, entity: Entity) {}
    fn on_remove(&mut self, ecs: &mut Ecs, entity: Entity) {}
}

pub struct ComponentMap {
    map: HashMap<EcsTypeId, Box<dyn Component>>,
}
