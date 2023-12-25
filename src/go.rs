use core::fmt;
use std::ffi::{c_char, c_long, CString, CStr};

#[derive(Debug)]
#[repr(C)]
pub struct GoString {
  p: *const c_char,
  n: c_long
}

impl GoString {
  pub fn new(s: &str) -> GoString {
    let cstr = CString::new(s).unwrap();

    GoString {
      p: cstr.into_raw(),
      n: s.len() as c_long
    }
  }
}

impl fmt::Display for GoString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut str = "";

    if !self.p.is_null() {
      let cstr = unsafe { CStr::from_ptr(self.p) };
      str = cstr.to_str().unwrap();
    }

    write!(f, "GoString {{ p: {}, n: {} }}", str, self.n)
  }
}