use std::fmt;

#[derive(Debug)]
pub struct Error {
  message: String,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl std::error::Error for Error {}

impl Error {
  pub fn new(msg: &str) -> Self {
    Self {
      message: msg.to_string(),
    }
  }
}