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

この環境での `oj` の役割は、サンプル取得とサンプルテストだけです。
AtCoder への提出はブラウザで行うため、`oj login` と `oj submit` は使いません。

ブラウザで AtCoder にログインしておいてください。

## `oj` の責務

- `scripts/new-problem`: `oj d <url>` でサンプルを取得する
- `at-test`: `oj t -c "cargo run --quiet"` でサンプルテストを実行する
- `at-submit`: `oj` は使わず、ブラウザ提出に必要な情報を表示する

## 1問の流れ

問題ディレクトリ、Cargo プロジェクト、サンプルを作成します。

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

`at-submit` が表示した URL をブラウザで開き、対象問題と言語 Rust を選んで `src/main.rs` を提出します。

リポジトリルートから実行する場合:

```bash
scripts/test abs abc086_a
scripts/submit abs abc086_a   # ブラウザ提出用の情報を表示
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
scripts/
  at-test
  at-submit
  lib
  new-problem
  test
  submit
```
