//This program depicts the use of ansi_term crate and how it is used for controlling colours and formatting, such as blue bold text or yellow underlined text, on ANSI terminals.

// There are two main data structures in [ansi_term]: ANSIString and Style. A Style holds stylistic information: colours, whether the text should be bold, or blinking, or whatever. There are also Colour variants that represent simple foreground colour styles. An ANSIString is a string paired with a Style.

// Note: British English uses Colour instead of Color, don't get confused

extern crate ansi_term;

use ansi_term::Colour;

fn main() {
    println!("This is {} in color, {} in color and {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));
}

