//Pull the source of a MediaWiki page using reqwest::get and then look for all entries of internal and external links with Regex::captures_iter. Using Cow avoids excessive String allocations.

//MediaWiki link syntax is described here.
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate regex;

use std::io::Read;
use std::collections::HashSet;
use std::borrow::Cow;
use regex::Regex;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        Regex(regex::Error);
    }
}

fn extract_links(content: &str) -> Result<HashSet<Cow<str>>> {
    lazy_static! {
        static ref WIKI_REGEX: Regex =
            Regex::new(r"(?x)
                \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                |
                (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
            ").unwrap();
    }

    let links: HashSet<_> = WIKI_REGEX
        .captures_iter(content)
        .map(|c| match (c.name("internal"), c.name("external")) {
            (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
            (None, Some(val)) => Cow::from(val.as_str()),
            _ => unreachable!(),
        })
        .collect();

    Ok(links)
}

fn main() -> Result<()> {
    let mut content = String::new();
    reqwest::get(
        "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
    )?
        .read_to_string(&mut content)?;

    println!("{:#?}", extract_links(&content)?);

    Ok(())
}

/*

Error: Error(Reqwest(Error(Io(Custom { kind: TimedOut, error: "timed out" }), "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw")), State { next_error: None, backtrace: InternalBacktrace { backtrace: Some(stack backtrace:
   0: error_chain::backtrace::imp::InternalBacktrace::new
             at /home/jay/.cargo/registry/src/github.com-1ecc6299db9ec823/error-chain-0.12.1/src/backtrace.rs:56
   1: <error_chain::State as core::default::Default>::default
             at /home/jay/.cargo/registry/src/github.com-1ecc6299db9ec823/error-chain-0.12.1/src/lib.rs:671
   2: link::Error::from_kind
             at <::error_chain::error_chain::impl_error_chain_processed macros>:72
   3: <link::Error as core::convert::From<reqwest::error::Error>>::from
             at link/src/main.rs:16
   4: link::main
             at link/src/main.rs:50
   5: std::rt::lang_start::{{closure}}
             at /rustc/1423bec54cf2db283b614e527cfd602b481485d1/src/libstd/rt.rs:61
   6: std::rt::lang_start_internal::{{closure}}
             at src/libstd/rt.rs:48
      std::panicking::try::do_call
             at src/libstd/panicking.rs:287
   7: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:78
   8: std::panicking::try
             at src/libstd/panicking.rs:265
      std::panic::catch_unwind
             at src/libstd/panic.rs:396
      std::rt::lang_start_internal
             at src/libstd/rt.rs:47
   9: std::rt::lang_start
             at /rustc/1423bec54cf2db283b614e527cfd602b481485d1/src/libstd/rt.rs:61
  10: main
  11: __libc_start_main
  12: _start
) } })

*/
