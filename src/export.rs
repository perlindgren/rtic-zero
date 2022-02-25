use core::cell::{Cell, UnsafeCell};
use core::marker::PhantomData;
use core::marker::Sync;

// Newtype over `Cell` that forbids mutation through a shared reference
pub struct Priority {
    inner: Cell<u8>,
}

impl Priority {
    /// Create a new Priority
    ///
    /// # Safety
    ///
    /// Will overwrite the current Priority
    #[inline(always)]
    pub unsafe fn new(value: u8) -> Self {
        Priority {
            inner: Cell::new(value),
        }
    }

    /// Change the current priority to `value`
    // These two methods are used by `lock` (see below) but can't be used from the RTIC application
    #[inline(always)]
    fn set(&self, value: u8) {
        self.inner.set(value)
    }

    /// Get the current priority
    /// should maybe not be public but for testing its ok
    #[inline(always)]
    pub fn get(&self) -> u8 {
        self.inner.get()
    }
}

// Resource
// Storage in `static` using UnsafeCell
//
// Safety:
// pub is ok
// - inner cell is private
// - lock is private
pub struct Resource<'a, T, const CEIL: u8> {
    cell: UnsafeCell<T>,
    _lifetime: PhantomData<&'a T>,
}

unsafe impl<'a, T, const CEIL: u8> Sync for Resource<'a, T, CEIL> {}

// architecture specific lock implementation
impl<'a, T, const CEIL: u8> Resource<'a, T, CEIL> {
    // internal
    #[inline(always)]
    fn lock<R>(&self, priority: &'a Priority, f: impl FnOnce(&mut T) -> R) -> R {
        println!("Resource CEIL = {}", CEIL);
        let current = priority.get();
        if CEIL > current {
            priority.set(CEIL);
            println!("-- raise system ceiling to {}", CEIL);
            let r = f(unsafe { &mut *self.cell.get() });
            println!("-- lower system ceiling to {}", current);
            priority.set(current);
            r
        } else {
            println!("-- lock free access to resource");
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
pub struct Mutex<'a, T, const CEIL: u8> {
    priority: &'a Priority,
    resource: &'a Resource<'a, T, CEIL>,
}

impl<'a, T, const CEIL: u8> Mutex<'a, T, CEIL> {
    #[inline(always)]
    pub fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        println!("Mutex CEIL = {}", CEIL);
        self.resource.lock(self.priority, f)
    }

    #[inline(always)]
    pub const unsafe fn new(resource: &'a Resource<'a, T, CEIL>, priority: &'a Priority) -> Self {
        Self { priority, resource }
    }
}
