use crate::{mem, ptr};

pub type Key = u32;

#[inline]
pub fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let res = unsafe { eva_abi::eva_rt_tls_key_create(mem::transmute(dtor)) };
    res.into()
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    let Ok(key) = key.try_into() else {
        rtabort!("invalid key passed to set");
    };
    let res = unsafe { eva_abi::eva_rt_tls_set_specific(key, value.cast()) };
    debug_assert!(res.is_ok());
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    let Ok(key) = key.try_into() else {
        rtabort!("invalid key passed to set");
    };
    let res = unsafe { eva_abi::eva_rt_tls_get_specific(key) };
    res.map(|ptr| ptr.as_ptr()).unwrap_or(ptr::null_mut()).cast()
}

#[inline]
pub unsafe fn destroy(key: Key) {
    let Ok(key) = key.try_into() else {
        rtabort!("invalid key passed to set");
    };
    let res = unsafe { eva_abi::eva_rt_tls_key_delete(key) };
    debug_assert!(res.is_ok());
}
