#![no_std]

use mutex::Mutex as MutexTrait;
use rtic_zero::priority::Priority;

use core::{
    cell::UnsafeCell,
    marker::{PhantomData, Sync},
};

use cortex_m_semihosting::hprintln;

// Resource
// Storage in `static` using UnsafeCell
//
// Safety:
// pub is ok
// - inner cell is private
// - lock requires priority (unsafe to construct)
pub struct Resource<'a, T, const CEIL: u8> {
    cell: UnsafeCell<T>,
    _lifetime: PhantomData<&'a T>,
}

unsafe impl<'a, T, const CEIL: u8> Sync for Resource<'a, T, CEIL> {}

// architecture specific lock implementation
impl<'a, T, const CEIL: u8> Resource<'a, T, CEIL> {
    // internal
    #[inline(always)]
    pub fn lock<R>(&self, priority: &'a Priority, f: impl FnOnce(&mut T) -> R) -> R {
        hprintln!("Resource CEIL = {}", CEIL).ok();
        let current = priority.get();
        if CEIL > current {
            priority.set(CEIL);
            hprintln!("-- raise system ceiling to {}", CEIL).ok();
            let r = f(unsafe { &mut *self.cell.get() });
            hprintln!("-- lower system ceiling to {}", current).ok();
            priority.set(current);
            r
        } else {
            hprintln!("-- lock free access to resource").ok();
            f(unsafe { &mut *self.cell.get() })
        }
    }

    #[inline(always)]
    pub const fn new(v: T) -> Self {
        Self {
            cell: UnsafeCell::new(v),
            _lifetime: PhantomData,
        }
    }
}

// The public API
// Resource access by Mutex proxy
//
// Safety
// - pub fn lock        requires &mut access (preventing re-locking)
// - pub unsafe fn new  new proxies only by code gen
pub struct MutexProxy<'a, T, const CEIL: u8> {
    priority: &'a Priority,
    resource: &'a Resource<'a, T, CEIL>,
}

impl<'a, T, const CEIL: u8> MutexProxy<'a, T, CEIL> {
    #[inline(always)]
    pub fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        hprintln!("Mutex CEIL = {}", CEIL).ok();
        self.resource.lock(self.priority, f)
    }

    #[inline(always)]
    pub const unsafe fn new(resource: &'a Resource<'a, T, CEIL>, priority: &'a Priority) -> Self {
        Self { priority, resource }
    }
}

impl<'a, T, const CEIL: u8> MutexTrait<T> for MutexProxy<'a, T, CEIL> {
    fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        self.lock(f)
    }
}
