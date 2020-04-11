mod error;
mod config;
mod issue;
mod gitlab_api;
mod label_list;
mod slack_message;
mod slack_notifier;
mod request_sender;
mod lsh_client;
mod lsh_response;
mod mock_lsh_client;

use error::Error;
use config::Config;
use issue::Issue;
use gitlab_api::GitlabApi;
use label_list::LabelList;
use slack_notifier::SlackNotifier;
use request_sender::RequestSender;
pub use lsh_client::LshClient;

pub fn run<T: RequestSender>(lsh_client: &T) -> Result<(), error::Error> {
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
    use mock_lsh_client::MockLshClient;

    #[test]
    fn it_works() {
        let lsh_client = MockLshClient::_new();
        assert_eq!(run(&lsh_client).unwrap(), ());
    }
}
