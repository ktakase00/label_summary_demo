use crate::error::Error;
use crate::config::SlackConfig;
use crate::slack_message::SlackMessage;
use crate::request_sender::RequestSender;

pub struct SlackNotifier<'a, T: RequestSender> {
    lsh_client: &'a T,
    config: &'a SlackConfig,
}

impl<'a, T: RequestSender> SlackNotifier<'a, T> {
    pub fn new(lsh_client: &'a T, config: &'a SlackConfig) -> SlackNotifier<'a, T> {
        SlackNotifier {
            lsh_client,
            config,
        }
    }

    pub fn execute(&self, content: &str) -> Result<String, Error> {
        let message = SlackMessage::new(content, self.config);
        let json = message.to_json()?;

        let resp = self.lsh_client.request_post(json, self.config.token())?;
        Ok(resp)
    }
}
