//This example will sort in parallel a vector of Strings.

//Allocate a vector of empty Strings. par_iter_mut().for_each populates random values in parallel. Although multiple options exist to sort an enumerable data type, par_sort_unstable is usually faster than stable sorting algorithms.

extern crate rand;
extern crate rayon;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rayon::prelude::*;

fn main() {
  let mut vec = vec![String::new(); 100_000];
  vec.par_iter_mut().for_each(|p| {
    let mut rng = thread_rng();
    *p = (0..5).map(|_| rng.sample(&Alphanumeric)).collect()
  });
  vec.par_sort_unstable();
}

