extern crate libc;
use libc::{size_t, malloc, c_void};
use std::mem;
use std::default::Default;

pub struct Singleton<T>;

impl<T: Default> Singleton<T> {
	pub fn instance<'r>() -> &'r mut T {
		static mut v: *mut c_void = 0 as *mut c_void;

		unsafe {
			if v as uint == 0 {
				let x: *mut T = Singleton::new();
				v = x as *mut c_void;
			}		
			mem::transmute(v)
		}
	}

	unsafe fn new() -> *mut T {
	    let value: T = Default::default();

		let ptr = malloc(::std::mem::size_of::<T>() as size_t) as *mut T;
		assert!(!ptr.is_null());
		// `*ptr` is uninitialized, and `*ptr = value` would attempt to destroy it
		// move_val_init moves a value into this memory without
		// attempting to drop the original value.
		mem::replace(&mut *ptr, value);
		ptr
	}
}
