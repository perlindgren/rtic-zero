#![no_std]

use mutex::Mutex as MutexTrait;
use rtic_zero::priority::Priority;

use core::mem::MaybeUninit;
use core::{cell::UnsafeCell, marker::Sync};
use cortex_m_semihosting::hprintln;

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
    // internal
    #[inline(always)]
    fn lock<'a, R>(&self, priority: &'a Priority, f: impl FnOnce(&mut T) -> R) -> R {
        hprintln!("Resource CEIL = {}", CEIL).ok();
        let current = priority.get();
        if CEIL > current {
            priority.set(CEIL);
            hprintln!("-- raise system ceiling to {}", CEIL).ok();

            let r = f(unsafe { &mut *(*self.cell.get()).as_mut_ptr() });

            hprintln!("-- lower system ceiling to {}", current).ok();
            priority.set(current);

            r
        } else {
            hprintln!("-- lock free access to resource").ok();

            f(unsafe { &mut *(*self.cell.get()).as_mut_ptr() })
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
