//Creates a temporary directory with tempfile::Builder and synchronously downloads a file over HTTP using reqwest::get.
//Creates a target File with name obtained from Response::url within tempdir() and copies downloaded data into it with io::copy. The temporary directory is automatically removed on run function return.

#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate tempfile;

use std::io::copy;
use std::fs::File;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let mut response = reqwest::get(target)?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    copy(&mut response, &mut dest)?;
    Ok(())
}

/*
Error: Error(HttpRequest(Error(Io(Custom { kind: TimedOut, error: "timed out" }), "https://www.rust-lang.org/logos/rust-logo-512x512.png")), State { next_error: None, backtrace: InternalBacktrace { backtrace: Some(stack backtrace:
   0: error_chain::backtrace::imp::InternalBacktrace::new
             at /home/jay/.cargo/registry/src/github.com-1ecc6299db9ec823/error-chain-0.12.1/src/backtrace.rs:56
   1: <error_chain::State as core::default::Default>::default
             at /home/jay/.cargo/registry/src/github.com-1ecc6299db9ec823/error-chain-0.12.1/src/lib.rs:671
   2: dl::Error::from_kind
             at <::error_chain::error_chain::impl_error_chain_processed macros>:72
   3: <dl::Error as core::convert::From<reqwest::error::Error>>::from
             at dl/src/main.rs:13
   4: dl::main
             at dl/src/main.rs:23
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
