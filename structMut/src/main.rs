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
impl Person {
	fn add_age(& mut self, a: u32) -> u32{
			self.age = self.age + a;
			self.age
	}
}
fn main() {
	let kobe = Person { age: 18 };
	println!("age {}",kobe.get_age());
	println!("age {}",kobe.add_age(1));
	println!("age {}",kobe.get_age());
}
