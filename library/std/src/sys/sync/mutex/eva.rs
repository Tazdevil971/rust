pub struct Mutex {
    pub inner: eva_abi::Mutex2,
}

impl Mutex {
    #[inline]
    pub const fn new() -> Self {
        Self { inner: eva_abi::Mutex2::INIT }
    }

    #[inline]
    pub fn lock(&self) {
        unsafe { eva_abi::eva_rt_sync_mutex_lock(&self.inner) }
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        unsafe { eva_abi::eva_rt_sync_mutex_unlock(&self.inner) }
    }

    #[inline]
    pub fn try_lock(&self) -> bool {
        unsafe { eva_abi::eva_rt_sync_mutex_try_lock(&self.inner) }
    }
}
