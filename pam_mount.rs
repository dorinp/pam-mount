#![crate_id = "pam_mount#0.1"]
#![crate_type = "dylib"]

use std::libc::{c_int, size_t};
mod pam;

// PAM_EXTERN int pam_sm_authenticate(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variable)]
#[allow(dead_code)]
fn pam_sm_authenticate(pamh: pam::pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	println!("pam_sm_authenticate: hello from rust!!! {}", argc);
	println!("pam_sm_authenticate: done {}", pam::getPassword(pamh));
	// println!("The program \"{}\" calculates the value {}",  program, accumulator);
	pam::PAM_SUCCESS as c_int
}
