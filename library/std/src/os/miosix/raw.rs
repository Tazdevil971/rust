//! Miosix-specific raw type definitions

#![stable(feature = "raw_ext", since = "1.1.0")]
#![rustc_deprecated(
    since = "1.8.0",
    reason = "these type aliases are no longer supported by \
              the standard library, the `libc` crate on \
              crates.io should be used instead for the correct \
              definitions"
)]
#![allow(deprecated)]
#![allow(missing_debug_implementations)]

use crate::os::unix::raw::{gid_t, uid_t};
use crate::os::raw::{c_ushort, c_ulong, c_short, c_long, c_longlong};


#[stable(feature = "raw_ext", since = "1.1.0")]
pub type dev_t = c_short;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type mode_t = c_ushort;


#[stable(feature = "raw_ext", since = "1.1.0")]
pub type pthread_t = c_ulong;

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type blkcnt_t = c_long;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type blksize_t = c_long;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type ino_t = c_ulong;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type nlink_t = c_ushort;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type off_t = c_longlong;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type time_t = i64;

#[repr(C)]
#[derive(Clone)]
#[stable(feature = "raw_ext", since = "1.1.0")]
pub struct stat {
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_dev: dev_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_ino: ino_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_mode: mode_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_nlink: nlink_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_uid: uid_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_gid: gid_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_rdev: dev_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_size: off_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_atime: time_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_spare1: c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_mtime: time_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_spare2: c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_ctime: time_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_spare3: c_long,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_blksize: blksize_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_blocks: blkcnt_t,
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub st_spare4: [c_long; 2usize],
}