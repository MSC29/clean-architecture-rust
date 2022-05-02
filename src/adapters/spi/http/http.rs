use reqwest;

pub struct HttpConnection {}

impl HttpConnection {

    pub fn client(&self) -> reqwest::Client{
        reqwest::Client::new()
    }
}