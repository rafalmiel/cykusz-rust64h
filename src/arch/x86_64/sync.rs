use core::ops::{Deref, DerefMut};
use kernel::sched::{task_locked, task_unlocked};
use arch::int::{disable_interrupts, enable_interrupts};
use spin::{Mutex as M, MutexGuard as MG};

pub struct Mutex<T> {
    l: M<T>,
}

pub struct MutexGuard<'a, T: ?Sized + 'a> {
    g: Option<MG<'a, T>>,
    irq: bool,
    task_locked: bool
}

impl<T> Mutex<T> {

    pub const fn new(user_data: T) -> Mutex<T> {
        Mutex {
            l: M::new(user_data),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        task_locked();
        MutexGuard {
            g: Some(self.l.lock()),
            irq: false,
            task_locked: true
        }
    }

    pub fn lock_irq(&self) -> MutexGuard<T> {
        disable_interrupts();
        task_locked();
        MutexGuard {
            g: Some(self.l.lock()),
            irq: true,
            task_locked: true
        }
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        self.g.as_ref().unwrap()
    }
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        self.g.as_mut().unwrap()
    }
}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        drop(self.g.take());
        if self.task_locked {
            task_unlocked();
        }
        if self.irq {
            enable_interrupts();
        }
    }
}