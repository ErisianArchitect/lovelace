mod refs;

use ::core::{
    ptr::{
        NonNull,
    },
    mem::{
        ManuallyDrop,
    },
    any::{
        TypeId,
    }
};

#[repr(transparent)]
#[derive(Debug)]
pub struct Handle<T: Sized> {
    pub(crate) ptr: NonNull<T>,
}

impl<T: Sized> Handle<T> {
    const LAYOUT: ::core::alloc::Layout = ::core::alloc::Layout::new::<T>();

    pub fn new(value: T) -> Self {
        use ::std::alloc::{alloc, handle_alloc_error, Layout};
        let ptr = unsafe { alloc(Layout::new::<T>()) };
        let Some(ptr) = NonNull::new(ptr.cast::<T>()) else {
            handle_alloc_error(Layout::new::<T>());
        };
        unsafe { ptr.write(value); }
        Self { ptr }
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn transmute_ref<R: Sized>(&self) -> &Handle<R> {
        unsafe { (self as *const Self).cast::<Handle<R>>().as_ref_unchecked() }
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn transmute_mut<R: Sized>(&self) -> &mut Handle<R> {
        unsafe { (self as *const Self).cast::<Handle<R>>().cast_mut().as_mut_unchecked() }
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn transmute<R: Sized>(self) -> Handle<R> {
        unsafe { ::core::mem::transmute(self) }
    }
}

impl<T: Sized> Drop for Handle<T> {
    fn drop(&mut self) {
        unsafe {
            self.ptr.drop_in_place();
            ::std::alloc::dealloc(self.ptr.as_ptr().cast(), Self::LAYOUT);
        }
    }
}

pub struct ComponentHandler<T> {
    is_type: fn(TypeId) -> bool,
    on_drop: fn(&mut Component<T>),
    get_ref: fn(&Component<T>) -> &T,
}

#[repr(C)]
pub struct Component<T: Sized> {
    entity: u64,
    on_drop: fn(&mut Component<T>),
    data: ManuallyDrop<T>,
}

fn drop_component<T: Sized>(component: &mut Component<T>) {
    unsafe {
        ManuallyDrop::drop(&mut component.data);
    }
}

impl<T: Sized> Drop for Component<T> {
    fn drop(&mut self) {
        (self.on_drop)(self);
    }
}

impl<T: Sized> Component<T> {
    pub fn new(entity: u64, value: T) -> Self {
        assert_eq!(size_of::<Component<T>>(), size_of::<Component<()>>(), "Generic size mismatch.");
        Self {
            entity,
            on_drop: drop_component::<T>,
            data: ManuallyDrop::new(value),
        }
    }
    
    #[must_use]
    #[inline(always)]
    pub unsafe fn transmute_ref<R: Sized>(&self) -> &Component<R> {
        unsafe { (self as *const Self).cast::<Component<R>>().as_ref_unchecked() }
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn transmute_mut<R: Sized>(&self) -> &mut Component<R> {
        unsafe { (self as *const Self).cast::<Component<R>>().cast_mut().as_mut_unchecked() }
    }

    #[must_use]
    pub fn into_erased_handle(self) -> Handle<Component<()>> {
        unsafe { Handle::new(self).transmute() }
    }
}

struct OnDrop;

impl Drop for OnDrop {
    fn drop(&mut self) {
        println!("Dropping.");
    }
}

pub fn main() {
    let comp = Component::new(3, OnDrop).into_erased_handle();
    println!("About to drop.");
    drop(comp);
    println!("After drop.");
}

/*

struct TypeInfo {
    drop: fn()
}

struct TypeMap {
    map: HashMap<TypeKey, TypeInfo>,
}

#[repr(C)]
struct Component<T: Sized> {
    parent: Entity,
    // If T is Sized, does that mean that a pointer to T is the same size as a pointer to ()?
    on_drop: Option<fn(Handle<T>)>,
    data: T,
}

struct EntityData {
    components: HashMap<TypeKey, Handle<Component<()>>>,
    
}

fn foo(
    entity: Entity,
    mut ecs: EcsBridge<'_>,
    mut world: ResMut<World>,
    _: Ref<Delete>,
) {
    ecs.remove_entity(entity);
}
*/
