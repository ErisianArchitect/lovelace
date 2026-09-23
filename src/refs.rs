
use ::core::{
    ptr::{
        NonNull,
    },
    marker::{
        PhantomData,
    },
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ref<'a, T: 'static + Sized>(NonNull<T>, PhantomData<&'a T>);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefMut<'a, T: 'static + Sized>(NonNull<T>, PhantomData<&'a mut T>);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Res<'a, T: 'static + Sized>(NonNull<T>, PhantomData<&'a T>);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResMut<'a, T: 'static + Sized>(NonNull<T>, PhantomData<&'a mut T>);

impl<'a, T: 'static + Sized> Ref<'a, T> {
    #[must_use]
    #[inline(always)]
    pub fn as_ref(&self) -> &'a T {
        unsafe { self.0.as_ref() }
    }
}

impl<'a, T: 'static + Sized> RefMut<'a, T> {
    #[must_use]
    #[inline(always)]
    pub fn as_ref(&self) -> &'a T {
        unsafe { self.0.as_ref() }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_mut(&mut self) -> &'a mut T {
        unsafe { self.0.as_mut() }
    }
}

impl<'a, T: 'static + Sized> Res<'a, T> {
    #[must_use]
    #[inline(always)]
    pub fn as_ref(&self) -> &'a T {
        unsafe { self.0.as_ref() }
    }
}

impl<'a, T: 'static + Sized> ResMut<'a, T> {
    #[must_use]
    #[inline(always)]
    pub fn as_ref(&self) -> &'a T {
        unsafe { self.0.as_ref() }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_mut(&mut self) -> &'a mut T {
        unsafe { self.0.as_mut() }
    }
}

impl<'a, T: 'static + Sized> std::ops::Deref for Ref<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T: 'static + Sized> std::ops::Deref for RefMut<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T: 'static + Sized> std::ops::DerefMut for RefMut<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<'a, T: 'static + Sized> std::ops::Deref for Res<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T: 'static + Sized> std::ops::Deref for ResMut<'a, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T: 'static + Sized> std::ops::DerefMut for ResMut<'a, T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

struct World;
struct Physics;

impl World {
    pub fn explode(&mut self, radius: f32, center: [f32; 3]) {
        println!("Explosion at ({}, {}, {}) with a radius of {radius}", center[0], center[1], center[2]);
    }
}

struct EcsBridge<'a> {
    ecs: NonNull<crate::Ecs>,
    _phantom: PhantomData<(&'a (),)>,
}

fn foo(
    ecs: &mut crate::Ecs,
    entity: u64,
    mut world: ResMut<World>,
    mut _phys: ResMut<Physics>,
    mut _b: RefMut<u32>,
    _a: Ref<u32>,
) {
    world.explode(3.0, [0.0; 3]);
}
