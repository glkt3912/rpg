# rpg - Rust Password Generator

Rustで書かれた、セキュアなコマンドラインパスワード生成ツールです。

## 機能

- `rand::thread_rng()`を使用した暗号学的に安全なランダムパスワード生成
- カスタマイズ可能なパスワード長（1-1024文字）
- 柔軟な文字種選択：
  - 大文字（A-Z）
  - 小文字（a-z）
  - 数字（0-9）
  - 記号（!@#$%^&*()_+-=[]{}|;:,.<>?）
- シンプルで直感的なCLIインターフェース
- 高速・軽量

## インストール

### ソースからビルド

```bash
git clone <repository-url>
cd rpg
cargo install --path .
```

または、単純にビルドして実行：

```bash
cargo build --release
./target/release/rpg
```

## 使い方

### 基本的な使い方

デフォルト設定でパスワードを生成（16文字、全文字種）：

```bash
rpg
```

出力例：
```
aB3$xY9@pQ2#mN5!
```

### パスワード長を指定

```bash
rpg -l 20
rpg --length 32
```

### 文字種を除外

```bash
# 大文字なし
rpg --no-uppercase

# 記号なし
rpg --no-symbols

# 小文字と数字のみ
rpg --no-uppercase --no-symbols

# 長さと組み合わせ
rpg --no-symbols -l 24
```

### 使用例

```bash
# 記号なしの8文字パスワード
$ rpg -l 8 --no-symbols
Kx9mP2Qn

# 小文字と数字のみの32文字パスワード
$ rpg -l 32 --no-uppercase --no-symbols
a8f3k9m2x7n5q1p4r6t8y0z3c5v7b9

# 全文字種を含む16文字パスワード（デフォルト）
$ rpg
aB3$xY9@pQ2#mN5!
```

### ヘルプを表示

```bash
rpg --help
```

## 開発

### テストを実行

```bash
cargo test
```

### コードをフォーマット

```bash
cargo fmt
```

### Linterを実行

```bash
cargo clippy
```

### リリースビルドを作成

```bash
cargo build --release
```

## プロジェクト構成

```
rpg/
├── Cargo.toml          # プロジェクト設定
├── README.md           # このファイル
└── src/
    ├── main.rs         # CLIエントリーポイント
    ├── lib.rs          # ライブラリルート
    ├── error.rs        # エラー型定義
    ├── config.rs       # 設定とCLI引数
    ├── charset.rs      # 文字セット定義
    └── generator.rs    # パスワード生成ロジック
```

## セキュリティ

- `rand::thread_rng()`による暗号学的に安全な乱数生成
- `SliceRandom::choose()`による均一分布の実装
- メモリ安全性のため最大パスワード長を1024文字に制限

## 今後の機能拡張

- クリップボード対応
- パスフレーズ生成（diceware方式）
- パスワード強度評価
- パターンベース生成
- 設定ファイル対応

## ライセンス

MIT
