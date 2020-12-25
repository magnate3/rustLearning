// 生命周期 `'a` 和 `'b`。这两个生命周期都必须至少要和 `print_refs`
// 函数的一样长。
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
println!("x is {} and y is {}", x, y);
}
fn main() {
// 创建变量，给下面代码借用。
let (four, nine) = (4, 9);
// 两个变量的借用（`&`）都传进函数。
print_refs(&four, &four);
println!("four is {} and nine is {}", four, nine);
}
