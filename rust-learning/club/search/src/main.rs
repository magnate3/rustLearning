 use binary::binary_search;
fn main() {
let sa=[2, 3, 3, 6, 6, 7, 9, 13, 15, 19, 20, 22, 23, 24, 25];
//pub fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
 let key = 9;
 let k = binary_search(&sa,&key);
  println!("{:?}",k);
}

