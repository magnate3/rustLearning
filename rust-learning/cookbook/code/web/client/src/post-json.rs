//Creates a gist with POST request to GitHub gists API v3 using Client::post and removes it with DELETE request using Client::delete.
//The reqwest::Client is responsible for details of both requests including URL, body and authentication. The POST body from serde_json::json! macro provides arbitrary JSON body. Call to RequestBuilder::json sets the request body. RequestBuilder::basic_auth handles authentication. The call to RequestBuilder::send synchronously executes the requests.

#[macro_use]
extern crate error_chain;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use reqwest::Client;

error_chain! {
    foreign_links {
        EnvVar(env::VarError);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Deserialize, Debug)]
struct Gist {
    id: String,
    html_url: String,
}

fn main() -> Result<()> {
    let gh_user = env::var("GH_USER")?;
    let gh_pass = env::var("GH_PASS")?;

    let gist_body = json!({
        "description": "the description for this gist",
        "public": true,
        "files": {
             "main.rs": {
             "content": r#"fn main() { println!("hello world!");}"#
            }
        }});

    let request_url = "https://api.github.com/gists";
    let mut response = Client::new()
        .post(request_url)
        .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
        .json(&gist_body)
        .send()?;

    let gist: Gist = response.json()?;
    println!("Created {:?}", gist);

    let request_url = format!("{}/{}",request_url, gist.id);
    let response = Client::new()
        .delete(&request_url)
        .basic_auth(gh_user, Some(gh_pass))
        .send()?;

    println!("Gist {} deleted! Status code: {}",gist.id, response.status());
    Ok(())
}

/*
Error: Error(EnvVar(NotPresent), State { next_error: None, backtrace: InternalBacktrace { backtrace: Some(stack backtrace:
   0: error_chain::backtrace::imp::InternalBacktrace::new
             at /home/jay/.cargo/registry/src/github.com-1ecc6299db9ec823/error-chain-0.12.1/src/backtrace.rs:56
   1: <error_chain::State as core::default::Default>::default
             at /home/jay/.cargo/registry/src/github.com-1ecc6299db9ec823/error-chain-0.12.1/src/lib.rs:671
   2: client::Error::from_kind
             at <::error_chain::error_chain::impl_error_chain_processed macros>:72
   3: <client::Error as core::convert::From<std::env::VarError>>::from
             at client/src/main.rs:15
   4: client::main
             at client/src/main.rs:29
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
