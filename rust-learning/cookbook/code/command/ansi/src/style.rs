//For anything more complex than plain foreground colour changes, the code needs to construct Style struct. Style::new() creates the struct, and properties chained.
extern crate ansi_term;

use ansi_term::Style;

fn main() {
    println!("{} and this is not",
             Style::new().bold().paint("This is Bold"));
}

