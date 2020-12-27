#[derive(Debug)]
struct Person {
	age: u32,
}
impl Person {
	fn get_age(self) -> u32{
			self.age
	}
}
fn main() {
    //let val1 :i32 = 5;
    //let val2 :i32 = 5;
    //let f1 = |x:i32|-> i32{ val1*x + val2};
	let kobe = Person { age: 18 };
	println!("age {}",kobe.get_age());
	println!("age {}",kobe.get_age());
}
