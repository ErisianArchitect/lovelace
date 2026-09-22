mod util;

pub mod refs;
pub mod component;
pub mod ecs;
pub mod entity;
pub mod resource;

pub use component::{Component};
pub use ecs::{Ecs, EcsTypeId};
pub use entity::{Entity, EntityParts};
pub use resource::{Resource};

/*

fn foo_system(
    mut ecs: EcsBridge<'_>,
    world: ResMut<World, Required<Immediate>>,
    
) {
}

trait SystemFunction {
    type Components: ComponentQuery;
    type Resources: ResourceQuery;

    fn call(mut ecs: EcsBridge<'_>)
}

SystemData {
    // The function that is called for this system.
    system: Box<dyn SystemFunction>,
    queries: Vec<SystemQuery>,
}

Ecs {
    entities: HashMap<EntityId, EntityData>,
    components: HashMap<ComponentId, ComponentData>,
    systems: HashMap<SystemId, SystemData>,
    resources: HashMap<ResourceId, ResourceData>,
}

EcsQuery {
    when: When,
    components: ComponentList,
    resources: ResourceList,
}
*/
