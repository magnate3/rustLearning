
fn main() {
    let doubler = |x| x * 2;
    let value = 5;
    let twice = doubler(value);
    println!(
        "------------> {} doubled is {} <-------------",
        value, twice
    );

    let big_closure = |b, c| {
        let z = b + c;
        z * twice + value //注意这句，闭包把周围的变量都捕捉到了。这比较危险，但很方便有效
    };

    let some_number = big_closure(1, 2);
    println!("Result from closure: {}", some_number);

    let get_value = || value; //没有参数哦
    println!("get_value: {}", get_value());
    println!("get_value: {}", get_value()); //可以执行多次

    let s = String::from("abcde");
    let get_value = || s;
    println!("get_value: {}", get_value());
    //println!("get_value: {}", get_value());
    //上面这句将出错，因为s的所有权转移了，get_value()只能执行一次。从MIR中看到 FnOnce 函数
 
    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);
    //println!("{:?}",haystack); //如果去掉move，这行能工作
    println!("{}", contains(&1));
    println!("{}", contains(&4));
}
