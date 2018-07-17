pub mod msws;
pub mod pcg;

pub use msws::*;
pub use pcg::*;

pub trait Generator {
  fn new(seed: u64) -> Self;
  fn next(&mut self) -> u32;
}