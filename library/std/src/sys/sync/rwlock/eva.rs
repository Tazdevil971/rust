pub struct RwLock {
    pub inner: eva_abi::Mutex2,
}

impl RwLock {
    #[inline]
    pub const fn new() -> Self {
        Self { inner: eva_abi::Mutex2::INIT }
    }

    #[inline]
    pub fn read(&self) {
        unsafe { eva_abi::eva_rt_sync_mutex_lock(&self.inner) }
    }

    #[inline]
    pub fn try_read(&self) -> bool {
        unsafe { eva_abi::eva_rt_sync_mutex_try_lock(&self.inner) }
    }

    #[inline]
    pub fn write(&self) {
        unsafe { eva_abi::eva_rt_sync_mutex_lock(&self.inner) }
    }

    #[inline]
    pub fn try_write(&self) -> bool {
        unsafe { eva_abi::eva_rt_sync_mutex_try_lock(&self.inner) }
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        unsafe { eva_abi::eva_rt_sync_mutex_unlock(&self.inner) }
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        unsafe { eva_abi::eva_rt_sync_mutex_unlock(&self.inner) }
    }

    #[inline]
    pub unsafe fn downgrade(&self) {
        // No-op
    }
}
