# text-stats

テキストファイルの行数、単語数、文字数をカウントするコマンドラインツール

## 機能

- テキストファイルの行数、単語数、文字数をカウント
- 複数ファイルの同時処理
- 標準入力からの読み込み
- 出力形式のカスタマイズ
- JSON形式での出力

## インストール

```bash
# リポジトリのクローン
git clone https://github.com/yourusername/text-stats.git
cd text-stats

# ビルド
cargo build --release

# インストール（オプション）
cargo install --path .
```

## 使用方法

```bash
# 基本的な使用
text-stats ファイル名

# 複数ファイルの処理
text-stats ファイル1 ファイル2

# 特定の統計のみを表示
text-stats --lines ファイル名    # 行数のみ
text-stats --words ファイル名    # 単語数のみ
text-stats --chars ファイル名    # 文字数のみ

# JSON形式で出力
text-stats --json ファイル名

# 標準入力から読み込み
cat ファイル名 | text-stats -
```

## オプション

- `--lines`, `-l`: 行数のみを表示
- `--words`, `-w`: 単語数のみを表示
- `--chars`, `-c`: 文字数のみを表示
- `--json`, `-j`: JSON形式で出力

## 技術スタック

- Rust
- clap (コマンドライン引数パース)
- serde (JSONシリアライズ)

## 開発環境のセットアップ

```bash
# 依存関係のインストール
cargo build

# テストの実行
cargo test

# リントの実行
cargo clippy
```

## ライセンス

MIT License

## 作者

natsuoooooo