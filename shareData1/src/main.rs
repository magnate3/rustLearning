use std::thread;
use std::time;
use std::sync::Arc;

fn main() {
	let data =Arc::new(vec![1,2,3,4,5]);
	let cp =  Arc::clone(&data);
	//let cp = data.clone();
    let handle =thread::spawn( move || {
               println!("threadid   {:?} and data.len {}", thread::current().id(), cp.len());
			   thread::sleep(time::Duration::from_secs(5));
			   let val = Arc::strong_count(&cp);
			    println!(" val is {}", val);
		}
    );
    println!("threadid   {:?} and data.len {}", thread::current().id(), data.len());
	let val = Arc::strong_count(&data);
    println!(" val is {}", val);
	handle.join();
}
