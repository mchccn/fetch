#![allow(unused_imports)]

use std::env::*;
use std::process::exit;
use url::{Url, ParseError};
use serde_json::{from_str};
use serde::{Serialize, Deserialize};
use reqwest::blocking::{Client};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use std::str::FromStr;
use colored::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Options {
    #[serde(default)]
    method: String,
    #[serde(default)]
    headers: HashMap<String, String>,
    #[serde(default)]
    credentials: String,
    #[serde(default)]
    body: String,
}

fn main() -> Result<(), ()> {
    let mut args = args().collect::<Vec<String>>();

    args.remove(0);

    if args.len() == 0 {
        println!("No host provided.");

        exit(1);
    }

    let host = Url::parse(&args.remove(0));

    if host.is_err() {
        eprintln!("{}", "[ERROR] Failed to parse URL.".red());

        exit(1);
    }

    let mut host = host.unwrap();

    if !host.scheme().starts_with("http") {
        host.set_scheme("http").unwrap();

        eprintln!("{}", "[WARN] Protocol not specified, defaulting to HTT".yellow());

        exit(1);
    }

    if host.host().is_none() {
        eprintln!("{}", "[ERROR] URL is missing a host.".red());

        exit(1);
    }

    let options = from_str::<Options>(&args.remove(0));

    if options.is_err() {
        options.unwrap();

        eprintln!("{}", "[ERROR] Failed to parse options.".red());

        exit(1);
    }

    let mut options = options.unwrap();

    println!("{:?}", options);

    options.method = options.method.to_lowercase();

    if !vec!["get", "post", "put", "delete", "head", "options", "patch"].contains(&options.method.as_str()) {
        eprintln!("{}", "[ERROR] Invalid HTTP method.".red());

        exit(1);
    }

    let mut headers = HeaderMap::new();

    for (key, entry) in options.headers.iter() {
        let name = HeaderName::from_str(key);

        if name.is_err() {
            eprintln!("{}", format!("{}{}{}", "[ERROR] Invalid header name '".red(), key, "'.".red()));

            exit(1);
        }

        let value = HeaderValue::from_str(entry);

        if value.is_err() {
            eprintln!("{}", format!("{}{}{}", "[ERROR] Invalid header value '".red(), entry, "'.".red()));

            exit(1);
        }

        headers.insert(name.unwrap(), value.unwrap());
    }
    
    //

    Client::new()
        .get(&String::from(host))
        .headers(headers)
        .send()
        .unwrap()
        .text()
        .unwrap()
        .split("\n")
        .for_each(|line| println!("{}", line));

    Ok(())
}
