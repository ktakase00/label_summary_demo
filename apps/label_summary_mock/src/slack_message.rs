use serde::Serialize;
use crate::error::Error;
use crate::config::SlackConfig;

#[derive(Debug, Serialize)]
pub struct SlackMessage<'a> {
    channel: &'a str,
    username: &'a str,
    text: &'a str,
}

impl<'a> SlackMessage<'a> {
    pub fn new(content: &'a str, config: &'a SlackConfig) -> SlackMessage<'a> {
        SlackMessage {
            channel: config.channel(),
            username: config.username(),
            text: content,
        }
    }

    pub fn to_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }
}
