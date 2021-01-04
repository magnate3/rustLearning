use std::thread;
use std::time;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
	let data =Arc::new(Mutex::new(vec![1,2,3,4,5]));
	let cp =  Arc::clone(&data);
    let handle =thread::spawn( move || {
			   let mut elems = cp.lock().unwrap();
			   (*elems)[2] = 100;
               println!("threadid   {:?} and data.len {}", thread::current().id(), cp.lock().unwrap().len());
               println!("threadid   {:?} and data.len {:?}", thread::current().id(), cp.lock().unwrap());
		}
    );
    println!("threadid   {:?} and data.len {}", thread::current().id(), data.lock().unwrap().len());
    println!("threadid   {:?} and data.len {:?}", thread::current().id(), data.lock().unwrap());
	let val = Arc::strong_count(&data);
    println!(" val is {}", val);
	handle.join();
}
