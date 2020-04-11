use reqwest::blocking::Client;
use crate::error::Error;
use crate::issue::Issue;
use crate::config::GitlabConfig;

pub struct GitlabApi<'a> {
    client: &'a Client,
    config: &'a GitlabConfig,
}

impl<'a> GitlabApi<'a> {
    pub fn new(client: &'a Client, config: &'a GitlabConfig) -> GitlabApi<'a> {
        GitlabApi {
            client,
            config
        }
    }

    pub fn issue_all(&self) -> Result<Vec<Issue>, Error> {
        let mut page = 1;
        let mut total_page = -1;
        let per_page = 100;
        let base_url = format!("https://{}/api/v4/projects/{}/issues?per_page={}",
            self.config.host_name(),
            self.config.project_id(),
            per_page
        );
        let mut issue_list = Vec::<Issue>::new();

        while total_page < 0 || page <= total_page {
            let url = format!("{}&page={}", base_url, page);
            let resp = self.client.get(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", self.config.token()))
                .send()?
            ;
 
            let headers = resp.headers().clone();
            let mut list = resp.json::<Vec<Issue>>()?;
            issue_list.append(&mut list);
 
            total_page = match headers.get("x-total-pages".to_string()) {
                Some(val) => val.to_str()?.parse()?,
                None => { break; },
            };
            page += 1;
        }
        Ok(issue_list)
    }
}
