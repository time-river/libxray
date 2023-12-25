use std::ffi::{CStr, c_char, CString};

pub fn from_c_char(str: *const c_char) -> String {
  let cstr = unsafe { CStr::from_ptr(str) };

  cstr.to_str().unwrap().to_string()
}

pub fn to_c_char(s: &str) -> *const c_char {
  let cstr = CString::new(s).unwrap();

  return cstr.into_raw();
}
