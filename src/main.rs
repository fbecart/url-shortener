extern crate hyper;
extern crate iron;
extern crate rand;
extern crate urlencoded;

#[cfg(test)]
extern crate iron_test;

mod handlers;
mod storage;

use iron::prelude::*;

use std::env;

use self::handlers::UrlShortenerHandler;
use self::storage::InMemoryKeyValueStore;

fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let short_url_prefix = env::var("SHORT_URL_PREFIX")
        .unwrap_or(format!("http://localhost:{}/", port));

    println!("Starting URL Shortener on port {}...", port);

    let addr = format!("localhost:{}", port);
    let handler = UrlShortenerHandler::new(short_url_prefix, InMemoryKeyValueStore::new());
    match Iron::new(handler).http(&addr[..]) {
        Ok(_) => println!("Server started"),
        Err(e) => {
            println!("Error: {}", e.to_string());
            std::process::exit(-1);
        }
    }
}
