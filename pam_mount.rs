#![crate_id = "pam_mount#0.1"]
#![crate_type = "dylib"]
use std::libc::{c_int, size_t};
use pam::{pam_handle_t, PAM_SUCCESS, PamResult};
use singleton::Singleton;
use cryptsetup::{CryptoMounter};
mod pam;
mod singleton;
mod cryptsetup;
mod mount;

type VectorOfPairs = Vec<(~str, ~str)>;

fn creds() -> &mut VectorOfPairs {
	let z: &mut VectorOfPairs = Singleton::instance();
	z
}

fn on_login(pamh: pam_handle_t) -> PamResult {
	match pam::getPassword(pamh) {
		Ok(pass) => {
			// println!("pam_sm_authenticate: done {}", pass);
			let user = pam::getUser(pamh).unwrap();
			creds().push((user, pass));
		},
		err		 => println!("pam_sm_authenticate: {}", err)
	}
	PAM_SUCCESS
}

fn do_mount(user: &str, password: &str) {
	println!("welcome {} {}", user, password);
	let loop_dev = "home";
	let cm = CryptoMounter::new("/home/d/dev/pam-mount/file.bin", cryptsetup::LUKS1, loop_dev)
	.and_then(|cm|{
		cm.unlock(password)
	});

	let dev = "/dev/mapper/" + loop_dev;
	println!("{}", dev);
	let ctx = mount::Context::new(dev, "/mnt");
	// ctx.mount();

}

/*PAM_EXTERN int pam_sm_open_session(pam_handle_t *pamh, int flags, argc, argv);
*/
#[no_mangle]
#[allow(unused_variable)]
#[allow(visible_private_types)]
pub fn pam_sm_open_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	let user = pam::getUser(pamh).unwrap();
	println!("pam_sm_open_session: {}", user);
	let mut index = -1;
	let o = creds().iter().find(|& &(ref u, ref p)| { index+=1; u.eq(&user) });
	println!("{}", o);
	match o {
		Some(tuple@&(_, ref password)) => {
			do_mount(user, *password);
			creds().swap_remove(index);
		},
		None => println!("weird, nothing found")
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


