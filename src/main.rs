extern crate hyper;
extern crate iron;
extern crate rand;
extern crate urlencoded;

#[cfg(test)]
extern crate iron_test;

mod handlers;

use iron::prelude::*;

use std::env;

use self::handlers::UrlShortenerHandler;

fn main() {
    println!("Starting URL Shortener on port 3000...");

    let short_url_prefix = env::var("SHORT_URL_PREFIX")
        .unwrap_or("http://localhost:3000/".to_string());
    let handler = UrlShortenerHandler::new(short_url_prefix);
    match Iron::new(handler).http("localhost:3000") {
        Ok(_) => println!("Server started"),
        Err(e) => {
            println!("Error: {}", e.to_string());
            std::process::exit(-1);
        }
    }
}
