extern crate iron;
extern crate rand;
extern crate urlencoded;

use iron::headers;
use iron::prelude::*;
use iron::method::Method;
use iron::modifiers::{Redirect, Header};
use iron::{Handler, Url, status};

use rand::Rng;

use std::collections::HashMap;
use std::sync::RwLock;

use urlencoded::UrlEncodedBody;

struct UrlShortenerHandler {
    shortened_urls: RwLock<HashMap<String, Url>>,
}

impl UrlShortenerHandler {
    fn new() -> Self {
        UrlShortenerHandler { shortened_urls: RwLock::new(HashMap::new()) }
    }
}

impl Handler for UrlShortenerHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match &req.method {
            &Method::Post => {
                let long_url = Url::parse(req.get_ref::<UrlEncodedBody>()
                        .unwrap()
                        .get("url")
                        .unwrap()
                        .get(0)
                        .unwrap())
                    .unwrap();

                let random_key: String = rand::thread_rng().gen_ascii_chars().take(5).collect();
                let short_url = format!("http://localhost:3000/{}", random_key);

                self.shortened_urls.write().unwrap().insert(random_key, long_url);

                Ok(Response::with((status::Created, Header(headers::Location(short_url)))))
            }
            &Method::Get | &Method::Head => {
                let request_path = &req.url.path().join("/");
                match self.shortened_urls.read().unwrap().get(request_path) {
                    Some(url) => Ok(Response::with((status::Found, Redirect(url.clone())))),
                    None => Ok(Response::with(status::NotFound)),
                }
            }
            _ => Ok(Response::with(status::NotFound)),
        }
    }
}

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
