use reqwest::blocking::Client;
use crate::error::Error;
use crate::config::SlackConfig;
use crate::slack_message::SlackMessage;

pub struct SlackNotifier<'a> {
    client: &'a Client,
    config: &'a SlackConfig,
}

impl<'a> SlackNotifier<'a> {
    pub fn new(client: &'a Client, config: &'a SlackConfig) -> SlackNotifier<'a> {
        SlackNotifier {
            client,
            config,
        }
    }

    pub fn execute(&self, content: &str) -> Result<String, Error> {
        let message = SlackMessage::new(content, self.config);
        let json = message.to_json()?;

        let resp = self.client.post("https://slack.com/api/chat.postMessage")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.config.token()))
            .body(json)
            .send()?
            .text()?;
        Ok(resp)
    }
}
