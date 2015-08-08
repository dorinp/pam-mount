extern crate libc;
#[allow(non_camel_case_types)] 
use std::ptr;
use std::io::{Result, Error};
use self::libc::{c_int, c_char, size_t, uint32_t};
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
}	

#[derive(Debug)]	
#[allow(dead_code)]
pub enum ContainerFormat {
	LOOPAES, LUKS1, PLAIN, TCRYPT
}

#[derive(Debug)]
#[allow(raw_pointer_derive)]
pub struct CryptoMounter {
	cd: *const crypt_device,
	dm_name: String
}

impl CryptoMounter {

	pub fn new(container: &str, container_format: ContainerFormat, dm_name: &str) -> Result<CryptoMounter> {
		let cd: *const crypt_device = ptr::null();

		let r = unsafe {
			crypt_init(&cd, CryptoMounter::to_ptr(container)) as usize
		};

		let cm = CryptoMounter {cd: cd, dm_name: dm_name.to_string()};
		if r == 0 { cm.load(container_format) } else {Err(Error::last_os_error())}
	}

	fn load(self: CryptoMounter, container_format: ContainerFormat) -> Result<CryptoMounter> {
		let r = unsafe {
			crypt_load(self.cd, CryptoMounter::to_ptr(&format!("{:?}", container_format)), ptr::null())
		};

		self.result(r)
	}

	pub fn unlock(self: CryptoMounter, password: &str) -> Result<CryptoMounter> {
		let r =	unsafe {
			crypt_activate_by_passphrase(self.cd, CryptoMounter::to_ptr(&self.dm_name), CRYPT_ANY_SLOT, 
			CryptoMounter::to_ptr(password), password.len() as size_t, 0)
		};

		self.result(r)
	}

	pub fn lock(self: CryptoMounter) -> Result<CryptoMounter>  {
		let r = unsafe {
			crypt_deactivate(self.cd, CryptoMounter::to_ptr(&self.dm_name))
		};
		self.result(r)
	}

	fn result(self: CryptoMounter, r: c_int) -> Result<CryptoMounter> {
		if r == 0 {Ok(self) } else {Err(Error::last_os_error())}	
	}

	fn to_ptr(s: &str) -> *const c_char {
		CString::new(s).unwrap().as_ptr()	
	}
}

impl Drop for CryptoMounter {
	fn drop(&mut self) {
		unsafe {crypt_free(self.cd)}
	}
}

mod tests {
	#[test]
	#[allow(unused_must_use)]
	fn the_test() {
		use cryptsetup::{CryptoMounter, ContainerFormat};
		let cm = CryptoMounter::new("file.bin", ContainerFormat::LUKS1, "home").and_then(|cm|{
			cm.unlock("preved")
		});
		println!("{:?}", cm);
		// cm.unwrap().lock();
	}	

}
