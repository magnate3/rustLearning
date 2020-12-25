use std::thread;
use std::sync::Arc;
fn main() {
    let v = vec![1, 2, 3];
    let shared_numbers = Arc::new(v);
	let handle = thread::spawn(move || {
								        println!("Here's a vector: {:?}", v);
    });
    drop(v); // oh no! 
    handle.join().unwrap();
}
