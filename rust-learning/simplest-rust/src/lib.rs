//! It is usually using fuctions.

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
// use trace_caller::trace;
//use std::io::prelude::*;

use regex::Regex;
//use reqwest::ClientBuilder;
//use reqwest::Client;
//use reqwest::header::ETAG;

//#[trace]

/// It is geted substring from string.
pub fn getmidstr(start:&str,end:&str,s:&str)->String{
  //  let mut new = None;
    //let mut place = (x:usize,y:usize);
    let mut placex:usize=0;
    let mut placey:usize=0;
    if let  Some(first)=s.find(start){
        // The number counted is from the length of start substring. 
        placex=first+start.len(); 
    };
    if let Some(last)=s.find(end){
        placey=last;
    }

    // It's important to note that this is a slice of bytes, it will not actually return the first six characters.
    // To be careful - this code can cause panicking.
    // A example can be see tlib/middlesubstring.rs
     let slice = &s[placex..placey];

     slice.to_string()
  
}

/// To save a String to a new file. If ok, it was return true else return false.
pub fn str_file(p: &str, savestr: &str) -> bool {
    if !Path::new(p).exists() {
        let mut f = File::create(p).expect("It is not ceate file.");
        f.write_all(savestr.as_bytes()).expect("Writed failed.");
        return true;
    }
    false
}

// It is return a string all readed from a file.
pub fn file_str(p: &str) -> String {
    let mut buffer: String = String::new();
    let mut f = File::open(p).expect("It is not open file.");

    f.read_to_string(&mut buffer)
        .expect("It is geted string from  file.");

    buffer
}


/// The directory is or not exist;
pub fn exist_dir(p: &str) -> bool {
    Path::new(p).exists() && Path::new(p).is_dir()
}

// The file is or not exist;
pub fn exist_file(p: &str) -> bool {
    Path::new(p).exists() && Path::new(p).is_file()
}

/// Returned all u32 from string.
pub fn strtou32(s: &str) -> u32 {
    let stu = s.to_string();
    let picks: String = stu.chars().filter(|x| x.is_digit(10)).collect();

    if picks.len() == 0 {
        return 0;
    }

    picks.parse::<u32>().unwrap()

}


///Return string from a url using get.
pub fn getcon(link: &str) -> String {
    let c = reqwest::Client::new()
        .get(link)
        .send()
        .unwrap()
        .text()
        .unwrap();
    c.to_string()

    //    let client = Client::new();
    //    let  r = client.get(link).text()?;
    //    if r.status().is_success{
    //        return r.text();
    //    } else {
    //        return String::from("?");
    //    }
    /*
    match reqwest::get(link){


    }

    ?.text()?;
    r

    let mut body = String::new();
    let client = reqwest::Client::new();
    let response = client.head(link).send()?;
    if response.status().is_success(){
        response.to_string(&body);
    }
    body
    */
}

/// To get list(vec) from string using Regex.
pub fn getlistvec(re: &str, content: &str) -> Vec<String> {
    let re1 = Regex::new(re).unwrap();

    let mut rveg: Vec<String> = Vec::new();

    for cap in re1.captures_iter(&content) {
        rveg.push(cap[1].to_string());
    }
    rveg
}

// #[trace]

/// To get list(String) from string using Regex.

pub fn getliststr(re: &str, content: &str) -> String {
    let re1 = Regex::new(re).unwrap();
    let mut rstr = String::new();
    for cap in re1.captures_iter(&content) {
        rstr.push_str(&cap[1].to_string());
        rstr.push_str(" ");
    }
    rstr
}
