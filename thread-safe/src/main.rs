use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
  let foo = Arc::new(vec![0]); 
  let bar = Arc::clone(&foo);
  thread::spawn(move || {
    thread::sleep(Duration::from_millis(20));
    println!("{:?}", *bar);
  });
  
  println!("{:?}", foo);
} 