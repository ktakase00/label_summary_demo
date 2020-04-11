use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use crate::error::Error;
use crate::issue::Issue;
use crate::request_sender::RequestSender;
use crate::lsh_response::LshResponse;

pub struct MockLshClient {
    _client: Client,
}

impl MockLshClient {
    pub fn _new() -> MockLshClient {
        MockLshClient {
            _client: Client::new(),
        }
    }
}

impl<'a> RequestSender for MockLshClient {
    fn request_get(&self, _url: &str, _token: &str) -> Result<LshResponse, Error> {
        let headers = HeaderMap::new();
        let list = Vec::<Issue>::new();
        Ok(LshResponse::new(headers, list))
    }

    fn request_post(&self, _json: String, _token: &str) -> Result<String, Error> {
        let resp = String::new();
        Ok(resp)
    }
}
