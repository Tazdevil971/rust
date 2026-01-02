use crate::io;
use crate::num::NonZero;
use crate::thread::ThreadInit;
use crate::time::Duration;

pub struct Thread {
    thread: eva_abi::Thread,
}

pub const DEFAULT_MIN_STACK_SIZE: usize = 4096;

impl Thread {
    pub unsafe fn new(stack: usize, init: Box<ThreadInit>) -> io::Result<Self> {
        let stack = stack.max(DEFAULT_MIN_STACK_SIZE);
        let thread = unsafe {
            eva_abi::eva_rt_spawn(stack, 0, thread_start, c"Rust Thread", Box::into_raw(init) as _)
        };

        extern "C" fn thread_start(user: *mut ()) {
            let init = unsafe { Box::from_raw(user as *mut ThreadInit) };
            let rust_start = init.init();
            rust_start();
        }

        let thread = thread
            .ok_or_else(|| io::const_error!(io::ErrorKind::Other, "Failed to spawn thread"))?;
        Ok(Self { thread })
    }

    pub fn join(self) {
        let res = unsafe { eva_abi::eva_rt_join_unchecked(self.thread) };
        debug_assert!(res.is_ok());
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        // TODO: Detach!
    }
}

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    Ok(NonZero::new(1).unwrap())
}

pub fn yield_now() {
    unsafe {
        eva_abi::eva_rt_yield_now();
    }
}

pub fn sleep(dur: Duration) {
    unsafe {
        eva_abi::eva_rt_sleep_for(dur);
    }
}

pub fn current_os_id() -> Option<u64> {
    let thread = unsafe { eva_abi::eva_rt_current() };
    Some(thread.0.as_ptr() as _)
}
