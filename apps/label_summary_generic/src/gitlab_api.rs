use crate::Error;
use crate::Issue;
use crate::config::GitlabConfig;
use crate::request_sender::RequestSender;

pub struct GitlabApi<'a, T: RequestSender> {
    lsh_client: &'a T,
    config: &'a GitlabConfig,
}

impl<'a, T: RequestSender> GitlabApi<'a, T> {
    pub fn new(lsh_client: &'a T, config: &'a GitlabConfig) -> GitlabApi<'a, T> {
        GitlabApi {
            lsh_client,
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
            let resp = self.lsh_client.request_get(&url, self.config.token())?;
 
            let headers = resp.headers().clone();
            let mut list = resp.list();
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
