struct Rec {
width : u32,
length : u32
}
impl Clone for Rec {
    fn clone( & self) -> Rec{
			Rec{width: self.width.clone(), length : self.length.clone()}
	}
}
impl Copy for Rec {
			//Rec{width: self.width.clone(), length : self.length.clone()}
		 //println!("do copy");
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
		 println!("{},{}", rec.width, rec.length);
		 let rec2 =rec; // copy, not move because of impl of copy
		 println!("{},{}", rec2.width, rec2.length);
		 let rec3 = process(rec);
		 println!("{},{}", rec3.width, rec3.length);
}
