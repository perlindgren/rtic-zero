//
#![no_std]
#![no_main]

use app::*;
use lm3s6965 as _;
use panic_halt as _;

// Backend
pub use rtic_zero_cortex_m as rtic_arch;

use cortex_m_semihosting::{debug, hprintln};

#[no_mangle]
fn init(cx: init::Context) {
    hprintln!("init");
}
