use crate::error::Error;
use crate::config::SlackConfig;
use crate::slack_message::SlackMessage;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use crate::lsh_client::MockLshClient as LshClient;
    } else {
        use crate::lsh_client::LshClient;
    }
}

pub struct SlackNotifier<'a> {
    lsh_client: &'a LshClient,
    config: &'a SlackConfig,
}

impl<'a> SlackNotifier<'a> {
    pub fn new(lsh_client: &'a LshClient, config: &'a SlackConfig) -> SlackNotifier<'a> {
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
