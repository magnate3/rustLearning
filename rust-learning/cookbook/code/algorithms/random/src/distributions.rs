//By default, random numbers have uniform distribution. To generate numbers with other distributions you instantiate a distribution, then sample from that distribution using Distribution::sample with help of a random-number generator rand::Rng.

//The distributions available are documented here. An example using the Normal distribution is shown below.
extern crate rand;

use rand::distributions::{Normal, Distribution};

fn main() {
  let mut rng = rand::thread_rng();
  let normal = Normal::new(2.0, 3.0);
  let v = normal.sample(&mut rng);
  println!("{} is from a N(2, 9) distribution", v)
}

