//例子二
use futures::{ self, executor};
use std::thread;
use std::time;
use async_std::task::{sleep, spawn};
async fn song() {
		loop {

    println!("song!");
	sleep(time::Duration::from_secs(5)).await;
		}
}


async fn dance() {
		loop {
	sleep(time::Duration::from_secs(5)).await;
    println!("Dance!");
		}
}


async fn async_main() {
    let f1 = song();
    let f2 = dance();
    futures::join!(f1, f2);
}
fn main() {
    executor::block_on(async_main());
    println!("Hello, world!");
}

 
