
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
            ArenaCheckout,
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

// impl Drop for Fred {
//     fn drop(&mut self) {
//         if const { PRINT } {
//             println!("Fred: {}", self.baz);
//         }
//     }
// }

struct Bob<'a> {
    fred1: ABox<'a, Fred>,
    fred2: ABox<'a, Fred>,
}

// impl<'a> Drop for Bob<'a> {
//     fn drop(&mut self) {
//         // println!("Fred 1: {}", self.fred1.baz);
//         // println!("Fred 2: {}", self.fred2.baz);
//     }
// }

fn add_to_arena<'a>(arena: &'a ArenaCheckout<'a>) -> ABox<'a, Bob<'a>> {
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

const PRINT: bool = false;

pub fn main() {
    {
        let mut arena = Arena::new(1024*1024*1024);
        let start = std::time::Instant::now();
        // for _ in 0..2 {
        for i in 0..1_000_000 {
        // {
            let alloc = arena.checkout();
            let mut bob = add_to_arena(&alloc);
            let fred = alloc.add(Fred {
                foo: 1,
                bar: 2,
                baz: "Hmm",
            });
            // println!("Setting fred2.");
            bob.fred2 = fred;
            // println!("After assignment.");
            // take_bob(bob);
            // println!("--- Dropping ---");
        }
        let elapsed = start.elapsed();
        println!("Time: {elapsed:.3?}");
        drop(arena);
    }
}
