#[derive(Debug)]
struct MyType {
    name: String
}

impl MyType {
    fn do_something(self, age: u32) {
    //等价于 fn do_something(self: Self, age: u32) {
    //等价于 fn do_something(self: MyType, age: u32) {
        println!("name = {}", self.name);
        println!("age = {}", age);
    }

    fn do_something2(&self, age: u32) {
        println!("do_something2 name = {}", self.name);
        println!("do_something2 age = {}", age);
    }
}

fn main() {
    let my_type = MyType{name: "linghuyichong".to_string()};
    //使用self
    my_type.do_something(18);   //等价于MyType::do_something(my_type, 18);
    //my_type.do_something(81);   //
    //println!("my_type: {:#?}", my_type);    //在do_something中，传入的是对象，而不是引用，因此my_type的所有权就转移到函数中了，因此不能再使用

    //使用&self
    let my_type2 = MyType{name: "linghuyichong".to_string()};
    my_type2.do_something2(18);
    my_type2.do_something2(18);
    println!("my_type2: {:#?}", my_type2);//在do_something中，传入是引用，函数并没有获取my_type2的所有权，因此此处可以使用
    println!("Hello, world!");
}

 
