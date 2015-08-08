#![crate_name = "pam_mount"]
#![crate_type = "dylib"]
//#![feature(no_std)]
//#![no_std]
extern crate libc;
	
use self::libc::{c_int, size_t};
use pam::{pam_handle_t, PamResult};
use singleton::{Singleton, VectorOfPairs};
use cryptsetup::{CryptoMounter, ContainerFormat};
use pam::PamResult::{PAM_SUCCESS};

mod pam;
mod singleton;
mod cryptsetup;
mod mount;
mod syslog;
mod config;

fn creds<'a>() -> &'a mut VectorOfPairs {
	Singleton::get()
}

fn on_login(pamh: pam_handle_t) -> PamResult {
	match pam::get_password(pamh) {
		Ok(pass) => {
			let u1 = pam::get_user(pamh);
			syslog::info(&format!("pam_sm_authenticate: user is: {:?}", u1));
			let user = u1.unwrap();
			creds().push((user, pass));
		},
		Err(err) => syslog::err(&format!("pam_sm_authenticate: unable to get credentials: {}", err))
	}
	PAM_SUCCESS
}

fn do_mount(user: &str, password: &str) {
	match mount_info_for(user) {
		Some((container, dev, mountpoint)) => {
			syslog::info(&format!("{}: unlocking  {}", user, container));
			let r = CryptoMounter::new(&container, ContainerFormat::LUKS1, user)
			.and_then(|cm|{
				cm.unlock(password)
			});
			log_if_error(r, "unable to unlock");

			syslog::info(&format!("{}: mounting {} to {}", user, dev, mountpoint));
			let ctx = mount::Context::new(&dev, &mountpoint);
			let r = ctx.mount();
			log_if_error(r, "unable to mount");
		},
		None => ()
	}
}

fn log_if_error<OK, E: ToString>(r: Result<OK, E>, message: &str) {
	match r {
		Ok(_) => (),
		Err(err) => syslog::err(&format!("{}: {}", message, err.to_string()))
	}
}

fn on_session_closed(user: &str) {
	match mount_info_for(user) {
		Some((container, dev, mountpoint)) => {
			let ctx = mount::Context::new(&dev, &mountpoint);
			syslog::info(&format!("umounting {}", dev));
			let r = ctx.umount();
			log_if_error(r, "unable to unmount");

			syslog::info(&format!("{}: locking {}", user, container));
			let r = CryptoMounter::new(&container, ContainerFormat::LUKS1, &dev)
			.and_then(|cm|{
				cm.lock()
			});
			log_if_error(r, "unable to unlock");
		},
		None => syslog::info(&format!("no config found for user {}", user))
	}
}

fn mount_info_for(user: &str) -> Option<(String, String, String)> {
	config::container_for(user, "/etc/security/pam_mount.conf").map(|container| {
		(container, format!("/dev/mapper/{}", user), format!("/home/{}", user))
	})
}

/*PAM_EXTERN int pam_sm_open_session(pam_handle_t *pamh, int flags, argc, argv);
*/
#[no_mangle]
#[allow(unused_variables)]
pub fn pam_sm_open_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
	let r = pam::get_user(pamh);
	if r.is_ok() {
		let user = r.unwrap();
		syslog::info(&format!("pam_sm_open_session {}", user));

		let mut index = -1;
		let saved_credentials = creds().iter().find(|& &(ref u, ref p)| { index+=1; u.eq(&user) });

		match saved_credentials {
			Some(&(_, ref password)) => {
				do_mount(&user, &(*password));
				creds().swap_remove(index);
			},
			None => syslog::warn(&format!("weird, nothing found for {}", user))
		}
	} else {
		syslog::err(&format!("pam_sm_open_session: could not get user name: {:?}", r));
	}
	PAM_SUCCESS as c_int
}

#[no_mangle]
#[allow(unused_variables)]
pub fn pam_sm_close_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
	on_session_closed(&pam::get_user(pamh).unwrap());
	PAM_SUCCESS as c_int
}

// PAM_EXTERN int pam_sm_authenticate(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variables)]
pub fn pam_sm_authenticate(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
	on_login(pamh) as c_int
}

// PAM_EXTERN int pam_sm_setcred(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variables)]
pub fn pam_sm_setcred(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
	PAM_SUCCESS as c_int
}
