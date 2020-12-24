// functions1.rs
// Make me compile! Execute `rustlings hint functions1` for hints :)

// I AM  DONE

fn main() {
    call_me();
}

fn call_me(){
	
	use std::process::Command;
let r=Command::new("rustlings")
.arg("hint")
.arg("functions")
.output()
.expect("errors.");

println!("{:?}",r);

let k = r.stdout;	
println!("{:?}",k);
	
}