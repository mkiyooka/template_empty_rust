# Rust Project Template

Rust開発のためのテンプレートプロジェクトです。

## プロジェクト構成

- **パッケージマネージャ**: Cargo
- **Rust Edition**: 2024
- **テストフレームワーク**: 標準テストフレームワーク
- **ベンチマーク**: Criterion

## セットアップ

```bash
# 依存関係のビルド
cargo build

# テスト実行
cargo test

# リリースビルド
cargo build --release
```

## 実行

```bash
# メインアプリケーション
cargo run

# リリースモード実行
cargo run --release

# 例の実行
cargo run --example basic_example
```

## 開発ツール

```bash
# コードフォーマット
cargo fmt

# フォーマット確認
cargo fmt --check

# 静的解析
cargo clippy

# Clippy自動修正
cargo clippy --fix

# ドキュメント生成
cargo doc --open

# ベンチマーク実行
cargo bench

# ビルド成果物のクリーンアップ
cargo clean
```

## ディレクトリ構成

- `src/` - ソースコード
    - `main.rs` - バイナリクレートのエントリポイント
    - `lib.rs` - ライブラリクレート
- `tests/` - 統合テスト
    - `integration_test.rs` - サンプル統合テスト
- `examples/` - 使用例
    - `basic_example.rs` - サンプル使用例
- `benches/` - ベンチマーク
    - `benchmarks.rs` - サンプルベンチマーク
- `.vscode/` - VSCode設定
    - `tasks.json` - cargoタスク設定
    - `launch.json` - デバッグ設定
    - `settings.json` - エディタ設定
    - `extensions.json` - 推奨拡張機能
- `.zed/` - Zed設定
    - `tasks.json` - cargoタスク設定
    - `debug.json` - デバッグ設定
    - `settings.json` - エディタ設定

## VSCode / Zed サポート

このテンプレートには、VSCodeとZed両方の設定が含まれています。

### VSCode

- `Cmd+Shift+P` → `Tasks: Run Task` でcargoタスクを実行
- `F5` キーでデバッグ実行

### Zed

- タスクパネルからcargoタスクを実行
- デバッグパネルからデバッグ実行

## 推奨拡張機能

### VSCode

- rust-lang.rust-analyzer
- vadimcn.vscode-lldb
- serayuzgur.crates
- tamasfe.even-better-toml

### Zed

- CodeLLDBデバッガが組み込みサポート
