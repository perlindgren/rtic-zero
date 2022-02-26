//
#![no_std]
#![no_main]

use app::*;
use lm3s6965 as _;
use panic_halt as _;

use cortex_m_semihosting::{debug, hprintln};

#[no_mangle]
pub fn plepps() {}
