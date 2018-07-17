use std::num::Wrapping;

const PCG_MULTIPLIER: u64 = 6364136223846793005u64;
const PCG_INCREMENT: u64  = 1442695040888963407u64;

#[allow(dead_code)]
pub type PCG = PCG_XSH_RR;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PCG_XSH_RR {
  state: Wrapping<u64>
}

impl super::Generator for PCG_XSH_RR {
  fn new(seed: u64) -> Self {
    let mut new = Self { state: Wrapping(seed) + Wrapping(PCG_INCREMENT) };
    new.next();
    new
  }

  fn next(&mut self) -> u32 {
    let mut x = self.state;
    let count = x >> 59;

    self.state = x * Wrapping(PCG_MULTIPLIER) + Wrapping(PCG_INCREMENT);
    x ^= x >> 18;
    Wrapping((x >> 27).0 as u32).rotate_right(count.0 as u32).0 as u32
  }
}
