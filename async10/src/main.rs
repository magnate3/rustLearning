//例子二
use futures::{ self, executor,join};
use std::thread;
use std::time;
use async_std::task::{sleep,spawn};
use async_std::task;
async fn worker1(){
    println!("start worker1!");
	sleep(time::Duration::from_secs(5)).await;
    println!("stop worker1!");
}
async fn worker2(){
    println!("start worker2!");
	sleep(time::Duration::from_secs(5)).await;
    println!("stop worker2!");
}
async fn worker3(){
    println!("start worker3!");
	sleep(time::Duration::from_secs(5)).await;
    println!("stop worker3!");
}
async fn async_main() {
	let f1 = async {
	worker1().await;
	worker2().await;
	};
	let f2 = worker2();
	let  handle1 = task::spawn(f1);
	let  handle2 = task::spawn(f2);
	handle1.await;
	handle2.await;
}
async fn  async_main2() {
    println!("async main2 start !");
}

fn main() {
	let  f1 = task::spawn(async_main());
    executor::block_on(f1);
    println!("Hello, world!");
}

 
