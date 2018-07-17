#![feature(wrapping_int_impl)]

extern crate rand;
extern crate sha3;
extern crate digest;

mod rng;
use rng::*;

use digest::Digest;
use sha3::Sha3_256;

fn main() {
  let seed = 0u64;
  println!("Seed: {}", seed);

  const MAX: u32 = 3;

  let mut gen = MSWS::new(seed);
  for i in 0..MAX {
    println!("MSWS {}: {}", i+1, gen.next());
  }

  let mut gen2 = PCG::new(seed);
  for i in 0..MAX {
    println!("PCG {}: {}", i+1, gen2.next());
  }

  let mut gen3 = XorShift::new(seed);
  for i in 0..MAX {
    println!("XorShift {}: {}", i+1, gen3.next());
  }

  println!("SHA3 Hash of '{}': {:X?}",
           "Hello there my dude!",
           Sha3_256::digest_str("Hello there my dude!")); // outputs 32 bytes
}
