use reqwest::header::HeaderMap;
use crate::issue::Issue;

pub struct LshResponse {
    headers: HeaderMap,
    list: Vec<Issue>,
}

impl LshResponse {
    pub fn new(headers: HeaderMap, list: Vec<Issue>) -> LshResponse {
        LshResponse {
            headers,
            list,
        }
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn list(self) -> Vec<Issue> {
        self.list
    }
}
