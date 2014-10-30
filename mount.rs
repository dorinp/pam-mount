extern crate libc;
#[allow(non_camel_case_types)] 
use libc::{c_int, c_char};
use std::os::last_os_error;

#[repr(C)]
type libmnt_context = *const uint;

#[link(name = "mount")]
#[allow(ctypes)] 
extern "C" {
	fn mnt_new_context() -> libmnt_context;
	fn mnt_context_set_source(ctx: libmnt_context, source: *const c_char) -> c_int;
	fn mnt_context_set_target(ctx: libmnt_context, target: *const c_char) -> c_int;
	fn mnt_context_mount(ctx: libmnt_context) -> c_int;
	fn mnt_context_umount(ctx: libmnt_context) -> c_int;
	fn mnt_free_context(ctx: libmnt_context);
	//int mnt_context_strerror(struct libmnt_context *cxt, char *buf, size_t bufsiz);
	// fn mnt_context_strerror(ctx: libmnt_context, buf: *c_char, bufsiz: size_t) -> c_int;
}	

/*	enum MountError {
	5000 MNT_ERR_NOFSTAB    not found required entry in fstab 
	5001 MNT_ERR_NOFSTYPE   failed to detect filesystem type
	5002 MNT_ERR_NOSOURCE   required mount source undefined 
	5003 MNT_ERR_LOOPDEV	loopdev setup failed, errno set by libc 
	5004 MNT_ERR_MOUNTOPT   failed to parse/use userspace mount options 
	5005 MNT_ERR_APPLYFLAGS failed to apply MS_PROPAGATION flags 
	5006 MNT_ERR_AMBIFS     libblkid detected more filesystems on the device 
}
*/	
pub struct Context {
	ctx: libmnt_context
}

impl Context {
	pub fn new(source: &str, target: &str) -> Context {
		let x = Context {ctx: unsafe { mnt_new_context() }};
		unsafe { 
			let r = mnt_context_set_source(x.ctx, source.to_c_str().as_ptr()) as int;
			assert_eq!(r, 0);

			mnt_context_set_target(x.ctx, target.to_c_str().as_ptr());
			assert_eq!(r, 0);
		};
		x
	}

	pub fn mount(&self) -> Result<int, String> {
		Context::to_result( unsafe { mnt_context_mount(self.ctx) })
	}

	fn to_result(error_code: c_int) -> Result<int, String> {
		match error_code {
			0 => Ok(0),
			_ => Err(error_code.to_string() + ": " + last_os_error())
		}
	}

	pub fn umount(&self) -> Result<int, String> {
		Context::to_result( unsafe { mnt_context_umount(self.ctx) })
	}

/*	fn error_description(&self, err: c_int) -> String {
		let buf_size = 500;
		let mut buf: Vec<c_char> = Vec::with_capacity(buf_size);

		unsafe { 
			mnt_context_strerror(self.ctx, buf.as_ptr(), buf_size as size_t);
			let z = CString::new(buf.as_ptr(), false);
			z.as_str().unwrap_or("boo").to_owned()
		}
	}
*/}
impl Drop for Context {
	fn drop(&mut self) {
		unsafe { mnt_free_context(self.ctx) }
	}
}



#[allow(dead_code)]
fn main() {

	let ctx = Context::new("/dev/mapper/home", "/mnt");
	println!("{}", ctx.mount());
	let ctx = Context::new("/dev/mapper/home", "/mnt");
	println!("{}", ctx.umount());
}