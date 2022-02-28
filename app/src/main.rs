//
#![no_std]
#![no_main]

use app::*;
use lm3s6965;
use panic_halt as _;

// Backend
use cortex_m_semihosting::{debug, hprintln};
use rtic_arch::pend;
pub use rtic_zero_cortex_m as rtic_arch;

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

    // unsafe {
    //     cortex_m::interrupt::enable();
    // }

    // pend(lm3s6965::Interrupt::GPIOA);
    cortex_m::peripheral::NVIC::pend(lm3s6965::Interrupt::GPIOA);
    unsafe {
        cortex_m::peripheral::NVIC::unmask(lm3s6965::Interrupt::GPIOA);
    };
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

#[no_mangle]
fn t1(mut cx: t1::Context) {
    hprintln!("t1 local b {}", cx.local.b).ok();
    *cx.local.b += 1;

    hprintln!("t1 local b {}", cx.local.b).ok();
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe fn GPIOA() {
    hprintln!("GPIOA").ok();
}
