
use ::core::{
    any::{
        Any,
    },
};

use crate::{
    Ecs,
    entity::{
        Entity,
    },
};

pub trait Resource: 'static + Sized + Any {
    fn on_add(&mut self, ecs: &mut Ecs, entity: Entity) {}
    fn on_remove(&mut self, ecs: &mut Ecs, entity: Entity) {}
}
