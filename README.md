# なにか

指定したキーワードでarxivを検索し、好きなslackのチャンネルにポストする

# どうやって使うか

1. 好きなslackのworkspaceでポスト用のチャンネルを作成する
1. slackの[incoming-webhook](https://api.slack.com/messaging/webhooks) で該当チャンネルへのポストを行うwebhookを作成
1. 上記のwebhookのURLを記録しておく
1. このリポジトリをcloneする
1. 下記に従い`setting.toml`を編集する
1. 下記に従いコードを実行する

## setting.tomlの編集

```toml
[[arxiv]]
categories = ["cs.CV", "stat.ML"]  # 検索するカテゴリ required
slack  = "XXXXXXXXXXXXXXXX"  # ポストするslackのwebhook URL required
filter_by_main_category = true  # arxiv apiはサブカテゴリが一致するものも取得するが、上記で指定したカテゴリがメインカテゴリとして登録されているものだけにフィルタリングする required
search_title_words = ["Face", "Facial", "face", "facial"]  # タイトルに含まれていてほしい文字 optional
exclude_title_words = ["Surface"]  # タイトルには含まれてほしくない文字 optional
search_abstract_words = ["face", "facial"]  # abstに含まれていてほしい文字 optional
exclude_abstract_words = ["surface"]  # abstに含まれてほしくない文字 optional
star_keywords = ["CVPR", "ICCV", "ECCV", "NIPS", "NeurIPS", "AAAI", "accept"]  # ハイライト対象のワード optional

[[arxiv]]  # いくつでも設定可能
categories = ...
```
上記例だと「cs.CVもしくはstat.MLの中で、タイトルにFace, Facial, face, facialのどれかが含まれていて、かつSurfaceは含まれておらず、かつアブストラクトにfaceもしくはfacialが含まれていて、かつsurfaceは含まれていない論文」を検索し、メインカテゴリがcs.CV, stat.MLのもののみを取得する

その後、XXXXXXXXXXXXXXXXというwebhook urlをとおしてslackに送信する。ただし、CVPR, ICCV, ... という単語が論文のコメントにあれば:star:をつける


ということを意味します

この`[[arxiv]]`は複数設定することが可能です

## コード実行

### Dockerを使う場合

```shell script
git clone git@github.com:Nkriskeeic/arxiv-bot.git
cd arxiv-bot
docker-compose up
# arxiv-bot_app_1 exited with code 0 が出るまで待つ

# 以降はこれだけ実行すれば動きます
docker-compose run --rm app ./arxiv-bot LastUpdatedBy -m 100 --start 0 --save --slack --send
```

### Dockerを使わない場合

#### Rustのインストール

[公式](https://www.rust-lang.org/ja/tools/install) のコマンドを実行してください

#### 実行

```shell script
git clone git@github.com:Nkriskeeic/arxiv-bot.git
cd arxiv-bot
mkdir database
cargo install diesel_cli --no-default-features --features sqlite
diesel migration run

cargo build --release
./target/release/arxiv-bot LastUpdatedBy -m 100 --start 0 --save --slack --send
```

# コマンド補足

```shell script
SORT_FLAG=[Relevance|LastUpdatedBy|SubmittedDate]  # この中のどれかの基準でarxivから取得。左から関連順/更新順/最初のアップロード時刻順
MAX_RESULTS=100  # 何件取得するか
START=0  # ページネーション。何件すっ飛ばして取得するか

./target/debug/arxiv-bot SORT_FLAG
    --max-results[-m] MAX_RESULTS
    --start[-s] START
    [--save]  # DBに保存する。またslackに送ったことがない論文をキューに保存する
    [--slack]  # キューから論文を取得する
    [--send]  # キューから論文を取得して送信する（--slack --sendで動く。--send単体は動かない）
```
