//进程共享mutex
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  //错误，第一个线程搬走counter

            *num += 1;  //deref MutexGuard
        });
        handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());  //deref MutexGuard
}
