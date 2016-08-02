#![allow(non_camel_case_types)]
#![feature(start)]

use libc::{c_int, size_t, pam_handle_t};
use pam::PamResult;
use cryptsetup::CryptoMounter;
use pam::PamResult::PAM_SUCCESS;
use singleton::get as creds;
mod pam;
mod singleton;
mod cryptsetup;
mod mount;
mod syslog;
mod config;
mod libc;

#[start]
#[allow(dead_code)]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    // syslog::info("started");
    0
}

fn on_login(pamh: pam_handle_t) -> PamResult {
    match pam::get_password(pamh) {
        Ok(pass) => {
            let u1 = pam::get_user(pamh);
            syslog::info(pamh, &format!("pam_sm_authenticate: user is: {:?}", u1));
            let user = u1.unwrap();
            creds().push((user, pass));
        }
        Err(err) => syslog::err(pamh, &format!("pam_sm_authenticate: unable to get credentials: {}", err)),
    }
    PAM_SUCCESS
}

fn do_mount(pamh: pam_handle_t, user: &str, password: &str) {
    match mount_info_for(pamh, user) {
        Some((container, dev, mountpoint)) => {
            // syslog::info(pamh, &format!("{}: unlocking  {} to {}", user, container, dev));
            let r = CryptoMounter::new(&container, user).and_then(|cm| cm.unlock(password));
            log_if_error(pamh, &r, "unable to unlock");

            // syslog::info(pamh, &format!("{}: mounting {} to {}", user, dev, mountpoint));
            let ctx = mount::Context::new(&dev, &mountpoint);
            let r = ctx.mount();
            log_if_error(pamh, &r, "unable to mount");
        }
        None => (),
    }
}

fn log_if_error<OK, E: ToString>(pamh: pam_handle_t, r: &Result<OK, E>, message: &str) {
    match *r {
        Ok(_) => (),
        Err(ref err) => syslog::err(pamh, &format!("{}: {}", message, err.to_string())),
    }
}

fn on_session_closed(pamh: pam_handle_t, user: &str) -> PamResult {
    match mount_info_for(pamh, user) {
        Some((container, dev, mountpoint)) => {
            let ctx = mount::Context::new(&dev, &mountpoint);
            syslog::info(pamh, &format!("umounting {}", dev));
            let r = ctx.umount();
            log_if_error(pamh, &r, "unable to unmount");

            syslog::info(pamh, &format!("{}: locking {}", user, container));
            let r = CryptoMounter::new(&container, &dev).and_then(|cm| cm.lock());
            log_if_error(pamh, &r, "unable to unlock");
        }
        None => syslog::info(pamh, &format!("no config found for user {}", user)),
    }
    PAM_SUCCESS
}

fn mount_info_for(pamh: pam_handle_t, user: &str) -> Option<(String, String, String)> {
    config::container_for(pamh, user, "/etc/security/pam_mount.conf")
        .map(|container| (container, format!("/dev/mapper/{}", user), format!("/home/{}", user)))
}

// PAM_EXTERN int pam_sm_open_session(pam_handle_t *pamh, int flags, argc, argv);
//
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn pam_sm_open_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
    match pam::get_user(pamh) {
        Ok(user) => {
            syslog::info(pamh, &format!("pam_sm_open_session {}", user));
            let maybe_index = creds().iter().position(|&(ref u, _)| u.eq(&user));
            match maybe_index {
                Some(index) => {
                    let (usr, password) = creds().swap_remove(index);
                    do_mount(pamh, &usr, &password);
                }
                None => syslog::warn(pamh, &format!("weird, nothing found for {}", user)),
            }
        }
        Err(r) => syslog::err(pamh, &format!("pam_sm_open_session: could not get user name: {:?}", r)),
    }
    PAM_SUCCESS as c_int
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn pam_sm_close_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
    on_session_closed(pamh, &pam::get_user(pamh).unwrap()) as c_int
}

// PAM_EXTERN int pam_sm_authenticate(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn pam_sm_authenticate(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
    on_login(pamh) as c_int
}

// PAM_EXTERN int pam_sm_setcred(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn pam_sm_setcred(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
    PAM_SUCCESS as c_int
}
