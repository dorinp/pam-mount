use libc::{c_int, c_str};
use std::str;
use std::ptr;
use std::mem;
use self::PamItemType::{PAM_AUTHTOK, PAM_USER};
use self::PamResult::PAM_SUCCESS;
use std::ffi::CStr;

#[allow(non_camel_case_types)]
pub type pam_handle_t = *const usize;


#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(PartialEq,Debug,Clone)]
#[allow(dead_code)]
pub enum PamResult {
    PAM_SUCCESS = 0,
    PAM_OPEN_ERR = 1, // dlopen() failure when dynamically
    // loading a service module
    PAM_SYMBOL_ERR = 2, // Symbol not found
    PAM_SERVICE_ERR = 3, // Error in service module
    PAM_SYSTEM_ERR = 4, // System error
    PAM_BUF_ERR = 5, // Memory buffer error
    PAM_PERM_DENIED = 6, // Permission denied
    PAM_AUTH_ERR = 7, // Authentication failure
    PAM_CRED_INSUFFICIENT = 8, // Can not access authentication data
    // due to insufficient credentials
    PAM_AUTHINFO_UNAVAIL = 9, // Underlying authentication service
    // can not retrieve authentication */
    // information
    PAM_USER_UNKNOWN = 10, // User not known to the underlying
    // authenticaiton module
    PAM_MAXTRIES = 11, // An authentication service has
    // maintained a retry count which has */
    // been reached.  No further retries */
    // should be attempted
    PAM_NEW_AUTHTOK_REQD = 12, // New authentication token required.
    // This is normally returned if the */
    // machine security policies require */
    // that the password should be changed */
    // beccause the password is NULL or it */
    // has aged
    PAM_ACCT_EXPIRED = 13, // User account has expired
    PAM_SESSION_ERR = 14, // Can not make/remove an entry for
    // the specified session
    PAM_CRED_UNAVAIL = 15, // Underlying authentication service
    // can not retrieve user credentials */
    // unavailable
    PAM_CRED_EXPIRED = 16, // User credentials expired
    PAM_CRED_ERR = 17, // Failure setting user credentials
    PAM_NO_MODULE_DATA = 18, // No module specific data is present
    PAM_CONV_ERR = 19, // Conversation error
    PAM_AUTHTOK_ERR = 20, // Authentication token manipulation error
    PAM_AUTHTOK_RECOVERY_ERR = 21, // Authentication information
    // cannot be recovered
    PAM_AUTHTOK_LOCK_BUSY = 22, // Authentication token lock busy
    PAM_AUTHTOK_DISABLE_AGING = 23, // Authentication token aging disabled
    PAM_TRY_AGAIN = 24, // Preliminary check by password service
    PAM_IGNORE = 25, // Ignore underlying account module
    // regardless of whether the control */
    // flag is required, optional, or sufficient
    PAM_ABORT = 26, // Critical error (?module fail now request)
    PAM_AUTHTOK_EXPIRED = 27, // user's authentication token has expired
    PAM_MODULE_UNKNOWN = 28, // module is not known
    PAM_BAD_ITEM = 29, // Bad item passed to pam_*_item()
    PAM_CONV_AGAIN = 30, /* conversation function is event driven
                          * and data is not available yet */
    PAM_INCOMPLETE = 31, /* please call this function again to
                          * complete authentication stack. Before
                          * calling again, verify that conversation
                          * is completed */
}

impl PamResult {
    fn from_int(v: c_int) -> PamResult {
        unsafe { mem::transmute(v) }
    }
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum PamItemType {
    PAM_SERVICE = 1, // The service name
    PAM_USER = 2, // The user name
    PAM_TTY = 3, // The tty name
    PAM_RHOST = 4, // The remote host name
    PAM_CONV = 5, // The pam_conv structure
    PAM_AUTHTOK = 6, // The authentication token (password)
    PAM_OLDAUTHTOK = 7, // The old authentication token
    PAM_RUSER = 8, // The remote user name
    PAM_USER_PROMPT = 9, // the prompt for getting a username Linux-PAM extensions
    PAM_FAIL_DELAY = 10, // app supplied function to override failure delays
    PAM_XDISPLAY = 11, // X display name
    PAM_XAUTHDATA = 12, // X server authentication data
    PAM_AUTHTOK_TYPE = 13, // The type for pam_get_authtok
}

#[link(name = "pam")]
#[allow(improper_ctypes)]
extern "C" {
    // int pam_get_item(const pam_handle_t *pamh, int item_type, const void **item);
    fn pam_get_item(pamh: pam_handle_t, item_type: c_int, item: *mut c_str) -> c_int;

// int pam_set_data(	pamh, module_data_name, data, (*cleanup)(pam_handle_t *pamh, void *data, int error_status));
// fn pam_set_data(pamh: pam_handle_t, module_data_name: c_str, data: c_str, cleanup: *c_int) -> c_int;
// fn pam_get_data(pamh: pam_handle_t, module_data_name: c_str, data: *mut c_str) -> c_int;
}

pub fn get_password(pamh: pam_handle_t) -> Result<String, String> {
    get_item(pamh, PAM_AUTHTOK)
}

pub fn get_user(pamh: pam_handle_t) -> Result<String, String> {
    get_item(pamh, PAM_USER)
}

fn get_item(pamh: pam_handle_t, item_type: PamItemType) -> Result<String, String> {
    let mut info: c_str = ptr::null();
    let r = unsafe { pam_get_item(pamh, item_type as c_int, &mut info) };

    match PamResult::from_int(r) {
        PAM_SUCCESS => ok_if_not_null(info),
        e => Err(format!("{:?}", e)),
    }
}

fn ok_if_not_null(info: c_str) -> Result<String, String> {
    if info == ptr::null() {
        Err("the pointer is null".into())
    } else {
        let z = unsafe { CStr::from_ptr(info) };
        Ok(str::from_utf8(z.to_bytes()).unwrap_or("").into())
    }
}


// pub fn setData(pamh: pam_handle_t, name: &str, data: &str) -> Result<int, String> {
// let r = name.to_c_str().with_ref(|name| {
// data.to_c_str().with_ref(|data| {
// println!("set data: {}", data);
// unsafe { pam_set_data(pamh, name, data, ptr::null()) }
// })
// });
//
// match PamResult::from_int(r) {
// PAM_SUCCESS => Ok(0),
// e 			=> Err(e.to_str())
// }
// }
//
//
// pub fn getData(pamh: pam_handle_t, name: &str) -> Result<String, String> {
// let mut info: c_str = ptr::null();
//
// let r = name.to_c_str().with_ref(|name| {
// unsafe { pam_get_data(pamh, name, &mut info) }
// });
//
// println!("get data: {}", info);
// match PamResult::from_int(r) {
// PAM_SUCCESS => ok_if_not_null(info),
// e 			=> Err(e.to_str())
// }
// }
//
//
