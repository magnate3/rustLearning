use std::thread;
use std::time;
use std::sync::Arc;
use std::sync::Mutex;
fn work() {
		loop {
          println!(" i am work");
		  thread::sleep(time::Duration::from_secs(1));
		}
}
fn sleep() {
		loop {
          println!(" i am sleep");
		  thread::sleep(time::Duration::from_secs(1));
		}
}
fn main() {
    let handle =thread::spawn( move || {
					work();
		}
    );
    let handle2 =thread::spawn( move || {
					sleep();
		}
    );
	handle.join();
	handle2.join();
}
