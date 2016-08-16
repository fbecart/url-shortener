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

    fn extract_url(req: &mut Request) -> Result<Url, &'static str> {
        req.get_ref::<UrlEncodedBody>()
            .map_err(|_| "URL encoded body missing")
            .and_then(|data| match data.get("url") {
                Some(urls) => Ok(urls.first().unwrap()),
                None => Err("Parameter 'url' missing from the URL encoded body"),
            })
            .and_then(|url| Url::parse(url).map_err(|_| "Parameter 'url' is invalid"))
    }

    fn gen_unused_random_key(shortened_urls: &HashMap<String, Url>) -> String {
        let mut url_key: Option<String> = None;
        while url_key.is_none() {
            let random_key: String = rand::thread_rng().gen_ascii_chars().take(5).collect();
            url_key = if shortened_urls.contains_key(&random_key) {
                None
            } else {
                Some(random_key)
            }
        }
        url_key.unwrap()
    }

    fn handle_post_request(&self, req: &mut Request) -> IronResult<Response> {
        match Self::extract_url(req) {
            Ok(long_url) => {
                let mut shortened_urls = self.shortened_urls.write().unwrap();
                let url_key = Self::gen_unused_random_key(&shortened_urls);

                let short_url = format!("http://localhost:3000/{}", url_key);
                shortened_urls.insert(url_key, long_url);
                Ok(Response::with((status::Created, Header(headers::Location(short_url)))))
            }
            Err(e) => Ok(Response::with((status::BadRequest, e))),
        }
    }

    fn handle_get_request(&self, req: &mut Request) -> IronResult<Response> {
        let request_path = &req.url.path().join("/");
        match self.shortened_urls.read().unwrap().get(request_path) {
            Some(url) => Ok(Response::with((status::Found, Redirect(url.clone())))),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}

impl Handler for UrlShortenerHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match &req.method {
            &Method::Post => self.handle_post_request(req),
            &Method::Get | &Method::Head => self.handle_get_request(req),
            _ => Ok(Response::with(status::NotFound)),
        }
    }
}
