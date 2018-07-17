use std::num::Wrapping;

#[allow(dead_code)]
const MSWS_S_VALUE: u64 = 0xb5ad4eceda1ce2a9;

#[allow(dead_code)]
pub struct MSWS {
  state1: Wrapping<u64>,
  state2: Wrapping<u64>
}
impl super::Generator for MSWS {
  #[allow(dead_code)]
  fn new(seed: u64) -> Self {
    Self { state1: Wrapping(seed), state2: Wrapping(0u64) }
  }
  #[allow(dead_code)]
  fn next(&mut self) -> u32 {
    self.state1 *= self.state1;  // x *= x;
    self.state2 += Wrapping(MSWS_S_VALUE);
    self.state1 += self.state2;  // x += (w += s);
    let rsh = self.state1 >> 32;
    let lsh = self.state1 << 32;
    self.state1 = rsh | lsh;            
    self.state1.0 as u32         // return x = (x>>32) | (x<<32);
  }
}