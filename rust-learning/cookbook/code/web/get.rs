//Use reqwest::get to perform a HTTP GET request and then use Document::from_read to parse the response into a HTML document. find with the criteria of Name is "a" retrieves all links. Call filter_map on the Selection retrieves URLs from links that have the "href" attr (attribute).

#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

error_chain! {
   foreign_links {
       ReqError(reqwest::Error);
       IoError(std::io::Error);
   }
}

fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")?;

    Document::from_read(res)?
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}

