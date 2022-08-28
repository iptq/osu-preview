use std::fmt::{self, Debug, Display};

use wasm_bindgen::JsValue;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum Error {
  Js(JsValue),
  Other(anyhow::Error),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::Js(js) => js.fmt(f),
      Error::Other(other) => Display::fmt(other, f),
    }
  }
}

impl<I> From<I> for Error
where
  I: Into<JsValue>,
{
  fn from(err: I) -> Self { Error::Js(err.into()) }
}
