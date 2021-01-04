//进程共享mutex
use std::sync::Mutex;
use std::thread;
//使用Rc<T>共享
use std::rc::Rc;
fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
	for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap(); 
		//错误，Rc<T>不是线程安全
		//Rc<T>.lock -> Mutex<>.lock

            *num += 1;  //deref MutexGuard
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());  //deref MutexGuard
}
