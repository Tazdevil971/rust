#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unsupported/time.rs"]
pub mod time;

#[path = "../unsupported/common.rs"]
mod common;

pub use common::{cleanup, decode_error_kind, init, is_interrupted, unsupported, unsupported_err};

pub fn abort_internal() -> ! {
    unsafe {
        eva_abi::eva_rt_abort();
    }
    loop {}
}
