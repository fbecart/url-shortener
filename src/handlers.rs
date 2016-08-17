use iron::headers;
use iron::prelude::*;
use iron::method::Method;
use iron::modifiers::{RedirectRaw, Header};
use iron::{Handler, Url, status};

use rand;
use rand::Rng;

use std::collections::HashMap;
use std::sync::RwLock;

use urlencoded::UrlEncodedBody;

pub struct UrlShortenerHandler {
    short_url_prefix: String,
    shortened_urls: RwLock<HashMap<String, String>>,
}

impl UrlShortenerHandler {
    pub fn new(short_url_prefix: String) -> Self {
        UrlShortenerHandler {
            short_url_prefix: short_url_prefix,
            shortened_urls: RwLock::new(HashMap::new()),
        }
    }

    fn extract_url(req: &mut Request) -> Result<String, &'static str> {
        req.get_ref::<UrlEncodedBody>()
            .map_err(|_| "URL encoded body missing")
            .and_then(|data| match data.get("url") {
                Some(urls) => Ok(urls.first().unwrap()),
                None => Err("Parameter 'url' missing from the URL encoded body"),
            })
            .and_then(|url| {
                Url::parse(url).map(|_| url.to_owned()).map_err(|_| "Parameter 'url' is invalid")
            })
    }

    fn gen_unused_random_key(shortened_urls: &HashMap<String, String>) -> String {
        let mut url_key: Option<String> = None;
        while url_key.is_none() {
            let random_key: String = rand::thread_rng().gen_ascii_chars().take(7).collect();
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

                let short_url = format!("{}{}", self.short_url_prefix, url_key);
                shortened_urls.insert(url_key, long_url);
                Ok(Response::with((status::Created, Header(headers::Location(short_url)))))
            }
            Err(e) => Ok(Response::with((status::BadRequest, e))),
        }
    }

    fn handle_get_request(&self, req: &mut Request) -> IronResult<Response> {
        let request_path = &req.url.path().join("/");
        match self.shortened_urls.read().unwrap().get(request_path) {
            Some(url) => Ok(Response::with((status::Found, RedirectRaw(url.to_owned())))),
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

#[cfg(test)]
mod tests {
    use hyper::header::Location;

    use iron::headers::ContentType;
    use iron::{Headers, status};

    use iron_test::{request, response};

    use super::UrlShortenerHandler;

    #[test]
    fn short_url_not_found() {
        let handler = UrlShortenerHandler::new("http://localhost:3000/".to_string());

        let response = request::get("http://localhost:3000/hello", Headers::new(), &handler)
            .unwrap();
        assert_eq!(response.status.unwrap(), status::NotFound);
    }

    #[test]
    fn short_url_found() {
        let handler = UrlShortenerHandler::new("http://localhost:3000/".to_string());

        let mut request_headers = Headers::new();
        request_headers.set(ContentType::form_url_encoded());
        let response = request::post("http://localhost:3000",
                                     request_headers,
                                     "url=https://www.helloclue.com",
                                     &handler)
            .unwrap();
        let short_url = response.headers.get::<Location>().unwrap();
        let response = request::get(short_url, Headers::new(), &handler).unwrap();
        assert_eq!(response.status.unwrap(), status::Found);
        assert_eq!(response.headers.get::<Location>().unwrap().0,
                   "https://www.helloclue.com");
    }

    #[test]
    fn post_wrong_contenttype() {
        let handler = UrlShortenerHandler::new("http://localhost:3000/".to_string());

        let response = request::post("http://localhost:3000", Headers::new(), "", &handler)
            .unwrap();
        assert_eq!(response.status.unwrap(), status::BadRequest);
        assert_eq!(response::extract_body_to_string(response),
                   "URL encoded body missing");
    }

    #[test]
    fn post_missing_url() {
        let handler = UrlShortenerHandler::new("http://localhost:3000/".to_string());

        let mut request_headers = Headers::new();
        request_headers.set(ContentType::form_url_encoded());
        let response = request::post("http://localhost:3000", request_headers, "a=b", &handler)
            .unwrap();
        assert_eq!(response.status.unwrap(), status::BadRequest);
        assert_eq!(response::extract_body_to_string(response),
                   "Parameter 'url' missing from the URL encoded body");
    }

    #[test]
    fn post_invalid_url() {
        let handler = UrlShortenerHandler::new("http://localhost:3000/".to_string());

        let mut request_headers = Headers::new();
        request_headers.set(ContentType::form_url_encoded());
        let response = request::post("http://localhost:3000",
                                     request_headers,
                                     "url=invalid/url",
                                     &handler)
            .unwrap();
        assert_eq!(response.status.unwrap(), status::BadRequest);
        assert_eq!(response::extract_body_to_string(response),
                   "Parameter 'url' is invalid");
    }
}
