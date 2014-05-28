#![crate_id = "pam_mount#0.1"]
// #![crate_type = "dylib"]
use std::libc::{c_int, size_t, malloc};
use std::cast::transmute_mut;
use std::cell::RefCell;
use std::rc::{Rc};
use pam::{pam_handle_t, PAM_SUCCESS, PamResult};
mod pam;

// static mut vec: Rc[(~str, ~str)] = @[];

struct Kaboom {
	x: ~[~str]
}

/*#[start]
#[no_mangle]
pub fn start()  {
	// println!("Hello !!!");
}
*/
fn theStorage() -> &'static mut Kaboom {
	
	static mut MY_STATIC: *Kaboom = 0 as *Kaboom;
	// static mut ss: RefCell<[~str]> = x;

	// static k: Rc<Kaboom> = Rc::new(Kaboom{ x: ~[] });
	unsafe { 
		// let mut x = ~[~"hello", ~"world"];
		

		if (MY_STATIC == 0 as *Kaboom) {
			let newval = ~Rc::new(Kaboom {x: ~[]});
			// newval.inc_strong();
			MY_STATIC = newval.deref() as *Kaboom;
		}

		transmute_mut(&*MY_STATIC) 
	}
}

fn main() {
	println!("bazoo");
	theStorage().x.push(~"hello");
}	

fn on_login(pamh: pam_handle_t) -> PamResult {
	match pam::getPassword(pamh) {
		Ok(pass) => {
			println!("pam_sm_authenticate: done {}", pass);
			let r = pam::setData(pamh, "p", pass);
			println!("pam_sm_authenticate: setData {}", r);
		},
		err		 => println!("pam_sm_authenticate: {}", err)
	}
	PAM_SUCCESS
}

/*PAM_EXTERN int pam_sm_open_session(	pamh, flags, argc, argv);
pam_handle_t *pamh;
int flags;
int argc;
const char **argv;
*/
#[no_mangle]
#[allow(unused_variable)]
#[allow(visible_private_types)]
pub fn pam_sm_open_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	println!("pam_sm_open_session: {}", pam::getUser(pamh));
	match pam::getData(pamh, "p") {
		Ok(pass) => println!("pam_sm_open_session: p: {}", pass),
		err		 => println!("pam_sm_authenticate: {}", err)
	}
	PAM_SUCCESS as c_int
}

#[no_mangle]
#[allow(unused_variable)]
#[allow(visible_private_types)]
pub fn pam_sm_close_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	// println!("pam_sm_close_session: {}", pam::getPassword(pamh));
	PAM_SUCCESS as c_int
}

// PAM_EXTERN int pam_sm_authenticate(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variable)]
#[allow(visible_private_types)]
pub fn pam_sm_authenticate(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	on_login(pamh) as c_int
}

// PAM_EXTERN int pam_sm_setcred(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variable)]
#[allow(visible_private_types)]
pub fn pam_sm_setcred(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	// println!("pam_sm_setcred: hello from rust!!! {}", argc);
	PAM_SUCCESS as c_int
}


