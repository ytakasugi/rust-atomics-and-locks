use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

// `Send`(所有権を別スレッドに転送できる型)を実装した型Tに対し、
// 安全でない`Sync`(複数スレッドからのアクセスを許可)を実装する
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> &mut T {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        unsafe { &mut *self.value.get() }
    }
    /// Safety
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

#[test]
fn main() {
}