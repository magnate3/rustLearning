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
	//let kobe = f_closure{name:name,};
	let  f1= || name;
	println!("name {}",f1());
	println!("name {}",f1());
	//println!("name {}",kobe.fn_call());
	//println!("name {}",kobe.fn_call());
}
