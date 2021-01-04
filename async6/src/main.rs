//例子二
use futures::{ self, executor,join};
use std::thread;
use std::time;
use async_std::task::{sleep};

async fn async_main() {
		loop {
	//sleep(time::Duration::from_secs(5)).await;
	let f1 = sleep(time::Duration::from_secs(5));
	join!(f1);
    println!("Dance!");
		}
}
fn main() {
    executor::block_on(async_main());
    println!("Hello, world!");
}

 
