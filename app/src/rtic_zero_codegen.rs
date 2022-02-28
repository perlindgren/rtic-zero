#[allow(unused_imports)]
use crate::rtic_arch::{Resource, ResourceProxy};
#[allow(unused_imports)]
use core::mem::MaybeUninit;
use cortex_m_semihosting::debug;
#[allow(unused_imports)]
use mutex::Mutex;
#[allow(unused_imports)]
use rtic_zero::{priority::Priority, racy_cell::RacyCell};
#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let shared = init::run();
    resources::move_shared(shared);
    let priority = Priority::new(0);
    idle::run(&priority);
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
pub struct Shared {
    pub c: u64,
    pub d: u8,
}
mod resources {
    use super::*;
    #[allow(non_upper_case_globals)]
    pub static c: Resource<u64, 1> = Resource::new();
    #[allow(non_upper_case_globals)]
    pub static d: Resource<u8, 0> = Resource::new();
    pub unsafe fn move_shared(shared: Shared) {
        c.write_maybe_uninit(shared.c);
        d.write_maybe_uninit(shared.d);
    }
}
pub mod init {
    use super::*;
    #[allow(non_upper_case_globals)]
    static a: RacyCell<u32> = RacyCell::new(32);
    #[allow(non_upper_case_globals)]
    static b: RacyCell<u64> = RacyCell::new(64);
    pub struct Local<'a> {
        pub a: &'a mut u32,
        pub b: &'a mut u64,
    }
    impl<'a> Local<'a> {
        pub unsafe fn new() -> Self {
            Self {
                a: &mut *a.get_mut(),
                b: &mut *b.get_mut(),
            }
        }
    }
    pub struct Context<'a> {
        pub local: Local<'a>,
    }
    pub unsafe fn run() -> Shared {
        init(Context {
            local: Local::new(),
        })
    }
    extern "Rust" {
        fn init(cx: Context) -> Shared;
    }
}
pub mod idle {
    use super::*;
    #[allow(non_upper_case_globals)]
    static a: RacyCell<u32> = RacyCell::new(32);
    pub struct Local<'a> {
        pub a: &'a mut u32,
    }
    impl<'a> Local<'a> {
        pub unsafe fn new() -> Self {
            Self {
                a: &mut *a.get_mut(),
            }
        }
    }
    pub struct Shared<'a> {
        pub c: ResourceProxy<'a, u64, 1>,
        pub d: ResourceProxy<'a, u8, 0>,
    }
    impl<'a> Shared<'a> {
        pub unsafe fn new(priority: &'a Priority) -> Self {
            Self {
                c: ResourceProxy::new(&resources::c, priority),
                d: ResourceProxy::new(&resources::d, priority),
            }
        }
    }
    pub struct Context<'a> {
        pub local: Local<'a>,
        pub shared: Shared<'a>,
    }
    pub unsafe fn run<'a>(priority: &'a Priority) {
        idle(Context {
            local: Local::new(),
            shared: Shared::new(priority),
        });
    }
    extern "Rust" {
        fn idle(cx: Context);
    }
}
pub mod t1 {
    use super::*;
    #[allow(non_upper_case_globals)]
    static b: RacyCell<u64> = RacyCell::new(64);
    pub struct Local<'a> {
        pub b: &'a mut u64,
    }
    impl<'a> Local<'a> {
        pub unsafe fn new() -> Self {
            Self {
                b: &mut *b.get_mut(),
            }
        }
    }
    pub struct Shared<'a> {
        pub c: ResourceProxy<'a, u64, 1>,
    }
    impl<'a> Shared<'a> {
        pub unsafe fn new(priority: &'a Priority) -> Self {
            Self {
                c: ResourceProxy::new(&resources::c, priority),
            }
        }
    }
    pub struct Context<'a> {
        pub shared: Shared<'a>,
        pub local: Local<'a>,
    }
    pub fn pend() {}
}
