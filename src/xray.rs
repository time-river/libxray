use std::ffi::{c_char, c_longlong, c_int};
use core::fmt;
use std::ptr::null;

use super::go::GoString;
use super::utils;
use super::errors::Error;

#[link(name = "xray")]
extern "C" {
  fn xrayVersion() -> *const c_char;
  fn xrayStart(conf: *const c_char, dir: *const c_char, maxMem: c_longlong) -> *const c_char;
  fn xrayStop() -> *const c_char;
}

#[derive(Debug)]
pub struct Xray {
  conf: String,
  dir: Option<String>,
  max_mem: i64,
}

impl fmt::Display for Xray {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let dir = match &self.dir {
      Some(dir) => dir,
      None => "",
    };

    write!(f, "Xray {{ conf: {}, dir: {}, max_mem: {} }}",
        self.conf, dir, self.max_mem)
  }
}

pub fn xray_version() -> String {
  let ver = unsafe { xrayVersion() };
  return utils::from_c_char(ver)
}

impl Xray {
  pub fn new(conf: String, dir: Option<String>, max_mem: i64) -> Self {
    Self {
      conf: conf,
      dir: dir,
      max_mem: max_mem
    }
  }

  pub fn start(&self) -> Result<(), Error> {
    let conf = utils::to_c_char(&self.conf);
    let dir = match &self.dir {
      Some(dir) => utils::to_c_char(dir),
      None => null(),
    };

    let estr = unsafe {
      xrayStart(conf, dir, self.max_mem)
    };

    if estr.is_null() {
      return Ok(());
    } else {
      let str = utils::from_c_char(estr);
      return Err(Error::new(&str));
    }
  }

  pub fn stop(&self) -> Result<(), Error> {
    let estr = unsafe { xrayStop() };
  
    if estr.is_null() {
      return Ok(());
    } else {
      let str = utils::from_c_char(estr);
      return Err(Error::new(&str));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_version() {
    let ver = xray_version();
    println!("version: {}", ver);
  }

  #[test]
  fn test_start() {
    let conf = r###"
    {
      "log": {
        "loglevel": "debug"
      },
      "inbounds": [
        {
          "listen": "127.0.0.1",
          "port": 10086,
          "protocol": "socks"
        }
      ],
      "outbounds": [
        {
          "protocol": "freedom"
        }
      ]
    }
    "###;

      let xray = Xray::new(
        conf.to_string(),
        None,
        0,
      );

      xray.start().unwrap();
  }
}
