#![no_std]

use mutex::Mutex as MutexTrait;
use rtic_zero::priority::Priority;

use core::mem::MaybeUninit;
use core::{cell::UnsafeCell, marker::Sync};

use cortex_m::{
    interrupt::{self, CriticalSection, InterruptNumber},
    peripheral::NVIC,
    register::basepri,
};

use cortex_m_semihosting::hprintln;

extern "Rust" {
    fn nvic_prio_bits() -> u8;
}

#[inline]
pub fn logical2hw(logical: u8, nvic_prio_bits: u8) -> u8 {
    ((1 << nvic_prio_bits) - logical) << (8 - nvic_prio_bits)
}

/// Sets the given `interrupt` as unmasked (enabled)
///
/// This is a
/// [`NVIC::unmask`](../cortex_m/peripheral/struct.NVIC.html#method.unmask)
#[inline(always)]
pub unsafe fn unmask<I>(interrupt: I)
where
    I: InterruptNumber,
{
    hprintln!("unmask {}", interrupt.number()).ok();
    NVIC::unmask(interrupt);
}

/// Sets the given `interrupt` priority
///
/// This is a
/// [`NVIC::unmask`](../cortex_m/peripheral/struct.NVIC.html#method.unmask)
#[inline(always)]
pub unsafe fn set_priority<I>(interrupt: I, prio: u8)
where
    I: InterruptNumber,
{
    hprintln!("set_priority {} {}", interrupt.number(), prio).ok();
    let mut peripheral = cortex_m::peripheral::Peripherals::steal();
    peripheral
        .NVIC
        .set_priority(interrupt, logical2hw(prio, nvic_prio_bits()));
}

/// Sets the given `interrupt` as pending
///
/// This is a convenience function around
/// [`NVIC::pend`](../cortex_m/peripheral/struct.NVIC.html#method.pend)
#[inline(always)]
pub fn pend<I>(interrupt: I)
where
    I: InterruptNumber,
{
    NVIC::pend(interrupt)
}

/// Executes the function closure in a global critical section
///
/// TODO:DOC
#[inline(always)]
pub fn interrupt_free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    cortex_m::interrupt::free(f)
}

pub unsafe fn lock<T, R>(
    ptr: *mut T,
    priority: &Priority,
    ceiling: u8,
    nvic_prio_bits: u8,
    f: impl FnOnce(&mut T) -> R,
) -> R {
    let current = priority.get();

    if current < ceiling {
        if ceiling == (1 << nvic_prio_bits) {
            priority.set(u8::max_value());
            let r = interrupt::free(|_| f(&mut *ptr));
            priority.set(current);
            r
        } else {
            priority.set(ceiling);
            basepri::write(logical2hw(ceiling, nvic_prio_bits));
            let r = f(&mut *ptr);
            basepri::write(logical2hw(current, nvic_prio_bits));
            priority.set(current);
            r
        }
    } else {
        f(&mut *ptr)
    }
}

// Resource
// Storage in `static` using UnsafeCell
//
// Safety:
// pub is ok
// - inner cell is private
// - lock requires priority (unsafe to construct)
//
// WHAT ABOUT DROP SEMANTICS????
pub struct Resource<T, const CEIL: u8> {
    cell: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T, const CEIL: u8> Sync for Resource<T, CEIL> {}

// architecture specific lock implementation
impl<T, const CEIL: u8> Resource<T, CEIL> {
    #[inline(always)]
    unsafe fn get_mut(&self) -> &mut T {
        &mut *(*self.cell.get()).as_mut_ptr()
    }
    // internal
    #[inline(always)]
    fn lock<'a, R>(&self, priority: &'a Priority, f: impl FnOnce(&mut T) -> R) -> R {
        hprintln!("Resource CEIL = {}", CEIL).ok();
        let current = priority.get();
        if CEIL > current {
            if CEIL == (1 << unsafe { nvic_prio_bits() }) {
                hprintln!("-- raise system ceiling to {} by interrupt free", CEIL).ok();
                priority.set(CEIL);

                let r = interrupt::free(|_| f(unsafe { self.get_mut() }));

                hprintln!("-- lower system ceiling to {}", current).ok();
                priority.set(current);
                r
            } else {
                priority.set(CEIL);
                unsafe { basepri::write(logical2hw(CEIL, nvic_prio_bits())) };

                hprintln!("-- raise system ceiling to {} by basepri", CEIL).ok();

                let r = f(unsafe { self.get_mut() });

                hprintln!("-- lower system ceiling to {}", current).ok();

                unsafe { basepri::write(logical2hw(current, nvic_prio_bits())) };
                priority.set(current);
                r
            }
        } else {
            hprintln!("-- lock free access to resource").ok();

            f(unsafe { self.get_mut() })
        }
    }

    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            cell: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    #[inline(always)]
    pub unsafe fn write_maybe_uninit(&self, v: T) {
        (*self.cell.get()).write(v);
    }
}

// The public API
// Resource access by Mutex proxy
//
// Safety
// - pub fn lock        requires &mut access (preventing re-locking)
// - pub unsafe fn new  new proxies only by code gen
pub struct ResourceProxy<'a, T, const CEIL: u8> {
    priority: &'a Priority,
    resource: &'a Resource<T, CEIL>,
}

impl<'a, T, const CEIL: u8> ResourceProxy<'a, T, CEIL> {
    #[inline(always)]
    pub fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        hprintln!("Mutex CEIL = {}", CEIL).ok();
        self.resource.lock(self.priority, f)
    }

    #[inline(always)]
    pub const unsafe fn new(resource: &'a Resource<T, CEIL>, priority: &'a Priority) -> Self {
        Self { priority, resource }
    }
}

impl<'a, T, const CEIL: u8> MutexTrait<T> for ResourceProxy<'a, T, CEIL> {
    fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        self.lock(f)
    }
}
