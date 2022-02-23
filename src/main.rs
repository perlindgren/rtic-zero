// use core::fmt::Debug;

mod RTIC {
    use core::cell::UnsafeCell;
    use core::marker::PhantomData;
    use core::marker::{Send, Sync};

    // needs to be public for the code gen
    // but not to user code
    pub(crate) struct Resource<'a, T, const CEIL: u8> {
        cell: UnsafeCell<T>,
        _lifetime: PhantomData<&'a T>,
    }

    unsafe impl<'a, T, const CEIL: u8> Sync for Resource<'a, T, CEIL> {}

    impl<'a, T, const CEIL: u8> Resource<'a, T, CEIL> {
        // internal
        #[inline(always)]
        fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            println!("Resource CEIL = {}", CEIL);
            f(unsafe { &mut *self.cell.get() })
        }

        #[inline(always)]
        pub const fn new(v: T) -> Self {
            Self {
                cell: UnsafeCell::new(v),
                _lifetime: PhantomData,
            }
        }
    }

    // The public API
    pub struct Mutex<'a, T, const CEIL: u8> {
        resource: &'a Resource<'a, T, CEIL>,
    }

    impl<'a, T, const CEIL: u8> Mutex<'a, T, CEIL> {
        #[inline(always)]
        pub fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
            println!("Mutex CEIL = {}", CEIL);
            self.resource.lock(f)
        }

        #[inline(always)]
        pub(crate) const fn new(resource: &'a Resource<'a, T, CEIL>) -> Self {
            Self { resource }
        }
    }
}

use RTIC::*;

// auto generated
static R: Resource<u32, 1> = Resource::new(5);

fn inc(r: &mut Mutex<u32, 1>) {
    println!(
        "inc {}",
        r.lock(|v| {
            *v += 1;
            *v
        })
    );
}

fn dec(r: &mut Mutex<u32, 1>) {
    println!(
        "dec {}",
        r.lock(|v| {
            *v -= 1;
            *v
        })
    );
}

fn gen<const CEIL: u8>(r: &mut Mutex<u32, CEIL>) {
    println!(
        "gen {}",
        r.lock(|v| {
            *v -= CEIL as u32;
            *v
        })
    );
}

fn two(r1: &mut Mutex<u32, 1>, r2: &mut Mutex<u32, 1>) {
    println!("two");
    r1.lock(|v| {
        println!("v {}", *v);
        r2.lock(|v| {
            println!("v {}", *v);
        })
    })
}

fn main() {
    let mut m = Mutex::new(&R);
    let a = [inc, dec, gen];
    a[0](&mut m);
    a[1](&mut m);
    a[2](&mut m);
    // two(&mut m, &mut m); // cannot point to the same Mutex proxy
}
