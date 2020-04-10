use http;

// use http::header::{HeaderMap, HeaderName, HeaderValue};
// use http::status::StatusCode;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Request<T> {
    /// The request method, such as `GET`.
    pub method: String,
    /// The request url with host, such as `https://get.zoe.im/about-us`.
    pub url: String, // TODO: deserialize to uri?
    /// The request version, such as `HTTP/1.1`.
    pub version: Option<u8>,
    /// The request headers.
    pub headers: HashMap<String, String>,
    // TODO: cf: CloudFlare
    /// The request body
    pub body: Option<T>,

    /// auto parse from url
    #[serde(skip_serializing, skip_deserializing)]
    _uri: http::Uri,
}

impl<T> Request<T> {
    pub fn uri(&mut self) -> &http::Uri {
        if self._uri.eq(&http::Uri::default()) {
            // parse from url and assign to self._uri
            self._uri = self.url.as_str().parse::<http::Uri>().unwrap();
        }

        return &self._uri;
    }
}

// build the javascript response
#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    /// The response version, such as `HTTP/1.1`.
    // pub version: Option<u8>,
    /// The response status, such as `200`.
    pub status: u16,
    /// The response reason-phrase, such as `OK`.
    // pub reason: Option<&'buf str>,
    /// The response headers.
    pub headers: HashMap<String, String>,
    /// The response body
    pub body: T,
}

impl<T> Response<T> {
    pub fn new(body: T) -> Self {
        Response {
            status: 200,
            headers: HashMap::new(),
            body: body,
        }
    }

    pub fn header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
}
