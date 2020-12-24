//Randomly generates a string of given length ASCII characters in the range A-Z, a-z, 0-9, with Alphanumeric sample.
extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_string);
}

