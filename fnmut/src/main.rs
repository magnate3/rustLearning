#[derive(Debug)]
struct f_closure{
	name: String,
}
impl f_closure{
	fn fn_call( self) -> String{
			self.name
	}
}
fn get_string<T>(name: String , mut f: T) -> String where T : FnMut(String) -> String{
		f(name)	
	}
fn main() {
			let mut name = String::from("kobe");
			let  f1= |x : String | -> String {
		    name.push_str("24");
		   	format!("{}+ {}",x, name)
	};
    let name2 = String::from("dirk");
	println!("name2 {}",get_string(name2, f1));
    //let name3 = String::from("lakers ");
	//println!("name2 {}",get_string(name3, f1));

}
