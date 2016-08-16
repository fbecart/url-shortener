extern crate iron;

use std::collections::HashMap;

use iron::prelude::*;
use iron::modifiers::Redirect;
use iron::{Handler, Url, status};

struct UrlShortenerHandler {
    shortened_urls: HashMap<String, Url>,
}

impl UrlShortenerHandler {
    fn new() -> Self {
        let mut static_routes = HashMap::new();

        let google_url = "http://google.com";
        static_routes.insert("google".to_string(), Url::parse(google_url).unwrap());

        let rockets_url = "https://www.washingtonpost.com/graphics/business/rockets/";
        static_routes.insert("rockets".to_string(), Url::parse(rockets_url).unwrap());

        UrlShortenerHandler { shortened_urls: static_routes }
    }
}

impl Handler for UrlShortenerHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let request_path = &req.url.path().join("/");
        match self.shortened_urls.get(request_path) {
            Some(url) => Ok(Response::with((status::Found, Redirect(url.clone())))),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}

fn main() {
    let handler = UrlShortenerHandler::new();
    Iron::new(handler).http("localhost:3000").unwrap();
}
