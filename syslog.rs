extern crate libc;

use libc::{c_int, c_char};
use std::ptr::copy_nonoverlapping_memory;
use self::Severity::{LOG_NOTICE, LOG_ERR, LOG_WARNING, LOG_INFO};
use self::Facility::{LOG_DAEMON};

extern "C" {
 	// void openlog(const char *ident, int option, int facility);
 	fn openlog(ident: *const c_char, option: c_int, facility: c_int);
	fn syslog(priority: c_int, format: *const c_char);
	// fn closelog(); 
 }

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[deriving(PartialEq,Show,Copy)]
pub enum Severity {
  LOG_EMERG,
  LOG_ALERT,
  LOG_CRIT,
  LOG_ERR,
  LOG_WARNING,
  LOG_NOTICE,
  LOG_INFO,
  LOG_DEBUG
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[deriving(PartialEq,Show,Copy)]
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
  LOG_LOCAL7 = 23 << 3
}

pub fn open_log(ident: &str, facility: Facility) {
	static mut buf: [i8, ..30] = [
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0
	];

  unsafe { 
    copy_nonoverlapping_memory::<i8>(buf.as_mut_ptr(), ident.to_c_str().as_ptr(), 30-1);
    openlog(buf.as_ptr(), 0, facility as c_int) 
  }
}


pub fn notice(msg: &str) {
	log(msg, LOG_NOTICE);
}

pub fn err(msg: &str) {
	log(msg, LOG_ERR);
}

pub fn warn(msg: &str) {
  log(msg, LOG_WARNING);
}

pub fn info(msg: &str) {
	log(msg, LOG_INFO);
}

pub fn log(msg: &str, severity: Severity) {
  unsafe { syslog(severity as c_int, msg.to_c_str().as_ptr()) }
}

#[allow(dead_code)]
fn main() {
	open_log("yoo", LOG_DAEMON);
	log("preved", LOG_NOTICE);
	notice("brynza");
}