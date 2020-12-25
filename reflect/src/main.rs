use std::any::{Any, TypeId};

fn main() {
    let v1 = "Jackey";
    let mut a: &Any;
    a = &v1;
    println!("{:?}", a.type_id());
    assert!(a.is::<&str>());


    print_any(&v1);
    let v2: u32 = 33;
    print_any(&v2);
}

fn print_any(any: &Any) {
    if let Some(v) = any.downcast_ref::<u32>() {
        println!("u32 {:x}", v);
    } else if let Some(v) = any.downcast_ref::<&str>() {
        println!("str {:?}", v);
    } else {
        println!("else");
    }
}