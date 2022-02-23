use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::marker::Sync;

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

impl<'a, T, const CEIL: u8> Resource<'a, T, CEIL> {
    // internal
    #[inline(always)]
    fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        println!("Resource CEIL = {}", CEIL);
        f(unsafe { &mut *self.cell.get() })
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
    resource: &'a Resource<'a, T, CEIL>,
}

impl<'a, T, const CEIL: u8> Mutex<'a, T, CEIL> {
    #[inline(always)]
    pub fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        println!("Mutex CEIL = {}", CEIL);
        self.resource.lock(f)
    }

    #[inline(always)]
    pub const unsafe fn new(resource: &'a Resource<'a, T, CEIL>) -> Self {
        Self { resource }
    }
}
