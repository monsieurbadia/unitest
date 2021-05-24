use serde::Serialize;
use uwa::string::strbuf;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Test {
  pub name: String,
  pub text: String,
  pub pass: bool,
  pub case: Vec<TestCase>,
}

impl Test {
  #[inline]
  pub fn new(name: &str, pass: bool) -> Self {
    Self {
      text: strbuf![name],
      pass: false,
      ..Default::default()
    }
  }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TestCase {
  pub text: String,
  pub only: bool,
  pub pass: bool,
  pub skip: bool,
}

impl TestCase {
  #[inline]
  pub fn new() -> Self {
    Self {
      text: strbuf![],
      only: false,
      pass: false,
      skip: false,
    }
  }
}
