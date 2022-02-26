#![no_std]
pub trait Mutex<T> {
    fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R;
}
