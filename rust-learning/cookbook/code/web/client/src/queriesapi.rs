//Queries GitHub stargazers API v3 with reqwest::get to get list of all users who have marked a GitHub project with a star. reqwest::Response is deserialized with Response::json into User objects implementing serde::Deserialize.
#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;

    let users: Vec<User> = response.json()?;
    println!("{:?}", users);
    Ok(())
}

/*
https://api.github.com/repos/rust-lang-nursery/rust-cookbook/stargazers
[User { login: "trentspi", id: 20845184 }, User { login: "charlesetc", id: 5167293 }, User { login: "Ruin0x11", id: 6700637 }, User { login: "gmcabrita", id: 957820 }, User { login: "cnwalker", id: 8570077 }, User { login: "k0pernicus", id: 3605451 }, User { login: "jaxx", id: 723258 }, User { login: "dhharris", id: 9009622 }, User { login: "zhangsoledad", id: 3198439 }, User { login: "ssebastianj", id: 309535 }, User { login: "oclbdk", id: 136982 }, User { login: "Latrasis", id: 4656227 }, User { login: "narendasan", id: 1790613 }, User { login: "rishabh92", id: 13951936 }, User { login: "hueftl", id: 11706301 }, User { login: "seeekr", id: 302886 }, User { login: "krzyk", id: 105730 }, User { login: "rjammala", id: 4990663 }, User { login: "DaseinPhaos", id: 11028753 }, User { login: "jryans", id: 279572 }, User { login: "burdges", id: 680126 }, User { login: "zaa", id: 5245 }, User { login: "DenisKolodin", id: 418920 }, User { login: "messense", id: 1556054 }, User { login: "wdv4758h", id: 2716047 }, User { login: "iblis17", id: 761623 }, User { login: "realityone", id: 4059040 }, User { login: "sebasmagri", id: 11137 }, User { login: "sourcepirate", id: 5940286 }, User { login: "king6cong", id: 302560 }]

*/
