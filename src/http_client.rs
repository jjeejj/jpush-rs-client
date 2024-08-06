use reqwest::{Client, Response, Error,Method};

#[derive(Debug)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }
    /// send a GET request to the given url
    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        self.client.get(url).send().await
    }
    /// send a POST request to the given url
    pub async fn post<T>(&self, url: &str, body: T) -> Result<Response, Error> {
        self.client.post(url).send().await
    }
}