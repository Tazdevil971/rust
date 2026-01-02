use crate::sys::sync::Mutex;
use crate::time::Duration;

pub struct Condvar {
    pub inner: eva_abi::Condvar2,
}

impl Condvar {
    #[inline]
    pub const fn new() -> Self {
        Self { inner: eva_abi::Condvar2::INIT }
    }

    #[inline]
    pub fn notify_one(&self) {
        unsafe { eva_abi::eva_rt_sync_condvar_notify_one(&self.inner) }
    }

    #[inline]
    pub fn notify_all(&self) {
        unsafe { eva_abi::eva_rt_sync_condvar_notify_all(&self.inner) }
    }

    #[inline]
    pub unsafe fn wait(&self, mutex: &Mutex) {
        unsafe { eva_abi::eva_rt_sync_condvar_wait(&self.inner, &mutex.inner) }
    }

    #[inline]
    pub unsafe fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        unsafe { eva_abi::eva_rt_sync_condvar_wait_for(&self.inner, &mutex.inner, dur) }
    }
}
