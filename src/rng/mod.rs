mod msws;
mod pcg;
mod xorshift;

pub use self::msws::*;
pub use self::pcg::*;
pub use self::xorshift::*;

pub trait Generator {
  fn new(seed: u64) -> Self;
  fn next(&mut self) -> u32;
}