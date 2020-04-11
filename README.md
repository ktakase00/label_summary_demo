# Rustで構造体をモックする

## 処理内容

GitLabの指定のプロジェクトから、Issuの一覧を取得し、Issueに設置されているラベルの種類とそれぞれの件数を集計する。集計結果をSlackに通知する。

## 準備

`apps/label_summary/config.yml` を作成し、必要なパラメータを設定する。 `apps/label_summary/config.yml.sample` 参照。

```
gitlab:
  token: 'aaa'
  host_name: 'gitlab.example.com'
  project_id: 1
slack:
  token: 'bbb'
  channel: 'ccc'
  username: 'MyName'
```

## 実行

```sh
docker-compose run --rm dev bash
cd /mnt/apps/label_summary
cargo build
cargo run
cargo test
```

## プロジェクト一覧

|プロジェクト名|内容|
|---|---|
|label_summary|モックなし|
|label_summary_generic|ジェネリクスでオブジェクトを切り替える|
|label_summary_mock|[mockall](https://github.com/asomers/mockall)クレートを使用|
