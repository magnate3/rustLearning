use std::alloc::{alloc, dealloc, Layout};
struct Demo 
{
	size : usize,
}
fn main() {
		let layout1 = Layout::from_size_align(1, 2);
		let layout2 = Layout::from_size_align(2, 2);
		let layout3 = Layout::from_size_align(4, 4);
		let layout4 = Layout::from_size_align(8, 8);
		 println!("layout1 {:?}", layout1);
		 println!("layout2 {:?}", layout2);
		 println!("layout3 {:?}", layout3);
		 println!("layout4 {:?}", layout4);
		let layout5 = Layout::new::<u8>();
		println!("layout5 {:?}", layout5);
		let layout6 = Layout::new::<Demo>();
		println!("layout6 {:?}", layout6);
   unsafe
   {


		let layout1 = Layout::from_size_align_unchecked(1, 2);
		let layout2 = Layout::from_size_align_unchecked(2, 2);
		let layout3 = Layout::from_size_align_unchecked(4, 4);
		let layout4 = Layout::from_size_align_unchecked(8, 8);
		 println!("layout1 {:?}", layout1);
		 println!("layout2 {:?}", layout2);
		 println!("layout3 {:?}", layout3);
		 println!("layout4 {:?}", layout4);
	}
}
