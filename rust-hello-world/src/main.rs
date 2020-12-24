// Custom smart pointer
struct CustomBox<T>{
a : T,
}
use :: std::ops::Deref;
impl<T> Deref for  CustomBox<T> {

		    type Target = T;
			fn deref(&self) -> &T
						    {
									        &self.a
							}
}

fn main() {

		let b = CustomBox{ a : 5 };
		println!("{}", *b.deref());
}
