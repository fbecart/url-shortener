extern crate hyper;
extern crate iron;
extern crate rand;
extern crate urlencoded;

#[cfg(test)]
extern crate iron_test;

mod handlers;

use iron::prelude::*;

use self::handlers::UrlShortenerHandler;

fn main() {
    println!("Starting URL Shortener on port 3000...");

    let handler = UrlShortenerHandler::new();
    match Iron::new(handler).http("localhost:3000") {
        Ok(_) => println!("Server started"),
        Err(e) => {
            println!("Error: {}", e.to_string());
            std::process::exit(-1);
        }
    }
}
