pub struct XorShift {
  state0: u32,
  state1: u32,
  state2: u32,
  state3: u32
}

impl super::Generator for XorShift {
  fn new(seed: u64) -> Self {
    let mut final_seed = seed;
    if seed == 0 { final_seed = 0x12f2b0c0efa9c094; }
    let left = final_seed as u32;
    let right = (final_seed >> 32) as u32;
    let mut new = XorShift { 
      state0: left,
      state1: right,
      state2: left,
      state3: right
    };
    new.next();
    new
  }
  fn next(&mut self) -> u32 {
    let mut t = self.state3;
    t ^= t << 11;
    t ^= t >> 8;
    self.state3 = self.state2;
    self.state2 = self.state1;
    self.state1 = self.state0;
    let s = self.state0;
    t ^= s;
	  t ^= s >> 19;	
    self.state0 = t;
    t
  }
}