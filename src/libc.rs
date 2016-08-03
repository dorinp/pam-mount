#![allow(non_camel_case_types)]
pub type c_int = i32;
pub type size_t = usize;
pub type c_char = i8;
pub type uint32_t = u32;
pub type c_str = *const c_char;
pub type pam_handle_t = *const usize;
