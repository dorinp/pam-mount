extern crate libc;
#[allow(non_camel_case_types)] 
use std::ptr;
use std::io::{Result, Error};
use libc::{c_int, c_char, size_t, uint32_t};
use std::ffi::CString;

#[allow(non_camel_case_types)]
type crypt_device = usize;
#[allow(non_camel_case_types)]
type p_cd = *const usize;

static CRYPT_ANY_SLOT: c_int = -1;
#[allow(improper_ctypes)] 
#[link(name = "cryptsetup")]
extern "C" {
	// int crypt_init 	(struct crypt_device **cd, const char *device)
	fn crypt_init(cd: *const p_cd, device: *const c_char) -> c_int;
	// int crypt_load(struct crypt_device *cd, const char *requested_type, void *params )
	fn crypt_load(cd: *const crypt_device, requested_type: *const c_char, params: *const c_char) -> c_int;
	
	// int crypt_activate_by_passphrase(struct crypt_device *cd,
	// const char *name, int keyslot, const char *passphrase,
	// size_t  	passphrase_size, uint32_t flags )
	fn crypt_activate_by_passphrase(cd: *const crypt_device, name: *const c_char, keyslot: c_int, 
		passphrase: *const c_char, passphrase_size: size_t, flags: uint32_t) -> c_int;

	// int crypt_deactivate (struct crypt_device *cd, const char *name )
	fn crypt_deactivate(cd: *const crypt_device, name: *const c_char) -> c_int;

	fn crypt_free(cd: *const crypt_device);
//	fn crypt_set_debug_level (level: c_int);
}	

#[derive(Debug)]	
#[allow(dead_code)]
pub enum ContainerFormat {
	LOOPAES, LUKS1, PLAIN, TCRYPT
}

#[derive(Debug)]
pub struct CryptoMounter {
	cd: *const crypt_device,
	dm_name: String
}

fn to_cstr(s: &str) -> CString {
	CString::new(s).unwrap()	
}

impl CryptoMounter {
	pub fn new(container: &str, dm_name: &str) -> Result<Self> {
		let cd: *const crypt_device = ptr::null();

		let r = unsafe {
			// crypt_set_debug_level(-1);
			crypt_init(&cd, to_cstr(container).as_ptr())
		};

		let cm = CryptoMounter {cd: cd, dm_name: dm_name.to_owned()};
		if r == 0 { cm.load() } else { Err(Error::from_raw_os_error(r)) }
	}

	fn load(self: Self) -> Result<Self> {
		let r = unsafe {
			crypt_load(self.cd, ptr::null(), ptr::null())
		};
		self.result(r)
	}

	pub fn unlock(self: Self, password: &str) -> Result<Self> {
		let r =	unsafe {
			crypt_activate_by_passphrase(self.cd, to_cstr(&self.dm_name[..]).as_ptr(), CRYPT_ANY_SLOT, 
				to_cstr(password).as_ptr(), password.len() as size_t, 0)
		};
		self.result(r)
	}

	pub fn lock(self: Self) -> Result<Self>  {
		let r = unsafe {
			crypt_deactivate(self.cd, to_cstr(&self.dm_name).as_ptr())
		};
		self.result(r)
	}

	fn result(self: Self, r: c_int) -> Result<Self> {
		if r == 0 {Ok(self) } else {Err(Error::from_raw_os_error(-r))}	
	}

}

impl Drop for CryptoMounter {
	fn drop(&mut self) {
		unsafe {crypt_free(self.cd)}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	#[allow(unused_must_use)]
	fn the_test() {
		use cryptsetup::{CryptoMounter, ContainerFormat};
		let cm = CryptoMounter::new("test.bin", ContainerFormat::LUKS1, "test").and_then(|cm|{
			println!("{:?}", cm);	
			cm.unlock("preved")
		});
		println!("{:?}", cm);
		// cm.unwrap().lock();
	}	

}
