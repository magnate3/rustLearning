struct Foo<'val> {
		    x: &'val i32,
}

fn main() {
		    let y = &5; 
			let f = Foo { x: y };
			println!("{}", f.x);
}
