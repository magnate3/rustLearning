//The parse method from the url crate validates and parses a &str into a Url struct. The input string may be malformed so this method returns Result<Url, ParseError>.
//Once the URL has been parsed, it can be used with all of the methods in the Url type.
extern crate url;

use url::{Url, ParseError};

fn main() -> Result<(), ParseError> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(())
}

