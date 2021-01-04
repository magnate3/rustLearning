use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);  //Mutex<i32>, 智能指针

    {
        let mut num = m.lock().unwrap();  //等待，阻塞线程，
        //返回MutexGuard, 智能指针，实施deref and drop
        //如果其他线程锁定并惊慌，谁都得不到锁，所以直接unwrap
        *num = 6;  //num is &mut i32, deref
    }

    println!("m = {:?}", m);
}