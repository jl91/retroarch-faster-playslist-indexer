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

# プラットフォーム設定
platforms-configuration = 🔄 プラットフォーム設定
select-source-platform = { $type }プラットフォームを選択（実行している場所）:
select-target-platform = { $type }プラットフォームを選択（使用される場所）:

# 出力設定
output-directory-config = 📁 出力ディレクトリ設定
output-dir-prompt = プレイリストの出力ディレクトリを入力
create-directory-prompt = ディレクトリが存在しません。'{ $path }'を作成しますか？
use-default-output-dir = デフォルトの出力ディレクトリ'./playlists'を使用しますか？
output-directory = └─ 出力ディレクトリ: { $path }

# 変換
conversion = 🔄 変換: { $source } → { $target }
playlist-conversion-mode = 🔄 プレイリスト変換モード
batch-conversion-mode = 🔄 バッチ変換モード
converting-to = 🎯 変換先: { $platform }
platform-detected = ✅ プラットフォーム検出: { $platform }
detecting-source-platform = 🔍 ソースプラットフォームを検出中...

# 統計とサマリー
scan-summary = 📈 スキャンサマリー:
total-roms = ├─ 総ROM数: { $count }
total-roms-found = 🔍 総ROM数: { $count }
total-roms-stat = ├─ 総ROM数: { $count }
roms-processed = ├─ 処理済みROM: { $count }
archives-found = ├─ 見つかったアーカイブ: { $count }
scan-time = └─ スキャン時間: { $time }秒
total-cache-entries = ├─ 総エントリ: { $count }

# 検証
validation-total = ├─ 総計: { $count }
validation-valid = ├─ ✅ 有効: { $count } ({ $percentage }%)
validation-need-rename = ├─ 🔄 リネームが必要: { $count }
validation-unknown = ├─ ❓ 不明: { $count }
validation-homebrew = ├─ 🏠 Homebrew/Hack: { $count }
validation-bad-dumps = ├─ ❌ 不良ダンプ: { $count }
validation-corrupted = └─ 💥 破損: { $count }

# コンソール/システム選択
console-cores-selection = 🎯 コンソール/コア選択
available-systems-consoles = 利用可能なシステム/コンソール:
select-system-or-finish = 設定するシステムを選択（または'選択完了'で終了）:
finish-selection = 🏁 選択完了
system-already-selected = ⚠️ システム { $system } は既に選択されています！
systems-selected-so-far = これまでに選択されたシステム: { $count }
system-added = ✅ システム { $system } が追加されました！

# 特定のコンソール設定
configuration-for-system = ⚙️ { $system } の設定
roms-directory-for-system = { $system } のROMディレクトリ
output-directory-for-system = { $system } プレイリストの出力ディレクトリ
create-output-directory = 出力ディレクトリ '{ $path }' を作成しますか？

# 強制システム
forced-system = 🎯 強制システム: { $system }
forced-system-scan = ├─ 強制システム: { $system }
forcing-rom-to-system = 🎯 { $rom } をシステム { $system } に強制

# Watchモード
watch-mode-active = ✅ Watchモードがアクティブです！停止するにはCtrl+Cを押してください...
watch-active-press-ctrl-c = ✅ ウォッチアクティブ！停止するには Ctrl+C を押してください...

# システムダウンロード
systems-for-download = 🎯 ダウンロード対象システム: { $systems }

# キャッシュ
cache-stats = 📊 キャッシュ統計:
cache-size = ├─ キャッシュサイズ: { $size }
cache-entries = ├─ エントリ: { $count }
cache-hit-rate = └─ ヒット率: { $rate }%
clearing-cache = 🗑️ キャッシュをクリア中...
cache-cleared = ✅ キャッシュが正常にクリアされました
cache-system-loaded = └─ { $system }：{ $count } システムエントリ
cache-total-entries = ├─ 総エントリ数：{ $count }
loading-cache = 📦 { $platform } のキャッシュを読み込み中...
saving-cache = 💾 { $platform } のキャッシュを保存中...

# エラー
error-invalid-path = 無効なパス: { $path }
error-no-roms-found = ディレクトリにROMが見つかりません: { $path }
error-platform-unsupported = サポートされていないプラットフォーム: { $platform }
error-roms-dir-required = ❌ エラー: 少なくとも1つのROMディレクトリを指定してください
no-roms-found = ⚠️ 指定されたディレクトリにROMが見つかりません
error-config-load = ❌ 設定の読み込みエラー: { $error }
error-scanner-create = ❌ スキャナー作成エラー: { $error }
error-getting-roms-dirs = ❌ ROMディレクトリ取得エラー: { $error }
error-getting-platforms = ❌ プラットフォーム取得エラー: { $error }
error-getting-output-dir = ❌ 出力ディレクトリ取得エラー: { $error }
error-building-playlists = ❌ プレイリスト構築エラー: { $error }
error-loading-playlist = ❌ プレイリスト読み込みエラー: { $error }
error-converting-playlist = ❌ プレイリスト変換エラー: { $error }
error-saving-playlist = ❌ プレイリスト保存エラー: { $error }
error-starting-watch = ❌ 監視開始エラー: { $error }
error-downloading-dats = ❌ DAT ダウンロードエラー: { $error }
error-validating-roms = ❌ ROM検証エラー: { $error }
error-deduplicating-roms = ❌ ROM重複除去エラー: { $error }
error-managing-cache = ❌ キャッシュ管理エラー: { $error }
no-lpl-files-found = ⚠️ 指定されたディレクトリに.lplファイルが見つかりません
error-processing-system = ❌ システム処理エラー: { $error }

# インタラクティブプロンプト
prompt-roms-dir = ROMディレクトリを選択:
prompt-source-platform = ソースプラットフォームを選択:
prompt-target-platform = ターゲットプラットフォームを選択:
prompt-output-dir = 出力ディレクトリを選択:
prompt-create-dir = ディレクトリが存在しません。作成しますか？ (y/n)

# 成功メッセージ
playlists-created = プレイリストが正常に作成されました
indexing-complete = 🎉 インデックス化が正常に完了しました！
processing-all-consoles-complete = 🎉 すべてのコンソールの処理が完了しました！
batch-conversion-complete = ✅ バッチ変換完了:
successful-conversions = ├─ 成功: { $count }
failed-conversions = ├─ 失敗: { $count }
report-generated = 📄 レポート生成: { $path }
validation-complete = ✅ 検証完了
deduplication-complete = ✅ 重複除去完了
no-available-systems = ⚠️ 利用可能なシステムが見つかりません

# ローディング
loading-playlist = 📄 読み込み中: { $filename }

# システムメッセージ
no-system-selected = ⚠️  システムが選択されていません。終了しています...
initialization-warning = 警告: ローカライゼーションの初期化に失敗しました: {$error}
usage-instruction = 使用法: {$command} --roms-dir <パス>
error-processing-failed = ❌ {$system}: {$error}
roms-found-summary = 📊 {$count} ROMが{$time}秒で見つかりました
archives-detected = 📦 {$count} 圧縮ファイルが検出されました
directory-not-exist-warning = ⚠️ ディレクトリが存在しません: { $path }
