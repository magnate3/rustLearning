struct Example {

		    a: i32
}

// when an instance of example
// goes out of scope, use this
// custom drop() method
impl Drop for Example {
		    fn drop(&mut self) {
			println!("Custom Drop: Dropping the instance of Example: {}", self.a);
		    }
}

fn main() {

		    let var_1 = Example{a: 1};
		    let var_2 = Example{a: 5};
		    println!("Example instances created");
}
