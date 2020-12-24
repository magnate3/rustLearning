extern crate rand;
mod lib1;
use rand::random;
fn main() {
    let correct:u8 = random();
    println!("correct value is {}",correct);
	lib1::greeting::hello();
}
