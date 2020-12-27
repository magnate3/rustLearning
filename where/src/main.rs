#[derive(Debug)]
struct f_closure{
	name: String,
}
impl f_closure{
	fn fn_call( self) -> String{
			self.name
	}
}
fn get_string<T>(name: String , f : T) -> String where T : Fn(String) -> String{
		f(name)	
	}
fn main() {
    let name = String::from("kobe");
	//let kobe = f_closure{name:name,};
	let  f1= |x : String | format!("{}+ {}",x, name);
    let name2 = String::from("dirk");
	println!("name2 {}",get_string(name2, f1));

}
