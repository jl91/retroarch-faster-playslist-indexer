# RetroArch Fast Playlist Indexer v1.3.0

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.3.0-brightgreen.svg)](https://github.com/seu-usuario/retroarch-fast-indexer)
[![Rust Edition](https://img.shields.io/badge/rust_edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

Rustで書かれた高性能ユニバーサルROMインデクサー。ネイティブスキャナーよりも高速にRetroArchプレイリスト（`.lpl`）を生成するように設計されています。

## 🌍 対応言語 / Supported Languages

- 🇺🇸 [English](README.md) (デフォルト)
- 🇧🇷 [Português](README-pt.md)
- 🇪🇸 [Español](README-es.md) 
- 🇫🇷 [Français](README-fr.md)
- 🇩🇪 [Deutsch](README-de.md)
- 🇯🇵 [日本語](README-ja.md)
- 🇷🇺 [Русский](README-ru.md)
- 🇨🇳 [中文](README-zh.md)

## 📋 バージョン履歴

### v1.3.0 (現在) - 07/05/2025 ✅
- **🏗️ Rust Edition 2024**: Rust 2024への完全移行
- **🚀 全機能実装**: キャッシュ、ウォッチモード、アーカイブサポート、DATダウンロード、検証、重複除去
- **🔧 依存関係更新**: Rust 1.88.0との完全な互換性
- **📦 機能的リリースビルド**: 最適化されたバイナリの生成成功
- **🛠️ 拡張CLI**: `watch`、`download-dats`、`validate`、`deduplicate`、`cache`コマンド
- **🌍 i18nシステム**: 基本的な多言語サポートの実装
- **⚠️ 警告ポリシー**: タスク完了前に常に警告を修正

### v1.2.0 - 07/05/2025 ✅
- **🔧 依存関係更新**: 最新バージョンへの移行（Rayon 1.10、Tokio 1.40、Clap 4.5など）
- **📦 ロードマップ実装**: ロードマップv1.1/v1.2からの全高度機能
- **🗄️ 永続キャッシュ**: 最適化のためのCRC32キャッシュシステム
- **📦 アーカイブサポート**: オプション機能付きZIP/7z
- **👀 ウォッチモード**: リアルタイムディレクトリ監視
- **🌐 DATダウンロード**: No-Intro/Redumpからの自動ダウンロード
- **✅ 検証**: DATによる完全検証システム
- **🗂️ 重複除去**: 5つのインテリジェントな重複除去戦略

### v1.1.0
- ⚡ **パフォーマンス向上**: 並列処理とファイル処理の最適化
- 🧹 **クリーンコード**: 警告と不要なインポートの削減
- 🔒 **安定性**: 現在のRustエコシステムとの100%互換性

### v1.0.0
- 🚀 **初回リリース**: 基本的な並列スキャナー
- 🔍 **CRC32検出**: 精密なROM識別
- 📋 **プレイリスト生成**: .lplファイルの作成
- 🎮 **マルチプラットフォーム**: 自動パス変換
- 🏷️ **DATサポート**: DATファイルによる命名

> **📊 詳細ステータス**: 開発と実装に関する完全な情報については、[`STATUS.md`](STATUS.md)をご覧ください

## 🚀 クイックスタート

### 📦 プリコンパイル済みバイナリ（推奨）

[`bin/`](bin/)フォルダからあなたのプラットフォーム用のバイナリをダウンロードしてください：

```bash
# Windows x64
.\bin\windows\x64\retroarch-indexer.exe --help

# Linux x64
./bin/linux/x64/retroarch-indexer --help

# macOS Intel
./bin/macos/intel/retroarch-indexer --help

# 利用可能なバイナリを確認
.\check-binaries.ps1  # Windows
./check-binaries.sh   # Linux/macOS
```

### 🔧 手動コンパイル

```bash
# リポジトリをクローン
git clone https://github.com/seu-usuario/retroarch-fast-indexer
cd retroarch-fast-indexer

# 基本コンパイル
cargo build --release

# 全機能付きコンパイル
cargo build --release --all-features

# 特定機能付きコンパイル
cargo build --release --features archive-support,dat-download

# 実行
./target/release/retroarch-indexer --help

# 複数プラットフォーム向けコンパイル
.\build-all.ps1  # Windows
./build-all.sh   # Linux/macOS
```

### 🚀 コンパイル機能

```toml
# Cargo.tomlで利用可能な機能
[features]
default = []
archive-support = ["zip", "sevenz-rust"]     # ZIP/7zサポート
dat-download = ["reqwest", "md5", "sha2"]    # 自動DATダウンロード
watch-mode = ["notify"]                      # ディレクトリ監視
checksums = ["md5", "sha2"]                  # 追加アルゴリズム
full = ["archive-support", "dat-download", "watch-mode", "checksums"]
```

### 基本的な使用方法

```bash
# インタラクティブモード - 同期する特定のコンソールを選択
retroarch-indexer --roms-dir /path/to/roms
# ツールが次から選択するよう促します：
# 1. インタラクティブモード - コンソール選択
# 2. 自動モード - すべてのディレクトリをスキャン

# 自動モード（従来の動作）
retroarch-indexer --roms-dir /path/to/roms --auto

# インターフェース言語を指定
retroarch-indexer --language ja --roms-dir /path/to/roms

# 既存のプレイリストを変換
retroarch-indexer convert Nintendo\ 64.lpl --target switch

# バッチ変換
retroarch-indexer convert-all --input-dir ./playlists --source windows --target switch
```

### 🎯 インタラクティブモード（新機能）

インタラクティブモードでは以下が可能です：
- すべてをスキャンする代わりに**特定のコンソールを選択**して処理
- 各コンソールに対して**ROMディレクトリを個別に設定**
- **すべてのROM**を選択したコンソールに属するものとして強制的に扱う
- コンソールごとに**正確に1つのプレイリストを生成**（自動検出を無視）

**インタラクティブモードのワークフロー：**
1. "インタラクティブモード - コンソール選択"を選択
2. 処理するシステムを選択（Nintendo 64、SNES等）
3. 各システムに対して以下を指定：
   - ROMディレクトリパス
   - プレイリストの出力ディレクトリ
4. ツールは各コンソールを個別に処理し、すべてのROMが選択されたシステムに属するものとして扱われることを保証

**インタラクティブセッションの例：**
```
🎮 実行モード
インデクサーの実行方法を選択してください：
  1. インタラクティブモード - コンソール選択 - 同期するコンソール/コアを選択
  2. 自動モード - すべてのディレクトリをスキャン - 提供されたすべてのディレクトリを自動的にスキャン

モードを選択: 1

🎯 コンソール/コア選択
利用可能なシステム/コンソール：
  Nintendo:
    • Nintendo - Nintendo 64
    • Nintendo - Super Nintendo Entertainment System
    • Nintendo - Game Boy Advance

設定するシステムを選択: Nintendo - Nintendo 64

⚙️ Nintendo - Nintendo 64の設定
Nintendo - Nintendo 64のROMディレクトリ: /path/to/n64/roms
出力ディレクトリ: ./playlists

🔄 処理中: Nintendo - Nintendo 64
🎯 すべてのROMをシステムに強制: Nintendo - Nintendo 64
📊 処理されたシステム:
└─ Nintendo - Nintendo 64: 25 ROMs
✅ 正常に完了
```

### 高度な使用方法 (v1.3.3)

```bash
# 自動ディレクトリ監視
retroarch-indexer watch --roms-dir /path/to/roms --output-dir ./playlists

# 自動DATダウンロード
retroarch-indexer download-dats --systems "Nintendo 64,SNES" --output-dir ./dats

# DATによるROM検証
retroarch-indexer validate --dat-dir ./dats --report validation-report.json

# インテリジェント重複除去
retroarch-indexer deduplicate --strategy filename-quality --dry-run

# キャッシュ管理
retroarch-indexer cache stats
retroarch-indexer cache clean

# スレッド制御とリアルタイム監視
retroarch-indexer --roms-dir ./roms --threads 8 -v  # 詳細ログ付き8スレッド
retroarch-indexer --roms-dir ./roms --threads 2     # 低速SSD用に2スレッドに制限
```

### 🔧 **リアルタイムスレッド監視 (v1.3.3)**

新しいシステムは各スレッドが何をしているかを正確に表示します：

```
🔍 ディレクトリ 1 / 1 をスキャン中: ./roms
⠁ [=====>----------------------------------] 2/10 (20%) ファイル 3 / 10 を処理中
⠁ スレッド  0 | 🔍 スキャン中: Super Mario 64.z64
⠁ スレッド  1 | 📦 展開中 game.zip (75.2%)
⠁ スレッド  2 | 🔢 CRC32計算中: Zelda OoT.z64
⠁ スレッド  3 | ✅ 完了: Mario Kart.z64
```

**可能なスレッドステータス:**
- 🔸 **アイドル**: 作業待機中
- 🔍 **スキャン中**: ディスクからファイルを読み込み中
- 📦 **展開中**: 圧縮ファイルを処理中（%付き）
- 🔢 **CRC32**: チェックサム計算中
- ✅ **完了**: ファイル処理成功
- ❌ **エラー**: 処理失敗

## 🌍 多言語サポート

国際化（i18n）システムは以下の言語をサポートしています：

```bash
# 言語を手動設定
retroarch-indexer --language en   # 英語（デフォルト）
retroarch-indexer --language pt   # ポルトガル語
retroarch-indexer --language es   # スペイン語
retroarch-indexer --language fr   # フランス語
retroarch-indexer --language de   # ドイツ語
retroarch-indexer --language ja   # 日本語
retroarch-indexer --language ru   # ロシア語
retroarch-indexer --language zh   # 中国語
```

システムはオペレーティングシステムの言語を自動検出し、利用できない場合は英語にフォールバックします。

## ✨ 機能 (v1.3.0)

- **🚀 15-17倍高速** RetroArchのネイティブスキャナーより
- **🎯 インタラクティブモード** 選択的なコンソール処理用
- **🔄 自動変換** プラットフォーム間でのパス変換
- **📋 統一マスタープレイリスト** 全ROMを含む
- **🎮 ユニバーサルサポート** 全ROMフォーマット対応
- **🏷️ インテリジェント命名** DATファイルによる
- **📊 最適化された並列処理**
- **🌍 多言語インターフェース** 8言語サポート

### 🆕 高度な機能
- **🗄️ 永続キャッシュ**: 再スキャン最適化のためのCRC32キャッシュ
- **📦 アーカイブサポート**: ZIP/7z内のROMの直接読み込み
- **👀 ウォッチモード**: 自動ディレクトリ監視
- **🌐 DATダウンロード**: 自動データベースダウンロード
- **✅ 検証**: DATによる整合性検証
- **🗂️ 重複除去**: 重複ROMのインテリジェント除去

### 📋 利用可能なCLIコマンド
```bash
retroarch-indexer                    # 基本インデックス作成
retroarch-indexer convert            # プレイリスト変換
retroarch-indexer convert-all        # バッチ変換
retroarch-indexer watch              # 自動監視
retroarch-indexer download-dats      # DATダウンロード
retroarch-indexer validate           # ROM検証
retroarch-indexer deduplicate        # 重複除去
retroarch-indexer cache              # キャッシュ管理
```

## 📖 ドキュメント

| ファイル | 説明 |
|---------|------|
| [`CLAUDE.md`](CLAUDE.md) | 🛠️ **ベストプラクティス**と技術アーキテクチャ |
| [`STATUS.md`](STATUS.md) | 📊 **プロジェクトステータス**と現在のロードマップ |
| [`LEGAL_COMPLIANCE.md`](LEGAL_COMPLIANCE.md) | ⚖️ **法的ガイドライン**使用について |

### 完全な技術ドキュメント

- **システムアーキテクチャ**とコードパターン
- **高度な設定**ガイド
- **使用例**全機能について
- **詳細なパフォーマンスベンチマーク**
- **開発ロードマップ**と現在のステータス

## 🛠️ 開発

```bash
# テスト実行
cargo test

# デバッグログ付きで実行
RUST_LOG=debug cargo run -- --roms-dir ./test-roms -vv

# コードフォーマット
cargo fmt

# リンティング
cargo clippy

# 警告を自動修正
cargo clippy --fix --allow-dirty --allow-staged
```

## ⚖️ 法的使用

**重要**: このツールは、物理的に所有するゲームの**法的コンテンツ**と**個人バックアップ**の整理とカタログ化専用です。

### ✅ 許可された使用
- 自分のカートリッジ/ディスクから作成したROMの整理
- 個人バックアップの整合性検証
- プラットフォーム間でのプレイリスト変換
- 学術研究と文化的保存

### ❌ 禁止された使用
- 著作権で保護されたROMのダウンロード、共有、配布
- 違法に取得したコンテンツでの使用
- 整理されたROMの商用配布

**完全な法的使用ガイドラインについては、[LEGAL_COMPLIANCE.md](LEGAL_COMPLIANCE.md)をご覧ください。**

## 📝 ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルをご覧ください。

---

**🎮 RetroArch Fast Playlist Indexer v1.3.0** - 2024年以来ROM整理を変革

**注意**: このプロジェクトはRetroArchと公式に関連していません。法的かつ倫理的にROM管理体験を改善するために作成された独立したツールです。
