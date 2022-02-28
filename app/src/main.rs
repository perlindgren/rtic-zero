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
fn init(cx: init::Context) -> Shared {
    hprintln!("init {} {}", cx.local.a, cx.local.b).ok();
    *cx.local.a += 1;
    *cx.local.b += 1;

    Shared { c: 1, d: 2 }
}

#[no_mangle]
fn idle(mut cx: idle::Context) -> ! {
    hprintln!("idle local a {}", cx.local.a).ok();
    *cx.local.a += 1;

    hprintln!("idle local a {}", cx.local.a).ok();

    hprintln!(
        "idle shared c {}",
        cx.shared.c.lock(|c| {
            hprintln!("c {}", *c).ok();
            *c += cx.shared.d.lock(|d| {
                hprintln!("d {}", d).ok();
                *d += *c as u8;
                hprintln!("d new {}", d).ok();
                *d
            }) as u64;

            *c
        })
    )
    .ok();

    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
