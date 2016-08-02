#![allow(non_camel_case_types)]
use libc::{c_int, pam_handle_t, c_str};
use self::Severity::*;
use std::ffi::CString;

#[link(name = "pam")]
extern "C" {
    // void pam_syslog(pam_handle_t *pamh, int priority, const char *fmt, ...);
    fn pam_syslog(pamh: pam_handle_t, priority: c_int, fmt: c_str, ...);
}


#[allow(dead_code)]
#[derive(PartialEq,Debug,Clone)]
pub enum Severity {
    LOG_EMERG,
    LOG_ALERT,
    LOG_CRIT,
    LOG_ERR,
    LOG_WARNING,
    LOG_NOTICE,
    LOG_INFO,
    LOG_DEBUG,
}

#[allow(dead_code)]
#[derive(PartialEq,Debug,Clone)]
pub enum Facility {
    LOG_KERN = 0 << 3,
    LOG_USER = 1 << 3,
    LOG_MAIL = 2 << 3,
    LOG_DAEMON = 3 << 3,
    LOG_AUTH = 4 << 3,
    LOG_SYSLOG = 5 << 3,
    LOG_LPR = 6 << 3,
    LOG_NEWS = 7 << 3,
    LOG_UUCP = 8 << 3,
    LOG_CRON = 9 << 3,
    LOG_AUTHPRIV = 10 << 3,
    LOG_FTP = 11 << 3,
    LOG_LOCAL0 = 16 << 3,
    LOG_LOCAL1 = 17 << 3,
    LOG_LOCAL2 = 18 << 3,
    LOG_LOCAL3 = 19 << 3,
    LOG_LOCAL4 = 20 << 3,
    LOG_LOCAL5 = 21 << 3,
    LOG_LOCAL6 = 22 << 3,
    LOG_LOCAL7 = 23 << 3,
}

// pub fn notice(pamh: pam_handle_t, msg: &str) {
//     log(pamh, msg, LOG_NOTICE);
// }

pub fn err(pamh: pam_handle_t, msg: &str) {
    log(pamh, LOG_ERR, msg);
}

// pub fn warn(pamh: pam_handle_t, msg: &str) {
//     log(pamh, LOG_WARNING, msg);
// }

pub fn info(pamh: pam_handle_t, msg: &str) {
    log(pamh, LOG_INFO, msg);
}

pub fn log(pamh: pam_handle_t, severity: Severity, msg: &str) {
    unsafe { pam_syslog(pamh, severity as c_int, CString::new(msg).unwrap().as_ptr()) }
}
