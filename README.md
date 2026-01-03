# rpg - Rust Password Generator

Rustで書かれた、セキュアなコマンドラインパスワード生成ツールです。

## 機能

### パスワード生成
- `rand::thread_rng()`を使用した暗号学的に安全なランダムパスワード生成
- カスタマイズ可能なパスワード長（1-1024文字）
- 柔軟な文字種選択：
  - 大文字（A-Z）
  - 小文字（a-z）
  - 数字（0-9）
  - 記号（!@#$%^&*()_+-=[]{}|;:,.<>?）

### パスフレーズ生成
- 複数の単語を組み合わせた覚えやすいパスフレーズ生成
- カスタマイズ可能な単語数（1-16語）
- ハイフン区切りで読みやすい形式

### 追加機能
- クリップボードへの自動コピー
- 複数パスワード/パスフレーズの一括生成
- カラー出力（文字種別に色分け、TTY自動検出）
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

出力例（カラー表示）：
```
aB3$xY9@pQ2#mN5!
```

### パスワード生成

#### パスワード長を指定

```bash
rpg -l 20
rpg --length 32
```

#### 文字種を除外

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

#### クリップボードにコピー

```bash
# パスワードを生成してクリップボードにコピー
rpg -c
rpg --copy

# 長いパスワードを生成してコピー
rpg -l 32 -c
```

#### 複数のパスワードを生成

```bash
# 3つのパスワードを生成
rpg -n 3

# 5つの記号なしパスワードを生成
rpg -n 5 --no-symbols

# 複数生成してクリップボードにコピー（最後の1つがコピーされます）
rpg -n 3 -c
```

### パスフレーズ生成

#### 基本的なパスフレーズ

```bash
# デフォルト4語のパスフレーズを生成
rpg --passphrase
```

出力例：
```
able-acid-aged-also
```

#### 単語数を指定

```bash
# 6語のパスフレーズを生成
rpg --passphrase --words 6

# 短いパスフレーズ（3語）
rpg --passphrase --words 3
```

#### パスフレーズとその他オプションの組み合わせ

```bash
# パスフレーズをクリップボードにコピー
rpg --passphrase -c

# 複数のパスフレーズを生成
rpg --passphrase -n 5

# 8語のパスフレーズを3つ生成
rpg --passphrase --words 8 -n 3
```

### カラー出力

ターミナル（TTY）が検出された場合、自動的にカラー出力が有効になります：
- **大文字**: 青色
- **小文字**: 緑色
- **数字**: 黄色
- **記号**: 赤色

パスフレーズの場合、単語ごとに異なる色で表示されます。

```bash
# カラー出力を無効化
rpg --no-color

# パイプやリダイレクト時は自動的に無効化されます
rpg > password.txt
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

# 覚えやすいパスフレーズ
$ rpg --passphrase
able-acid-aged-also

# 安全性の高い長いパスフレーズ
$ rpg --passphrase --words 8
able-acid-aged-also-area-army-away-baby

# パスワードを5つ生成して確認
$ rpg -n 5 -l 12
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
    ├── generator.rs    # パスワード生成ロジック
    ├── clipboard.rs    # クリップボード操作
    ├── colorize.rs     # カラー出力処理
    ├── passphrase.rs   # パスフレーズ生成ロジック
    └── wordlist.rs     # パスフレーズ用単語リスト
```

## セキュリティ

### パスワード生成
- `rand::thread_rng()`による暗号学的に安全な乱数生成
- `SliceRandom::choose()`による均一分布の実装
- メモリ安全性のため最大パスワード長を1024文字に制限

### パスフレーズ生成
- 256語の単語リストからランダムに選択（将来的に2048語に拡張予定）
- 単語の重複なし選択で高いエントロピーを確保
- 最大16語まで生成可能

### クリップボード
- プラットフォーム非依存のクリップボード操作（arboard使用）
- 複数生成時は最後の1つのみをコピー

## 今後の機能拡張

- 単語リストの拡張（256語 → 2048語）
- パスワード強度評価
- パターンベース生成
- 設定ファイル対応
- カスタム単語リスト対応

## ライセンス

MIT
