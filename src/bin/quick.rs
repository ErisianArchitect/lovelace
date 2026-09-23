
use ::core::{
    ptr::{
        NonNull,
    },
};

use lovelace::{
    alloc::{
        Alignment,
        aligned_alloc,
        dealloc,
        arena::{
            Arena,
            ABox,
        }
    },
};

fn with_ptr<T: ?Sized, F: FnOnce(&mut T) -> R, R>(mut ptr: NonNull<T>, f: F) -> R {
    (f)(unsafe { ptr.as_mut() })
}

struct Fred {
    foo: u64,
    bar: u32,
    baz: &'static str,
}

impl Drop for Fred {
    fn drop(&mut self) {
        // println!("Drop Foo: {}", self.foo);
        // println!("Drop Bar: {}", self.bar);
        // println!("Drop Baz: {}", self.baz);
    }
}

struct Bob<'a> {
    fred1: ABox<'a, Fred>,
    fred2: ABox<'a, Fred>,
}

impl<'a> Drop for Bob<'a> {
    fn drop(&mut self) {
        // println!("Fred 1: {}", self.fred1.baz);
        // println!("Fred 2: {}", self.fred2.baz);
    }
}

fn add_to_arena<'a>(arena: &'a Arena) -> ABox<'a, Bob<'a>> {
    let fred1 = arena.add(Fred {
        foo: 1,
        bar: 2,
        baz: "Add Fred 1",
    });
    let fred2 = arena.add(Fred {
        foo: 1,
        bar: 3,
        baz: "Add Fred 2",
    });
    arena.add(Bob {
        fred1,
        fred2,
    })
}

pub fn main() {
    {
        let arena = Arena::new(1024*1024*1024);
        let mut bob_count = 0u64;
        let start = std::time::Instant::now();
        for i in 0..1000000 {
            let mut bob = add_to_arena(&arena);
            let fred = arena.add(Fred {
                foo: 1,
                bar: 2,
                baz: "Hmm",
            });
            bob.fred1.baz = "Changed.";
            bob.fred2 = fred;
            bob_count += bob.fred1.foo;
            arena.clear(true);
            arena.clear(true);
        }
        let elapsed = start.elapsed();
        println!("Count: {bob_count}");
        println!("--- Dropping ---");
        println!("Time: {elapsed:.3?}");
        drop(arena);
    }
}
