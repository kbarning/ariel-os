//! This module provides a Mutex-protected [`RefCell`] --- basically a way to ensure
//! at runtime that some reference is used only once.
use core::cell::{Ref, RefCell, RefMut};
use critical_section::{CriticalSection, Mutex, with};

pub(crate) struct EnsureOnce<T> {
    inner: Mutex<RefCell<T>>,
}

impl<T> EnsureOnce<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: Mutex::new(RefCell::new(inner)),
        }
    }

    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Ref<'_, T>) -> R,
    {
        with(|cs| self.with_cs(cs, f))
    }

    pub fn with_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(RefMut<'_, T>) -> R,
    {
        with(|cs| self.with_mut_cs(cs, f))
    }

    pub fn with_cs<F, R>(&self, cs: CriticalSection<'_>, f: F) -> R
    where
        F: FnOnce(Ref<'_, T>) -> R,
    {
        f(self.inner.borrow(cs).borrow())
    }

    pub fn with_mut_cs<F, R>(&self, cs: CriticalSection<'_>, f: F) -> R
    where
        F: FnOnce(RefMut<'_, T>) -> R,
    {
        f(self.inner.borrow(cs).borrow_mut())
    }

    // pub fn borrow_mut<'a>(&'a self, cs: &'a CriticalSection<'_>) -> RefMut<T> {
    //     self.inner.borrow(cs).borrow_mut()
    // }
    // pub fn borrow<'a>(&'a self, cs: &'a CriticalSection<'_>) -> Ref<T> {
    //     self.inner.borrow(cs).borrow()
    // }

    #[allow(dead_code)]
    pub fn as_ptr(&self, cs: CriticalSection<'_>) -> *mut T {
        self.inner.borrow(cs).as_ptr()
    }
}
