//This example demonstrates using the rayon::any and rayon::all methods, which are parallelized counterparts to std::any and std::all. rayon::any checks in parallel whether any element of the iterator matches the predicate, and returns as soon as one is found. rayon::all checks in parallel whether all elements of the iterator match the predicate, and returns as soon as a non-matching element is found.
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8 ));
    assert!(vec.par_iter().all(|n| *n <= 8 ));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8 ));
    assert!(!vec.par_iter().all(|n| *n <= 8 )); 
}

