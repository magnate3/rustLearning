use std::thread;
use std::time;
use std::sync::mpsc;

fn main() {
    let (tx, rx) : (mpsc::Sender<i32>, mpsc::Receiver<i32>)  = mpsc::channel();
    //drop(rx);
    thread::spawn( move || {
        let val = 99;
		let sendinfo = tx.send(val);
        //tx.send(val).unwrap();
		if let Err(v) = sendinfo {
		    
               println!("err inof  {}", v);
		}
    });
    thread::sleep(time::Duration::from_secs(5));
    let received = rx.recv();
	if let Err(v) = received {
		    
               println!("err inof  {}", v);
		}else {
               println!("recv info  ");
		}
    let received2 = rx.recv();
	if let Err(v) = received2 {
		    
               println!("err inof  {}", v);
		}
}
