 trait Hello {
     fn say_hi(&self) {
         println!("hi");
     }
 }
 ​
 struct Student {}
 impl Hello for Student {}
 struct Teacher {}
 impl Hello for Teacher {}
 ​
 fn main() {
     let s = Student {};
     s.say_hi();
     let t = Teacher {};
     t.say_hi();
 }