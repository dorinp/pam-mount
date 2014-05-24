use std::ptr;
#[allow(non_camel_case_types)] 

mod c {
	use std::libc::{c_int, c_char, size_t, uint32_t};

	pub type crypt_device = uint;
	
	pub static CRYPT_ANY_SLOT: c_int = -1;
	#[link(name = "cryptsetup")]
	extern "C" {
		// int crypt_init 	(struct crypt_device **cd, const char *device)
		pub fn crypt_init(cd: **crypt_device, device: *c_char) -> c_int;
		// int crypt_load(struct crypt_device *cd, const char *requested_type, void *params )
		pub fn crypt_load(cd: *crypt_device, requested_type: *c_char, params: *c_char) -> c_int;
		
		// int crypt_activate_by_passphrase(struct crypt_device *cd,
		// const char *name, int keyslot, const char *passphrase,
		// size_t  	passphrase_size, uint32_t flags )
		pub fn crypt_activate_by_passphrase(cd: *crypt_device, name: *c_char, keyslot: c_int, 
			passphrase: *c_char, passphrase_size: size_t, flags: uint32_t) -> c_int;

		// int crypt_deactivate (struct crypt_device *cd, const char *name )
		pub fn crypt_deactivate(cd: *crypt_device, name: *c_char) -> c_int;

		pub fn crypt_free(cd: *crypt_device);
	}	
}

static LUKS1: &'static str = "LUKS1";


fn main() {
	let cd: *c::crypt_device = ptr::null();

	let r = "file.bin".to_c_str().with_ref(|device|{
		unsafe {c::crypt_init(&cd, device)}
	});
	println!("{}", r);

	let r = LUKS1.to_c_str().with_ref(|requested_type|{
		unsafe {c::crypt_load(cd, requested_type, ptr::null())}
	});
	println!("{}", r);

	
	let password = "preved";
	let r = "home".to_c_str().with_ref(|name| {
		password.to_c_str().with_ref(|passphrase| {
			unsafe {
				c::crypt_activate_by_passphrase(cd, name, c::CRYPT_ANY_SLOT, 
				passphrase, password.len() as std::libc::size_t, 0)
			}
		})
	});
	println!("{}", r);


	let r = "home".to_c_str().with_ref(|name|{
		unsafe {c::crypt_deactivate(cd, name)}
	});
	println!("{}", r);


	unsafe {c::crypt_free(cd)}
	
}