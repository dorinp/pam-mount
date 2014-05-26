#[allow(non_camel_case_types)] 

mod mount {
	use std::libc::{c_int, c_char};
	// use std::c_str::CString;

	type libmnt_context = *uint;

	#[link(name = "mount")]
	extern "C" {
		fn mnt_new_context() -> libmnt_context;
		fn mnt_context_set_source(ctx: libmnt_context, source: *c_char) -> c_int;
		fn mnt_context_set_target(ctx: libmnt_context, target: *c_char) -> c_int;
		fn mnt_context_mount(ctx: libmnt_context) -> c_int;
		fn mnt_context_umount(ctx: libmnt_context) -> c_int;
		fn mnt_free_context(ctx: libmnt_context);
		//int mnt_context_strerror(struct libmnt_context *cxt, char *buf, size_t bufsiz);
		// fn mnt_context_strerror(ctx: libmnt_context, buf: *c_char, bufsiz: size_t) -> c_int;
	}	

/*	enum MountError {
		5006 MNT_ERR_AMBIFS     libblkid detected more filesystems on the device 
		5005 MNT_ERR_APPLYFLAGS failed to apply MS_PROPAGATION flags 
		5003 MNT_ERR_LOOPDEV	loopdev setup failed, errno set by libc 
		5004 MNT_ERR_MOUNTOPT   failed to parse/use userspace mount options 
		5000 MNT_ERR_NOFSTAB    not found required entry in fstab 
		5001 MNT_ERR_NOFSTYPE   failed to detect filesystem type
		5002 MNT_ERR_NOSOURCE   required mount source undefined 
	}
*/	
	pub struct Context {
		priv ctx: libmnt_context
	}


	impl Context {
		pub fn new(source: &str, target: &str) -> Context {
			let x = Context {ctx: unsafe { mnt_new_context() }};
			unsafe { 

				let r = source.to_c_str().with_ref(|src| {
					mnt_context_set_source(x.ctx, src)
				}) as int;
				// let r = mnt_context_set_source(x.ctx, source.as_ptr());
				assert_eq!(r, 0);

				let r = target.to_c_str().with_ref(|tgt| {
					mnt_context_set_target(x.ctx, tgt)
				});
				assert_eq!(r, 0);
			};
			x
		}

		pub fn mount(&self) {
			let r = unsafe { mnt_context_mount(self.ctx) };
			assert_eq!(r, 0);
		}

		pub fn umount(&self) {
			let r = unsafe { mnt_context_umount(self.ctx) };
			// println!("{}, {}", r, self.error_description(r))
			assert_eq!(r, 0);
		}

/*		pub fn error_description(&self, err: c_int) -> ~str {
			let buf_size = 500;
			let mut buf: Vec<c_char> = Vec::with_capacity(buf_size);

			unsafe { 
				mnt_context_strerror(self.ctx, buf.as_ptr(), buf_size as size_t);
				let z = CString::new(buf.as_ptr(), false);
				z.as_str().unwrap_or("boo").to_owned()
			}
		}
*/	}
	impl Drop for Context {
    	fn drop(&mut self) {
    		unsafe { mnt_free_context(self.ctx) }
		}
	}

}


fn main() {
	
	let ctx = mount::Context::new("/dev/mapper/home", "/mnt");
	ctx.mount();
	ctx.umount();
	// println!("{}", ctx)
	// c::mnt_context_set_source(ctx, "/dev/mapper/home")
}