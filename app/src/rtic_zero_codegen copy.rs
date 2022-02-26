// Generated code DO NOT TOUCH!

// The architecture defined Resource
use crate::rtic_arch::{MutexProxy, Resource};

// The Mutex Trait
use mutex::Mutex;
use rtic_zero::racy_cell::RacyCell;

use cortex_m_semihosting::{debug, hprintln};
#[no_mangle]
unsafe extern "C" fn main() -> ! {
    hprintln!("main").ok();

    init::run();

    idle::run();

    loop {}
}

// auto generated
static R1: Resource<u32, 1> = Resource::new(5);
// // static R2A: Resource<u32, 2> = Resource::new(500);
// // static R2B: Resource<u32, 2> = Resource::new(1000);

pub mod init {
    use rtic_zero::racy_cell::RacyCell;

    static __local_a: RacyCell<u32> = RacyCell::new(0);

    pub struct Local<'a> {
        a: &'a mut u32,
    }

    impl<'a> Local<'a> {
        pub unsafe fn new() -> Self {
            Self {
                a: &mut *__local_a.get_mut(),
            }
        }
    }

    pub struct Context<'a> {
        pub local: Local<'a>,
    }

    pub unsafe fn run() {
        init(Context {
            local: Local::new(),
        });
    }

    extern "Rust" {
        fn init(cx: Context);
    }
}

pub mod idle {
    use rtic_zero::racy_cell::RacyCell;

    static __local_a: RacyCell<u32> = RacyCell::new(0);

    pub struct Local<'a> {
        a: &'a mut u32,
    }

    impl<'a> Local<'a> {
        pub unsafe fn new() -> Self {
            Self {
                a: &mut *__local_a.get_mut(),
            }
        }
    }

    pub struct Context<'a> {
        pub local: Local<'a>,
    }

    pub unsafe fn run() {
        init(Context {
            local: Local::new(),
        });
    }

    extern "Rust" {
        fn init(cx: Context);
    }
}

// fn inc(r: &mut Mutex<u32, 1>) {
//     hprintln!(
//         "inc {}",
//         r.lock(|v| {
//             *v += 1;
//             *v
//         })
//     );
// }

// fn dec(r: &mut Mutex<u32, 1>) {
//     println!(
//         "dec {}",
//         r.lock(|v| {
//             *v -= 1;
//             *v
//         })
//     );
// }

// fn gen<const CEIL: u8>(r: &mut Mutex<u32, CEIL>) {
//     println!(
//         "gen {}",
//         r.lock(|v| {
//             *v -= CEIL as u32;
//             *v
//         })
//     );
// }

// fn two<const CEIL1: u8, const CEIL2: u8>(r1: &mut Mutex<u32, CEIL1>, r2: &mut Mutex<u32, CEIL2>) {
//     println!("two");
//     r1.lock(|v| {
//         println!("v {}", *v);
//         r2.lock(|v| {
//             println!("v {}", *v);
//         })
//     })
// }

// mod some_external_code {
//     use rtic_zero::mutex::Mutex;

//     pub fn gen<const CEIL: u8>(r: &mut Mutex<u32, CEIL>) {
//         println!(
//             "external gen {}",
//             r.lock(|v| {
//                 *v -= CEIL as u32;
//                 *v
//             })
//         );
//     }
// }

// fn f_lock1(l: &mut impl Lock<u32>) {
//     l.lock(|u| *u += 1);
// }

// fn f_lock2(l: &mut impl Lock<u32>) {
//     l.lock(|u| *u += 1);
// }

// fn main() {
//     let priority = unsafe { Priority::new(0) };
//     let mut m1 = unsafe { Mutex::new(&R1, &priority) };
//     let mut m2a = unsafe { Mutex::new(&R2A, &priority) };
//     let mut m2b = unsafe { Mutex::new(&R2B, &priority) };
//     let f_array = [inc, dec, gen, some_external_code::gen];

//     for f in f_array {
//         f(&mut m1);
//     }

//     let f_lock_arr = [f_lock1, f_lock2];

//     f_lock_arr[0](&mut m1);
//     f_lock_arr[1](&mut m2a);

//     two(&mut m1, &mut m2a); // cannot point to the same Mutex proxy
//     println!("priority {}", priority.get());

//     two(&mut m2a, &mut m2b); // cannot point to the same Mutex proxy
//     println!("priority {}", priority.get());
// }
