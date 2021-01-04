use std::thread;
use std::time;
use std::sync::mpsc;

fn main() {
    let (tx, rx) : (mpsc::Sender<i32>, mpsc::Receiver<i32>)  = mpsc::channel();
	let tx2 = tx.clone();
    thread::spawn( move || {
        let val = 99;
		let sendinfo = tx.send(val);
		if let Err(v) = sendinfo {
		    
               println!("err inof  {}", v);
		}
        thread::sleep(time::Duration::from_secs(10));
        println!("thread1 over  ");
    });
    thread::spawn( move || {
        let val = 88;
		let sendinfo = tx2.send(val);
		if let Err(v) = sendinfo {
		    
               println!("err inof  {}", v);
		}
        thread::sleep(time::Duration::from_secs(10));
        println!("thread2 over  ");
    });
    thread::sleep(time::Duration::from_secs(1));
	for r in rx
	{
        println!("recv inof  {}", r);
	}
    println!("main thread over  ");
}
