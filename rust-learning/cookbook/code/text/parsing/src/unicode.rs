//Collect individual Unicode graphemes from UTF-8 string using the UnicodeSegmentation::graphemes function from the unicode-segmentation crate.
#[macro_use]
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let name = "José Guimarães\r\n";
    let graphemes = UnicodeSegmentation::graphemes(name, true)
        .collect::<Vec<&str>>();
    assert_eq!(graphemes[3], "é");
}

