trait IShape {
     fn area(&self) -> i32; 
 //    fn area(&self) -> i32 {
 //        println!("area");
 //    }
 }

struct Rec {
width :i32,
length:i32
 }

 impl IShape for Rec {
     fn area(&self)  -> i32 {
       self.width * self.length     
     }
 }
 fn ShapeInfo(T : impl IShape) {
		 println!("hi, shape area is {}", T.area());
 }
 fn GetShape() ->  impl IShape {
      Rec{width : 3, length : 4}

 }
 fn main() {
     let rec = Rec{width : 3, length : 4};
	 println!("hi, rec area is {}", rec.area());
	 ShapeInfo(rec);
	 println!("hi, rec area is {}", GetShape().area());
 }
