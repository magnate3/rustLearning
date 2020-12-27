#[derive(Debug)]
struct f_closure{
	name: String,
}
impl f_closure{
	fn fn_call( self) -> String{
			self.name
	}
}
fn main() {
    let name = String::from("kobe");
	let kobe = f_closure{name:name,};
	println!("name {}",kobe.fn_call());
	println!("name {}",kobe.fn_call());
}
