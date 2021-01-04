//例子二
use futures::{ self, executor};
use std::thread;
use std::time;
use async_std::task::{sleep, spawn};

async fn async_main() {
		loop {
	sleep(time::Duration::from_secs(5)).await;
    println!("Dance!");
		}
}
fn main() {
	let f2 = async {
    println!("main over!");
	};
    executor::block_on(f2);
    println!("Hello, world!");
}

 
