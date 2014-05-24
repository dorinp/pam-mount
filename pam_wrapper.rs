
use std::libc::{c_int, size_t, c_char};
use std::ptr;
#[allow(non_camel_case_types)]
pub type pam_handle_t = *uint;
#[allow(non_camel_case_types)]
pub type c_str = *c_char;

#[allow(non_camel_case_types)]
#[allow(dead_code)]

pub static PAM_SERVICE: c_int 	  = 1;	/* The service name */
pub static PAM_USER: c_int        = 2;	/* The user name */
pub static PAM_TTY: c_int         = 3;	/* The tty name */
pub static PAM_RHOST: c_int       = 4;	/* The remote host name */
pub static PAM_CONV: c_int        = 5;	/* The pam_conv structure */
pub static PAM_AUTHTOK: c_int     = 6;	/* The authentication token (password) */
pub static PAM_OLDAUTHTOK: c_int  = 7;	/* The old authentication token */
pub static PAM_RUSER: c_int       = 8;	/* The remote user name */
pub static PAM_USER_PROMPT: c_int = 9;    /* the prompt for getting a username Linux-PAM extensions */
pub static PAM_FAIL_DELAY: c_int  = 10;   /* app supplied function to override failure delays */
pub static PAM_XDISPLAY: c_int    = 11;   /* X display name */
pub static PAM_XAUTHDATA: c_int   = 12;   /* X server authentication data */
pub static PAM_AUTHTOK_TYPE: c_int= 13;   /* The type for pam_get_authtok */

pub static PAM_SUCCESS: c_int 	  = 0;

#[link(name = "pam")]
extern "C" {
	// int pam_get_item(const pam_handle_t *pamh, int item_type, const void **item);
	fn pam_get_item(pamh: pam_handle_t, item_type: c_int, item: *mut *i8) -> c_int;
}	


// PAM_EXTERN int pam_sm_authenticate(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variable)]
pub fn pam_sm_authenticate(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	println!("pam_sm_authenticate: hello from rust!!! {}", argc);
	println!("pam_sm_authenticate: done {}", getPassword(pamh));
	// println!("The program \"{}\" calculates the value {}",  program, accumulator);
	return PAM_SUCCESS;
}

// PAM_EXTERN int pam_sm_setcred(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variable)]
pub fn pam_sm_setcred(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	// println!("pam_sm_setcred: hello from rust!!! {}", argc);
	return PAM_SUCCESS;
}

fn getPassword(pamh: pam_handle_t) -> ~str {
	let mut r: ~str;
	unsafe {
		let mut info: c_str = ptr::null();
		pam_get_item(pamh, PAM_AUTHTOK, &mut info);
		let z = std::c_str::CString::new(info, false);
		r = z.as_str().unwrap_or("").to_owned();
	}
	return r;
}
