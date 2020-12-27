// Custom smart pointer
struct CustomBox<T>{
a : T,
}
use :: std::ops::Deref;
use :: std::ops::DerefMut;
impl<T> Deref for  CustomBox<T> {

		    type Target = T;
			fn deref(&self) -> &T
						    {
									        &self.a
							}
}

fn main() {
        let a :i32= 88;
		let b : &i32 = & a;
		println!(" a == *b ? {}", a == *b );
		let val1 :i32= 32;
		let val2:Packing<i32> = Packing::new(32);
		// complie error ： help: consider dereferencing the type: `*val2` 
		//println!(" val1 == val2 ? {}", val1  == val2 );
		println!(" val1 == val2 ? {}", val1  == *val2 );
		let mut val3:Packing<i32> = Packing::new(88);
		println!(" val3  {}", *val3 );
		*val3 = 99;
		println!(" val3  {}", *val3 );
}

struct Packing<T>
{
obj : T
}

impl <T>  Packing<T>{
		fn new(x : T) ->  Packing<T>
		{
             Packing{obj: x}
		}
}

impl<T> Deref for  Packing<T> {
		    type Target = T;
			fn deref(&self) ->  & T // * C++
						    {
									        &self.obj
							}
}
impl<T> DerefMut for  Packing<T> {

			// * C++
			fn deref_mut(& mut self) -> & mut   T
						    {
									        & mut self.obj
							}
}
