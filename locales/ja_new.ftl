# 日本語ローカライゼーション
app-name = RetroArch Fast Playlist Indexer
app-description = RetroArch用の高性能ユニバーサルROMインデクサー
app-header = 🚀 RetroArch Fast Playlist Indexer v1.3.3
app-separator = ═══════════════════════════════════════════════

# 実行モード
execution-mode = 実行モード
choose-indexer-execution = インデクサーの実行モードを選択してください：
interactive-mode-console-selection = インタラクティブモード（コンソール選択）
interactive-mode-desc = インデックス化する特定のシステムを選択
automatic-mode-scan-all = 自動モード（全スキャン）
automatic-mode-desc = すべてのシステムを自動的に検出・インデックス化
select-mode = モードを選択

# コマンド
cmd-scan = ROMをスキャンしてプレイリストを生成
cmd-convert = 既存のプレイリストを他のプラットフォームに変換
cmd-convert-all = ディレクトリ内のすべてのプレイリストを変換
cmd-watch = ディレクトリの変更を監視
cmd-download-dats = DATファイルをダウンロード
cmd-validate = DATファイルに対してROMを検証
cmd-deduplicate = 重複するROMを削除
cmd-cache = CRC32キャッシュを管理

# 共通
path = パス
platform = プラットフォーム
output = 出力
threads = スレッド
verbose = 詳細
help = ヘルプ
source = ソース
target = ターゲット

# スキャン
scanning-roms = ROMをスキャン中...
found-roms = { $count }個のROMが見つかりました
processing-file = 処理中: { $filename }
extracting-archive = アーカイブを展開中: { $progress }%
calculating-crc32 = CRC32を計算中
scan-complete = スキャン完了
scanning-directory = 📂 スキャン中: { $path }
scanning-directory-progress = 🔍 ディレクトリ { $current } / { $total } をスキャン中: { $path }
scanning-directory-indexed = 🔍 ディレクトリ { $current } / { $total } をスキャン中: { $path }

# システム検出
systems-detected = 検出されたシステム:
rom-count = { $count } ROM
master-playlist = マスタープレイリスト
master-playlist-info = └─ roms.lpl ({ $count } ROMのマスタープレイリスト)

# ディレクトリ設定
rom-directories-config = 📂 ROMディレクトリの設定
roms-dir-prompt = ROMディレクトリのパスを入力してください
roms-dir-prompt-additional = 別のROMディレクトリを入力してください（続行するにはEnter）
directory-not-found = ⚠️ ディレクトリが見つかりません: { $path }
not-a-directory = ⚠️ ディレクトリではありません: { $path }
directory-added = ✅ 追加済み: { $path }
directory-created = ✅ ディレクトリを作成しました: { $path }
max-directories-reached = ⚠️ 最大ディレクトリ数に達しました
directories-scanned = ├─ スキャン済みディレクトリ: { $count }
directories-count = { $count } ディレクトリ

# エラー
error-invalid-path = 無効なパス: { $path }
error-no-roms-found = ディレクトリにROMが見つかりません: { $path }
error-platform-unsupported = サポートされていないプラットフォーム: { $platform }
error-roms-dir-required = ❌ エラー: 少なくとも1つのROMディレクトリを指定してください
no-roms-found = ⚠️ 指定されたディレクトリにROMが見つかりません

# 成功メッセージ
playlists-created = プレイリストが正常に作成されました
indexing-complete = 🎉 インデックス化が正常に完了しました！

# ロード中
loading-playlist = 📄 ロード中: { $filename }
