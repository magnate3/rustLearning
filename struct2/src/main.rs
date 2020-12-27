#[derive(Debug)]
struct Person {
	age: u32,
}
impl Person {
	fn get_age(& self) -> u32{
			self.age
	}
}
impl Person {
	fn get_age_ref(& self) -> &u32{
			&self.age
	}
}
fn main() {
	let kobe = Person { age: 18 };
	println!("age {}",kobe.get_age());
	println!("age {}",kobe.get_age());
	println!("age {}",kobe.get_age_ref());
	println!("age {}",kobe.get_age_ref());
}
