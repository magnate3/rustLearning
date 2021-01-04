use std::thread;
use std::sync::Arc;

fn main() {
	let data =Arc::new(vec![1,2,3,4,5]);
	let cp =  Arc::clone(&data);
	//let cp = data.clone();
    let handle =thread::spawn( move || {
               println!("threadid   {:?} and data.len {}", thread::current().id(), cp.len());
		}
    );
    println!("threadid   {:?} and data.len {}", thread::current().id(), data.len());
	handle.join();
}
