mod error;
mod config;
mod issue;
mod gitlab_api;
mod label_list;
mod slack_message;
mod slack_notifier;

use reqwest::blocking::Client;
use config::Config;
use gitlab_api::GitlabApi;
use label_list::LabelList;
use slack_notifier::SlackNotifier;

pub fn run(client: &Client) -> Result<(), error::Error> {
    // 設定ファイル読み込み
    let config = Config::new("./config.yml")?;

    // GitLabからプロジェクトのIssue一覧を取得
    let gitlab_api = GitlabApi::new(client, config.gitlab());
    let issue_list = gitlab_api.issue_all()?;

    // Issueの一覧からラベルの一覧を生成、それぞれの件数も集計する
    let label_list = LabelList::new(&issue_list);
    // 通知内容を生成
    let content = label_list.format();

    // Slackへ通知
    let notifier = SlackNotifier::new(client, config.slack());
    let resp = notifier.execute(&content)?;
    println!("{}", resp);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let client = Client::new();
        assert_eq!(run(&client).unwrap(), ());
    }
}
