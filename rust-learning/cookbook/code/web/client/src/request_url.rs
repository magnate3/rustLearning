//Query the GitHub Users Endpoint using a HEAD request (Client::head) and then inspect the response code to determine success. This is a quick way to query a rest resource without needing to receive a body. reqwest::Client cofigured with ClientBuilder::timeout ensures a request will not last longer than a timeout.
extern crate reqwest;

use reqwest::Error;
use std::time::Duration;
use reqwest::ClientBuilder;


fn main() -> Result<(), Error> {
//    let user = "ferris-the-crab";
    let user = "rdigua";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send()?;

    if response.status().is_success() {
        println!("{} is a user!", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}

/*

https://api.github.com/users/rdigua
rdigua is a user!


https://api.github.com/users/ferris-the-crab
ferris-the-crab is not a user!

*/
