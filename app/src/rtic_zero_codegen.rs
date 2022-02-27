#[allow(unused_imports)]
use crate::rtic_arch::{Resource, ResourceProxy};
use cortex_m::register::primask::Primask;
use cortex_m_semihosting::debug;
#[allow(unused_imports)]
use mutex::Mutex;
#[allow(unused_imports)]
use rtic_zero::{priority::Priority, racy_cell::RacyCell};

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    init::run();
    let p = Priority::new(0);
    idle::run(&p);
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

pub mod resources {
    use super::*;
    #[allow(non_upper_case_globals)]
    pub static c: Resource<u64, 0> = Resource::new(0);
}

pub mod init {
    use super::*;
    #[allow(non_upper_case_globals)]
    static __rtic_internal_a: RacyCell<u32> = RacyCell::new(32);
    #[allow(non_upper_case_globals)]
    static __rtic_internal_b: RacyCell<u64> = RacyCell::new(64);
    pub struct Local<'a> {
        pub a: &'a mut u32,
        pub b: &'a mut u64,
    }
    impl<'a> Local<'a> {
        pub unsafe fn new() -> Self {
            Self {
                a: &mut *__rtic_internal_a.get_mut(),
                b: &mut *__rtic_internal_b.get_mut(),
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
    use super::*;
    #[allow(non_upper_case_globals)]
    static __rtic_internal_a: RacyCell<u32> = RacyCell::new(32);
    pub struct Local<'a> {
        pub a: &'a mut u32,
    }
    impl<'a> Local<'a> {
        pub unsafe fn new() -> Self {
            Self {
                a: &mut *__rtic_internal_a.get_mut(),
            }
        }
    }

    pub struct Shared<'a> {
        pub c: ResourceProxy<'a, u64, 0>,
    }

    impl<'a> Shared<'a> {
        pub unsafe fn new(p: &'a Priority) -> Self {
            Self {
                c: ResourceProxy::new(&resources::c, p),
            }
        }
    }

    pub struct Context<'a> {
        pub local: Local<'a>,
        pub shared: Shared<'a>,
    }
    pub unsafe fn run<'a>(p: &'a Priority) {
        idle(Context {
            local: Local::new(),
            shared: Shared::new(p),
        });
    }
    extern "Rust" {
        fn idle(cx: Context) -> !;
    }
}
