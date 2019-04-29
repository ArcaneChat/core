use libc;

use crate::dc_strbuilder::dc_strbuilder_t;
use crate::types::*;

pub use crate::imap::*;
pub use crate::mime::*;
pub use crate::smtp::*;
pub use libc::{
    atoi, calloc, close, closedir, exit, fclose, fgets, fopen, fread, free, fseek, ftell, fwrite,
    gmtime, gmtime_r, localtime, localtime_r, malloc, memcmp, memcpy, memmove, memset, mkdir, open,
    opendir, printf, read, readdir, realloc, remove, sleep, snprintf, sprintf, sscanf, strcasecmp,
    strcat, strchr, strcmp, strcpy, strcspn, strdup, strlen, strncasecmp, strncmp, strncpy,
    strrchr, strspn, strstr, strtol, system, time, tolower as __tolower, toupper as __toupper,
    usleep, write,
};

extern "C" {
    pub static mut _DefaultRuneLocale: _RuneLocale;

    pub fn clock() -> libc::clock_t;
    pub fn qsort(
        __base: *mut libc::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: Option<
            unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int,
        >,
    );
    pub fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    pub fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    pub fn strftime(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: *const tm,
    ) -> size_t;
    pub fn atol(_: *const libc::c_char) -> libc::c_long;
    pub fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;

    #[cfg(target_os = "macos")]
    pub fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    #[cfg(not(target_os = "macos"))]
    fn __assert(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;

    // -- DC Methods

    pub fn dc_strbuilder_catf(_: *mut dc_strbuilder_t, format: *const libc::c_char, _: ...);
    pub fn dc_mprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;

    // -- Pthread

    pub fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn pthread_join(_: pthread_t, _: *mut *mut libc::c_void) -> libc::c_int;
    pub fn pthread_cond_signal(_: *mut pthread_cond_t) -> libc::c_int;
    pub fn pthread_cond_timedwait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
        _: *const timespec,
    ) -> libc::c_int;
    pub fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_cond_destroy(_: *mut pthread_cond_t) -> libc::c_int;
    pub fn pthread_cond_init(_: *mut pthread_cond_t, _: *const pthread_condattr_t) -> libc::c_int;
    pub fn pthread_cond_wait(_: *mut pthread_cond_t, _: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    pub fn pthread_self() -> pthread_t;
}

#[cfg(not(target_os = "macos"))]
pub unsafe extern "C" fn __assert_rtn(
    a: *const libc::c_char,
    b: *const libc::c_char,
    c: libc::c_int,
    d: *const libc::c_char,
) -> ! {
    __assert(a, b, c, d)
}

#[cfg(not(target_os = "android"))]
pub use libc::atof;

#[cfg(target_os = "android")]
pub unsafe fn atof(nptr: *mut libc::c_char) -> libc::c_double {
    libc::strtod(nptr, std::ptr::null_mut())
}
