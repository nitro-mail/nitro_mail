use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use interprocess::local_socket::tokio::LocalSocketStream;

pub struct InterprocessConnectionGuard {
    pub connection: *mut LocalSocketStream,
    pub has_connection: Arc<AtomicBool>,
}

impl Deref for InterprocessConnectionGuard {
    type Target = LocalSocketStream;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.connection }
    }
}

impl DerefMut for InterprocessConnectionGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.connection }
    }
}
impl Drop for InterprocessConnectionGuard {
    fn drop(&mut self) {
        self.has_connection
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }
}
unsafe impl Send for InterprocessConnectionGuard {}
unsafe impl Sync for InterprocessConnectionGuard {}

pub struct InterprocessConnectionInner {
    pub has_connection: Arc<AtomicBool>,
    pub connection: UnsafeCell<LocalSocketStream>,
}
impl Drop for InterprocessConnectionInner {
    fn drop(&mut self) {
        // TODO: Make sure we have the connection before we close it
    }
}
impl InterprocessConnectionInner {
    pub fn new(connection: LocalSocketStream) -> Self {
        Self {
            has_connection: Arc::new(AtomicBool::new(true)),
            connection: UnsafeCell::new(connection),
        }
    }
}
impl InterprocessConnectionInner {
    pub fn get_guard(&self) -> Option<InterprocessConnectionGuard> {
        if self
            .has_connection
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::Acquire,
            )
            .is_err()
        {
            return None;
        }

        let connection = unsafe { &mut *self.connection.get() };
        Some(InterprocessConnectionGuard {
            connection,
            has_connection: self.has_connection.clone(),
        })
    }
    pub fn get_guard_panic(&self) -> InterprocessConnectionGuard {
        if self
            .has_connection
            .compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::Acquire,
            )
            .is_err()
        {
            panic!("Attempted to get connection guard when one already exists");
        }

        let connection = unsafe { &mut *self.connection.get() };
        InterprocessConnectionGuard {
            connection,
            has_connection: self.has_connection.clone(),
        }
    }
}
unsafe impl Send for InterprocessConnectionInner {}
unsafe impl Sync for InterprocessConnectionInner {}
