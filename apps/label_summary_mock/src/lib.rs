mod error;
mod config;
mod issue;
mod gitlab_api;
mod label_list;
mod slack_message;
mod slack_notifier;
mod lsh_client;
mod lsh_response;

use config::Config;
use gitlab_api::GitlabApi;
use label_list::LabelList;
use slack_notifier::SlackNotifier;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use lsh_client::MockLshClient as LshClient;
    } else {
        pub use lsh_client::LshClient;
    }
}

pub fn run(lsh_client: &LshClient) -> Result<(), error::Error> {
    // 設定ファイル読み込み
    let config = Config::new("./config.yml")?;

    // GitLabからプロジェクトのIssue一覧を取得
    let gitlab_api = GitlabApi::new(lsh_client, config.gitlab());
    let issue_list = gitlab_api.issue_all()?;

    // Issueの一覧からラベルの一覧を生成、それぞれの件数も集計する
    let label_list = LabelList::new(&issue_list);
    // 通知内容を生成
    let content = label_list.format();

    // Slackへ通知
    let notifier = SlackNotifier::new(lsh_client, config.slack());
    let resp = notifier.execute(&content)?;
    println!("{}", resp);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderMap;
    use crate::issue::Issue;
    use crate::lsh_response::LshResponse;

    #[test]
    fn it_works() {
        let mut lsh_client = LshClient::default();

        lsh_client.expect_request_get()
            .returning(|_, _| {
                let resp = LshResponse::new(HeaderMap::new(), Vec::<Issue>::new());
                Ok(resp)
            });

        lsh_client.expect_request_post()
            .returning(|_, _| Ok(String::new()));

        assert_eq!(run(&lsh_client).unwrap(), ());
    }
}
