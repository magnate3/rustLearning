use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
	let m1 = &m;
	let m2 = &m;
    //println!("Hello,m1:  {:?}!", *m1);
    //println!("Hello,m2:  {:?}!", *m2);
    hello(&m);
}
