//例子二
use futures::{ self, executor,join};
use std::thread;
use std::time;
use async_std::task::{sleep};
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
	let inner1 = async {
    worker1().await;
    worker2().await;
	};
	let inner2 = worker3();
	join!( inner1, inner2);

	};
	join!( f1);

}
fn main() {
    executor::block_on(async_main());
    println!("Hello, world!");
}

 
