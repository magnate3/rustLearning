use std::alloc::{alloc, dealloc, Layout};
struct Demo 
{
	size : usize,
}
fn main() {
   unsafe
   {
		let layout6 = Layout::new::<Demo>();
		let ptr : * mut Demo  = alloc(layout6) as * mut Demo;
		let mut box1 : Box<Demo> = Box::from_raw(ptr);
		(*box1).size=99;
		println!("layout6 {:?}", layout6);
		println!("box1 {}", (*box1).size);
		// not need bo delloc
	}
   unsafe
   {
		let layout6 = Layout::new::<Demo>();
		let ptr : * mut Demo  = alloc(layout6) as * mut Demo;
		(*ptr).size=99;
		println!("layout6 {:?}", layout6);
		println!("ptr {:?}", ptr);
		println!("ptr {}", (*ptr).size);
		dealloc(ptr as *mut u8,layout6);
	}
   unsafe
   {
		let layout6 = Layout::new::<Demo>();
		let ptr : * mut u8  = alloc(layout6) as * mut u8;
		println!("layout6 {:?}", layout6);
		println!("ptr {:?}", ptr);
		println!("ptr {}", *ptr);
		dealloc(ptr,layout6);
	}
   unsafe
   {
		let layout7 = Layout::new::<i32>();
		let ptr2 : * mut i32  = alloc(layout7) as *  mut i32;
		println!("ptr {:?}", ptr2);
		*ptr2 = 89;
		println!("*ptr2 {}", *ptr2);
		dealloc(ptr2 as *mut u8,layout7);
	}
}
