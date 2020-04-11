use reqwest::blocking::Client;
use crate::error::Error;
use crate::issue::Issue;
use crate::request_sender::RequestSender;
use crate::lsh_response::LshResponse;

pub struct LshClient {
    client: Client,
}

impl LshClient {
    pub fn new() -> LshClient {
        LshClient {
            client: Client::new(),
        }
    }
}

impl RequestSender for LshClient {
    fn request_get(&self, url: &str, token: &str) -> Result<LshResponse, Error> {
        let resp = self.client.get(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .send()?
        ;
        let headers = resp.headers().clone();
        let list = resp.json::<Vec<Issue>>()?;
        Ok(LshResponse::new(headers, list))
    }

    fn request_post(&self, json: String, token: &str) -> Result<String, Error> {
        let resp = self.client.post("https://slack.com/api/chat.postMessage")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .body(json)
            .send()?
            .text()?;
         Ok(resp)
    }
}
