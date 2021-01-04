fn main() {

   let s1=no_dangle();
   //let s1=no_dangle();
   println!("s1 is {}.", s1);
}

fn no_dangle() -> String {
	let s = String::from("hello");
	s
}
//fn dangle() -> & String {
//	let s = String::from("hello");
//	&s
//}
