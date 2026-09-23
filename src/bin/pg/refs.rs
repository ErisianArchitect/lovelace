
use ::core::{
    ptr::{
        NonNull,
    },
    marker::PhantomData,
};

pub trait Resource {}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResourceAct {
    Skip,
    Load,
    Error,
}

pub trait ResourceAction {
    const ON_RUN: ResourceAct = ResourceAct::Skip;
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Required<const VALUE: bool>;

impl ResourceAction for () {}

impl ResourceAction for Required<true> {
    const ON_RUN: ResourceAct = ResourceAct::Error;
}

/*
fn foo(
    mut ecs: EcsBridge<'_>,
    strings: Res<StringProvider>,
    file_system: Option<ResMut<FileSystem>>,
) {
}
*/

pub struct Res<'a, T, C = ()> {
    pub(crate) ptr: NonNull<T>,
    pub(crate) _phant: PhantomData<(&'a T, fn(C))>,
}

pub struct ResMut<'a, T, C = ()> {
    pub(crate) ptr: NonNull<T>,
    pub(crate) _phant: PhantomData<(&'a mut T, fn(C))>,
}

pub struct Ref<'a, T> {
    pub(crate) ptr: NonNull<T>,
    pub(crate) _phant: PhantomData<(&'a T,)>,
}

pub struct RefMut<'a, T> {
    pub(crate) ptr: NonNull<T>,
    pub(crate) _phant: PhantomData<(&'a mut T,)>,
}

struct EcsBridge<'a> {
    _phant: PhantomData<(fn(&'a ()),)>,
}

struct World;
struct StringProvider;

struct Player;
struct Transform;
struct Camera;

struct Query<Q> {
    _phant: PhantomData<(fn(Q),)>,
}

fn foo_system(
    mut ecs: EcsBridge<'_>,
    entity: u32,
    strings: Res<StringProvider>,
    mut world: ResMut<World>,
    mut player: RefMut<Player>,
    mut transform: RefMut<Transform>,
    mut camera: RefMut<Camera>,
    mut query: Query<(
            RefMut<Player>,
        )>,
) {
    
}
