use crate::rtic_arch::{MutexProxy, Resource};
use cortex_m_semihosting::hprintln;
use mutex::Mutex;
use rtic_zero::racy_cell::RacyCell;
#[no_mangle]
unsafe extern "C" fn main() -> ! {
    hprintln!("main").ok();
    init::run();
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
pub mod init {
    use super::*;
    static __rtic_internal_a: RacyCell<u32> = RacyCell::new(32);
    static __rtic_internal_b: RacyCell<u64> = RacyCell::new(64);
    pub struct Local<'a> {
        a: &'a mut u32,
        b: &'a mut u64,
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
    static __rtic_internal_a: RacyCell<u32> = RacyCell::new(32);
    static __rtic_internal_b: RacyCell<u64> = RacyCell::new(64);
    pub struct Local<'a> {
        a: &'a mut u32,
        b: &'a mut u64,
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
}
