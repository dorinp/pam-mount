use std::libc::{c_int, size_t};
#[allow(non_camel_case_types)]
pub type pam_handle_t = *u8;
/*mod pam_wrapper  {
	#[link(name = "pam")]
	extern {
	}	
}
*/
// PAM_EXTERN int pam_sm_authenticate(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variable)]
pub extern "C" fn pam_sm_authenticate(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	println!("pam_sm_authenticate: hello from rust!!! {}", argc);
	// println!("The program \"{}\" calculates the value {}",  program, accumulator);
	return 0
}

// PAM_EXTERN int pam_sm_setcred(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variable)]
pub extern "C" fn pam_sm_setcred(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	println!("pam_sm_setcred: hello from rust!!! {}", argc);
	return 0
}
