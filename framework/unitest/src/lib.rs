#![feature(decl_macro)]

pub mod testing;
#[cfg(test)]
mod tests;

pub mod prelude {
  pub use super::testing::{bind, must, run, test, then, unit};
}
