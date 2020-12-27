#![feature(core_intrinsics)]
fn print_type_of<T>(_: &T) {  //检测变量类型
  println!("{}", std::intrinsics::type_name::<T>());
}

#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroyed struct E");
    }
}
fn fn_fn<F>(func: F) where F: Fn() {
    println!("fn begins");
    func();
    func();
    println!("fn ended");
}

fn fn_mut<F>(mut func: F) where F: FnMut() {
    println!("fn_mut begins");
    func();
    // func();
    println!("fn_mut ended");
}
fn fn_once<F>(func: F) where F: FnOnce() {
    println!("fn_once begins");
    func();
    // func();
    println!("fn_once ended");
}
fn main() {
    let /*mut*/ e = E { a: "original".to_string() };
    // let e = 32;
    // e.a="test".to_string();
    
    // let f = move || println!("fn once calls: {:?}", e);
    let f = /*move*/ || { println!("FnMut closure calls: {:?}", e); };
    // let f = || { println!("FnMut closure calls: {:?}", e); e.a = "fn_mut".to_string(); };
    // f();
    // f();
    let mut x: Box<dyn FnMut() -> ()> = Box::new(f); //检测隐含的trait
    x();
    // e.a="test".to_string();
    fn_fn(f);
    fn_mut(f);
    fn_once(f);
    // println!("fn once calls: {:?}", e);
    println!("main ended");
}