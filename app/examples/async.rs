#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use app::*;
use panic_halt as _;

use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

// Backend (optional if you depend on the backend)
pub use rtic_zero_cortex_m as rtic_arch;

use cortex_m_semihosting::{debug, hprintln};

#[no_mangle]
fn init(cx: init::Context) -> Shared {
    hprintln!("init {} {}", cx.local.a, cx.local.b).ok();

    Shared { c: 1, d: 2 }
}

#[no_mangle]
fn idle(mut cx: idle::Context) -> ! {
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

#[no_mangle]
fn t1(mut cx: t1::Context) {}

#[no_mangle]
fn t2(mut cx: t2::Context) {}

//#[task(binds = SysTick, resources = [syst], priority = 2)]
// Exception SysTick
fn systic(mut cx: systic::Context) {
    hprintln!("systic interrupt").ok();
    // cx.resources.syst.lock(|syst| syst.disable_interrupt());
    // crate::app::foo::spawn(); // this should be from a Queue later
}

//=============
// Waker

static WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(waker_clone, waker_wake, waker_wake, waker_drop);

unsafe fn waker_clone(p: *const ()) -> RawWaker {
    RawWaker::new(p, &WAKER_VTABLE)
}

unsafe fn waker_wake(p: *const ()) {
    let f: fn() = mem::transmute(p);
    f();
}

unsafe fn waker_drop(_: *const ()) {
    // nop
}

//============
// Task

enum Task<F: Future + 'static> {
    Idle,
    Running(F),
    Done(F::Output),
}

impl<F: Future + 'static> Task<F> {
    const fn new() -> Self {
        Self::Idle
    }

    fn spawn(&mut self, future: impl FnOnce() -> F) {
        *self = Task::Running(future());
    }

    unsafe fn poll(&mut self, wake: fn()) {
        match self {
            Task::Idle => {}
            Task::Running(future) => {
                let future = Pin::new_unchecked(future);
                let waker_data: *const () = mem::transmute(wake);
                let waker = Waker::from_raw(RawWaker::new(waker_data, &WAKER_VTABLE));
                let mut cx = Context::from_waker(&waker);

                match future.poll(&mut cx) {
                    Poll::Ready(r) => *self = Task::Done(r),
                    Poll::Pending => {}
                };
            }
            Task::Done(_) => {}
        }
    }
}

//=============
// Timer
// Later we want a proper queue

// use heapless;
pub struct Timer {
    pub done: bool,
    // pub waker_task: Option<fn() -> Result<(), ()>>,
}

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.done {
            Poll::Ready(())
        } else {
            hprintln!("timer polled");
            cx.waker().wake_by_ref();
            hprintln!("after wake_by_ref");
            self.done = true;
            Poll::Pending
        }
    }
}

fn timer_delay(syst: &mut cortex_m::peripheral::SYST, t: u32) -> Timer {
    hprintln!("timer_delay {}", t);

    syst.set_reload(t);
    syst.enable_counter();
    syst.enable_interrupt();
    Timer {
        done: false,
        // waker_task: Some(app::foo::spawn), // we should add waker field to async task context i RTIC
    }
}
