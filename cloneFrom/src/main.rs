struct Rec {
width : u32,
length : u32
}
impl Clone for Rec {
    fn clone( & self) -> Rec{
			Rec{width: self.width.clone(), length : self.length.clone()}
	}
}
fn process(rec1: Rec) -> Rec {
	let mut rec2 = rec1;
   	rec2.width = 10;
    rec2.length = 11;
	rec2
}
fn main() {
		// clone is explicit , copy is implicit
		let rec = Rec{width : 4, length : 16};
		// rec.width = 100;
		// rec.length = 10;
		 println!("{},{}", rec.width, rec.length);
		 let mut rec2 =rec.clone();
		 println!("{},{}", rec2.width, rec2.length);
		 rec2.width = 100;
		 rec2.length = 10;
		 println!("{},{}", rec2.width, rec2.length);
		 rec2.clone_from(&rec);//copy
		 println!("{},{}", rec2.width, rec2.length);
		 let rec3 = process(rec);
		 println!("{},{}", rec3.width, rec3.length);
}
