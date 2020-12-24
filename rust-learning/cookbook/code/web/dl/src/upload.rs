//reqwest::Client establishes a connection to https://paste.rs following the reqwest::RequestBuilder pattern. Calling Client::post with a URL establishes the destination, RequestBuilder::body sets the content to send by reading the file, and RequestBuilder::send blocks until the file uploads and the response returns. read_to_string returns the response and displays in the console.
extern crate reqwest;

#[macro_use]
extern crate error_chain;

use std::fs::File;
use std::io::Read;
use reqwest::Client;

error_chain! {
    foreign_links {
        HttpRequest(reqwest::Error);
        IoError(::std::io::Error);
    }
}

fn main() -> Result<()> {
    let paste_api = "https://paste.rs";
    let file = File::open("message")?;

    let mut response = Client::new().post(paste_api).body(file).send()?;
    let mut response_body = String::new();
    response.read_to_string(&mut response_body)?;
    println!("Your paste is located at: {}", response_body);
    Ok(())
}

/*
Error: Error(IoError(Os { code: 2, kind: NotFound, message: "No such file or directory" }), State { next_error: None, backtrace: InternalBacktrace { backtrace: Some(stack backtrace:
   0: error_chain::backtrace::imp::InternalBacktrace::new
             at /home/jay/.cargo/registry/src/github.com-1ecc6299db9ec823/error-chain-0.12.1/src/backtrace.rs:56
   1: <error_chain::State as core::default::Default>::default
             at /home/jay/.cargo/registry/src/github.com-1ecc6299db9ec823/error-chain-0.12.1/src/lib.rs:671
   2: dl::Error::from_kind
             at <::error_chain::error_chain::impl_error_chain_processed macros>:72
   3: <dl::Error as core::convert::From<std::io::error::Error>>::from
             at dl/src/main.rs:11
   4: dl::main
             at dl/src/main.rs:20
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



It need a file that named message. It is ok when we create a file at excute dirctory.


*/

