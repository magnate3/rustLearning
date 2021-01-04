fn main() {
    //let s1 = gives_ownership();
	//println!("s1 : {}", s1);
    let s2 = String::from("hello");
    let s3 = take_and_gives_back(s2);
	println!("s3 : {}", s3);
} 
  // 此时执行drop释放堆中两个 hello 字符串，所有者是：s1 和 s3
  // hello -> s1
  // hello -> s2 -> a_string -> s3
//fn gives_ownership() {
//    let some_string = String::from("hello");
//    some_string
//}
fn take_and_gives_back(a_string: String) -> String {
    a_string
}
