# AtCoder Rust Practice

AtCoder の問題を Rust で解くための練習環境です。

## セットアップ

Rust stable を使います。

```bash
rustup default stable
```

Nix dev shell に入ります。

```bash
nix --extra-experimental-features "nix-command flakes" develop
```

AtCoder への提出には `oj` のログイン状態が必要です。確認:

```bash
oj login --check https://atcoder.jp/
```

`oj login` が通らない場合は、ブラウザで AtCoder にログインしてから `REVEL_SESSION` Cookie を次のファイルへ反映します。

```text
/Users/itoushinji/Library/Application Support/online-judge-tools/cookie.jar
```

## 1問の流れ

問題ディレクトリ、Cargo プロジェクト、サンプル、復習ログを作成します。

```bash
scripts/new-problem abs abc086_a https://atcoder.jp/contests/abs/tasks/abc086_a
```

実装します。

```bash
cd practice/atcoder/abs/abc086_a
$EDITOR src/main.rs
```

サンプルテスト:

```bash
at-test
```

提出:

```bash
at-submit
```

リポジトリルートから実行する場合:

```bash
scripts/test abs abc086_a
scripts/submit abs abc086_a
```

## ディレクトリ構成

```text
practice/
  atcoder/
    abs/
      abc086_a/
        Cargo.toml
        src/
          main.rs
        .atcoder-task-url
        test/
          sample-1.in
          sample-1.out
notes/
  atcoder/
    abs/
      abc086_a.md
scripts/
  at-test
  at-submit
  lib
  new-problem
  test
  submit
```

## 復習ログ

`scripts/new-problem` は `notes/atcoder/<contest>/<task-id>.md` に次の雛形を作ります。

```md
- 日付:
- 問題:
- URL:
- テーマ:
- 解法概要:
- 計算量:
- 詰まった点:
- Rustで詰まった点:
- 追加すべきテストケース:
- 次に解く類題:
```
