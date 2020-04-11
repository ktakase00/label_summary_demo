use crate::error::Error;
use crate::lsh_response::LshResponse;

pub trait RequestSender {
    fn request_get(&self, url: &str, token: &str) -> Result<LshResponse, Error>;
    fn request_post(&self, json: String, token: &str) -> Result<String, Error>;
}
