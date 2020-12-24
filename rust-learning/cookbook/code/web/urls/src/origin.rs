//origin produces the same result.
#[macro_use]
extern crate error_chain;
extern crate url;

use url::{Url, Origin, Host};

error_chain! {
    foreign_links {
        UrlParse(url::ParseError);
    }
}

fn main() -> Result<()> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    let expected_scheme = "ftp".to_owned();
    let expected_host = Host::Domain("rust-lang.org".to_owned());
    let expected_port = 21;
    let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

    let origin = url.origin();
    assert_eq!(origin, expected);
    println!("The origin is as expected!");

    Ok(())
}

