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

fn on_login(pamh: pam_handle_t) -> Result<PamResult, String> {
    let pass = try!(pam::get_password(pamh));
    let user = try!(pam::get_user(pamh));
    creds().push((user, pass));
    Ok(PAM_SUCCESS)
}

fn on_session_closed(user: &str) -> Result<(), String> {
    let (container, dev, mountpoint) = try!(mount_info_for(user));
    try!(mount::Context::new(&dev, &mountpoint).umount());
    let cm = try!(CryptoMounter::new(&container, &dev));
    cm.lock().map(|_| ())
}

fn on_session_open(pamh: pam_handle_t) -> Result<String, String> {
    fn get_credentials(user: &str) -> Result<String, String> {
        creds()
            .iter()
            .position(|&(ref u, _)| u.eq(user))
            .ok_or(format!("weird, nothing found for {}", user))
            .and_then(|index| Ok(creds().swap_remove(index).1))
    }

    fn do_mount(user: &str, password: &str) -> Result<String, String> {
        let (container, dev, mountpoint) = try!(mount_info_for(user));
        try!(CryptoMounter::new(&container, user)
            .and_then(|cm| cm.unlock(password))
            .map_err(|r| format!("{}: unable to unlock {} - {}", user, dev, r)));

        mount::Context::new(&dev, &mountpoint)
            .mount()
            .map_err(|r| format!("{}: unable to mount {} to {} - {}", user, dev, mountpoint, r))
            .map(|_| "".to_owned())
    }

    let user = try!(pam::get_user(pamh));
    let password = try!(get_credentials(&user));
    do_mount(&user, &password)
}

// PAM_EXTERN int pam_sm_open_session(pam_handle_t *pamh, int flags, argc, argv);
//
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn pam_sm_open_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
    log_errors(pamh, on_session_open(pamh))
}

#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn pam_sm_close_session(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
    log_errors(pamh, on_session_closed(&pam::get_user(pamh).unwrap()))
}

// PAM_EXTERN int pam_sm_authenticate(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn pam_sm_authenticate(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
    log_errors(pamh, on_login(pamh))
}

// PAM_EXTERN int pam_sm_setcred(pam_handle_t *pamh, int flags, int argc, const char **argv);
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn pam_sm_setcred(pamh: pam_handle_t, flags: c_int, argc: size_t, argv: *const u8) -> c_int {
    PAM_SUCCESS as c_int
}

fn mount_info_for(user: &str) -> Result<(String, String, String), String> {
    config::container_for(user, "/etc/security/pam_mount.conf")
        .map(|container| (container, format!("/dev/mapper/{}", user), format!("/home/{}", user)))
}

fn log_errors<T>(pamh: pam_handle_t, result: Result<T, String>) -> c_int {
    match result {
        Ok(_) => (),
        Err(message) => syslog::err(pamh, &message),
    }
    PAM_SUCCESS as c_int
}
