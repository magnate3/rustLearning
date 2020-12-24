// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// Execute `rustlings hint clippy1` for hints :)

// strict comparison of `f32` or `f64`

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    if x.to_string() !=  y.to_string(){
    //if y != x {
        println!("Success!");
    }
}
