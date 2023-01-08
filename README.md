# 英単語テスト作成プログラム

重要度により重みづけされた分布に従いTOML形式で書かれたファイルから抽出し、
英単語テストをLaTeX形式で出力する。
問題ファイルは入力ファイルと同じディレクトリに接頭辞exam\_of\_が付加されて出力され、
解答ファイルは接頭辞answer\_of\_が付加されて出力される。
余白調整のために[geometry](https://ctan.org/pkg/geometry)パッケージを利用している。

## インストール方法

コンパイルには`cargo`コマンドが必要なので、ない場合は[このページ](https://www.rust-lang.org/ja/tools/install)を参照すること。

bash が動く環境であれば [install.sh](install.sh) を実行すればインストールされる。
引数でインストール先ディレクトリを指定することも可能（引数がない場合のインストール先は ~/.cargo/bin）。
bash がない環境では[このサイト](https://doc.rust-lang.org/cargo/commands/cargo-install.html)を参考にすること。

## 使い方

```sh
english_vocabulary_test 0.1.0

英単語テスト作成

USAGE:
    english_vocabulary_test <CARD_FILE> <NUM_PROBLEM>

ARGS:
    <CARD_FILE>
    <NUM_PROBLEM>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## 補助スクリプト

LaTeXファイルを作成した後に[script/make\_exam\_pdf.sh](script/make_exam_pdf.sh)にカードファイルを引数として渡して実行することで
PDFファイルが作成される（内部でlatexmkを使用する）。

## カードファイルの書き方

下記のように`[[card]]`に続けて情報を記入していく。
記入例は[example\_of\_card\_list/example\_of\_card\_list.toml](example_of_card_list/example_of_card_list.toml)にある。


```toml
# 1つ目の単語情報
[[card]]
id = 1 # 単語番号: 単語帳の単語番号を書くと良い
page = 1 # ページ: 単語帳の該当ページを書くと良い
priority = 0 # 重要度: これにより抽出の際に重みづけがなされる。値が大きいほどより抽出されやすくなる。
english = "English" # 英語: これが解答となる。
phrase = true # 節かどうか: 任意
sentence = "文章" # 任意
noun = ["名詞1", "名詞2"] # 任意、リスト
adjective = ["形容詞1", "形容詞2"] # 任意、リスト
verb = ["動詞1", "動詞2"] # 任意、リスト
adverb = ["副詞1", "副詞2"] # 任意、リスト
preposition = ["前置詞1", "前置詞2"] # 任意、リスト

# 2つ目の単語情報
[[card]]
id = 2
page = 1
priority = 0
english = "English"
phrase = true
sentence = "文章"
noun = ["名詞1", "名詞2"]
adjective = ["形容詞1", "形容詞2"]
verb = ["動詞1", "動詞2"]
adverb = ["副詞1", "副詞2"]
preposition = ["前置詞1", "前置詞2"]

# 同じ要領で3つ目以降の単語情報を記入していく。
```

## 余談

cron とlpr コマンドを利用すれば毎日単語テストがプリンタから出てくる環境が構築できる。

## License
Copyright (c) 2023 Yuichi Ishida
Released under the MIT license
[https://opensource.org/licenses/mit-license.php](https://opensource.org/licenses/mit-license.php)
