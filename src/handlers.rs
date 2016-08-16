extern crate rand;

use iron::headers;
use iron::prelude::*;
use iron::method::Method;
use iron::modifiers::{Redirect, Header};
use iron::{Handler, Url, status};

use rand::Rng;

use std::collections::HashMap;
use std::sync::RwLock;

use urlencoded::UrlEncodedBody;

pub struct UrlShortenerHandler {
    shortened_urls: RwLock<HashMap<String, Url>>,
}

impl UrlShortenerHandler {
    pub fn new() -> Self {
        UrlShortenerHandler { shortened_urls: RwLock::new(HashMap::new()) }
    }
}

impl Handler for UrlShortenerHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match &req.method {
            &Method::Post => {
                let long_url = req.get_ref::<UrlEncodedBody>()
                    .map_err(|_| "URL encoded body missing")
                    .and_then(|data| match data.get("url") {
                        Some(urls) => Ok(urls.first().unwrap()),
                        None => Err("Parameter 'url' missing from the URL encoded body"),
                    })
                    .and_then(|url| Url::parse(url).map_err(|_| "Parameter 'url' is invalid"));

                match long_url {
                    Ok(long_url) => {
                        let random_key: String =
                        rand::thread_rng().gen_ascii_chars().take(5).collect();
                        let short_url = format!("http://localhost:3000/{}", random_key);

                        self.shortened_urls.write().unwrap().insert(random_key, long_url);

                        Ok(Response::with((status::Created, Header(headers::Location(short_url)))))
                    }
                    Err(e) => Ok(Response::with((status::BadRequest, e))),
                }
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
