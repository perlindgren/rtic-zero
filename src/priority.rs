use core::cell::Cell;

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
    pub fn set(&self, value: u8) {
        self.inner.set(value)
    }

    /// Get the current priority
    /// should maybe not be public but for testing its ok
    #[inline(always)]
    pub fn get(&self) -> u8 {
        self.inner.get()
    }
}
