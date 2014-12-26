extern crate libc;
#[allow(non_camel_case_types)] 

use std::ptr;
use std::result::Result;
use libc::{c_int, c_char, size_t, uint32_t};

#[allow(non_camel_case_types)]
type crypt_device = uint;
#[allow(non_camel_case_types)]
type p_cd = *const uint;

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




#[deriving(Show)]	
#[allow(dead_code)]
pub enum ContainerFormat {
	LOOPAES, LUKS1, PLAIN, TCRYPT
}

#[deriving(Show)]
#[allow(raw_pointer_deriving)]
pub struct CryptoMounter {
	cd: *const crypt_device,
	dm_name: String
}

impl CryptoMounter {

	pub fn new(container: &str, container_format: ContainerFormat, dm_name: &str) -> Result<Box<CryptoMounter>, int> {
		let cd: *const crypt_device = ptr::null();

		let r = unsafe {
			crypt_init(&cd, container.to_c_str().as_ptr())
		};

		let cm = box CryptoMounter {cd: cd, dm_name: dm_name.to_string()};
		if r == 0 { cm.load(container_format) } else {Err(r as int)}
	}

	fn load(self: Box<CryptoMounter>, container_format: ContainerFormat) -> Result<Box<CryptoMounter>, int> {
		let r = unsafe {
			crypt_load(self.cd, container_format.to_string().to_c_str().as_ptr(), ptr::null())
		};

		self.result(r)
	}

	pub fn unlock(self: Box<CryptoMounter>, password: &str) -> Result<Box<CryptoMounter>, int> {
		let r =	unsafe {
			crypt_activate_by_passphrase(self.cd, self.dm_name.to_c_str().as_ptr(), CRYPT_ANY_SLOT, 
			password.to_c_str().as_ptr(), password.len() as size_t, 0)
		};

		self.result(r)
	}

	pub fn lock(self: Box<CryptoMounter>) -> Result<Box<CryptoMounter>, int>  {
		let r = unsafe {
			crypt_deactivate(self.cd, self.dm_name.to_c_str().as_ptr())
		};
		self.result(r)
	}

	fn result(self: Box<CryptoMounter>, r: c_int) -> Result<Box<CryptoMounter>, int> {
		if r == 0 {Ok(self) } else {Err(r as int)}	
	}
}

impl Drop for CryptoMounter {
	fn drop(&mut self) {
		unsafe {crypt_free(self.cd)}
	}
}

#[test]
fn main() {
	let cm = CryptoMounter::new("file.bin", LUKS1, "home").and_then(|cm|{
		cm.unlock("preved")
	});
	println!("{}", cm);

	cm.unwrap().lock();
}	

