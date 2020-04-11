use std::fs::File;
use serde::Deserialize;
use serde_yaml;
use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    gitlab: GitlabConfig,
    slack: SlackConfig,
}

#[derive(Debug, Deserialize)]
pub struct GitlabConfig {
    token: String,
    host_name: String,
    project_id: usize,
}

#[derive(Debug, Deserialize)]
pub struct SlackConfig {
    token: String,
    channel: String,
    username: String,
}

impl Config {
    pub fn new(file_path: &str) -> Result<Config, Error> {
        let file = File::open(file_path)?;
        let config: Config = serde_yaml::from_reader(file)?;
        Ok(config)
    }

    pub fn gitlab(&self) -> &GitlabConfig {
        &self.gitlab
    }

    pub fn slack(&self) -> &SlackConfig {
        &self.slack
    }
}

impl GitlabConfig {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn host_name(&self) -> &str {
        &self.host_name
    }

    pub fn project_id(&self) -> usize {
        self.project_id
    }
}

impl SlackConfig {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn channel(&self) -> &str {
        &self.channel
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}
