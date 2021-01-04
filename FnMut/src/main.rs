fn consume_with_relish<F>(mut func: F) where F: FnMut()
{
    // `func` consumes its captured variables, so it cannot be run more
    // than once
    println!("Consumed 1 ");
    func();
    println!("Consumed 2");
    func();

    println!("Delicious!");

    // Attempting to invoke `func()` again will throw a `use of moved
    // value` error for `func`
}
fn main() {
	let mut  x = String::from("x");
    let consume_and_return_x = move || {
			x.push('*');
			println!("x {}", x);
	};
    consume_with_relish(consume_and_return_x);

// `consume_and_return_x` can no longer be invoked at this point
}
