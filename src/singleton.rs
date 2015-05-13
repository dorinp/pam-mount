extern crate libc;
use std::mem::transmute;
use std::ptr;

pub struct Singleton;
pub type VectorOfPairs = Vec<(String, String)>;

impl Singleton {
	pub fn get<'a>() -> &'a mut VectorOfPairs {
 		static mut _data:*const VectorOfPairs = 0 as *const VectorOfPairs;

		unsafe {
			if _data == ptr::null::<VectorOfPairs>() {
				let vec: VectorOfPairs = Vec::with_capacity(10);
				_data = transmute(Box::new(vec));
			}
			transmute(_data)
		}
	}

/*	pub fn instance<'r>() -> &'r mut T {
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
	*/
}
