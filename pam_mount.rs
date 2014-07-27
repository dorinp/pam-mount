#![crate_id = "pam_mount#0.1"]
#![crate_type = "dylib"]
extern crate libc;
	
use libc::{c_int, size_t};
use pam::{pam_handle_t, PAM_SUCCESS, PamResult};
use singleton::Singleton;
use cryptsetup::{CryptoMounter};
mod pam;
mod singleton;
mod cryptsetup;
mod mount;
mod syslog;

type VectorOfPairs = Vec<(String, String)>;

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
		Err(err) => syslog::err(format!("pam_sm_authenticate: unable to get credentials: {}", err))
	}
	PAM_SUCCESS
}

fn do_mount(user: &str, password: &str) {
	let (container, dev, mountpoint) = mount_info_for(user);

	syslog::info(user + ": unlocking  " + container);
	let r = CryptoMounter::new(container, cryptsetup::LUKS1, user)
	.and_then(|cm|{
		cm.unlock(password)
	});
	log_if_error(r, "unable to unlock");

	syslog::info(user + ": mounting " + dev + " to " + mountpoint);
	let ctx = mount::Context::new(dev, mountpoint);
	let r = ctx.mount();
	log_if_error(r, "unable to mount");
}

fn log_if_error<OK, E: ToStr>(r: Result<OK, E>, message: &str) {
	match r {
		Ok(_) => (),
		Err(err) => syslog::err(message  + ": " + err.to_str())
	}
}

fn on_session_closed(user: &str) {
	let (container, dev, mountpoint) = mount_info_for(user);

	let ctx = mount::Context::new(dev, mountpoint);
	syslog::info("umounting " + dev);
	let r = ctx.umount();
	log_if_error(r, "unable to unmount");

	syslog::info(user + ": locking  " + container);
	let r = CryptoMounter::new(container, cryptsetup::LUKS1, dev)
	.and_then(|cm|{
		cm.lock()
	});
	log_if_error(r, "unable to unlock");

}

fn mount_info_for(user: &str) -> (String, String, String) {
	match user {
		"d" 	=> (box "/home/d.bin", box "/dev/mapper/" + user, box "/home/" + user),
		_	=> (box "/home/macos.vdi", box "/dev/mapper/" + user, box "/home/" + user)
	}
}

/*PAM_EXTERN int pam_sm_open_session(pam_handle_t *pamh, int flags, argc, argv);
*/
#[no_mangle]
#[allow(unused_variable)]
#[allow(visible_private_types)]
pub fn pam_sm_open_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	let user = pam::getUser(pamh).unwrap();
	syslog::info("pam_sm_open_session " + user);

	let mut index = -1;
	let saved_credentials = creds().iter().find(|& &(ref u, ref p)| { index+=1; u.eq(&user) });

	match saved_credentials {
		Some(tuple@&(_, ref password)) => {
			do_mount(user, *password);
			creds().swap_remove(index);
		},
		None => syslog::warn("weird, nothing found for " + user)
	}
	PAM_SUCCESS as c_int
}

#[no_mangle]
#[allow(unused_variable)]
#[allow(visible_private_types)]
pub fn pam_sm_close_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *u8) -> c_int {
	on_session_closed(pam::getUser(pamh).unwrap());
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


