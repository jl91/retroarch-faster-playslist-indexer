# 日本語ローカライゼーション
app-name = RetroArch Fast Playlist Indexer
app-description = RetroArch用の高性能ユニバーサルROMインデクサー

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

# スキャン
scanning-roms = ROMをスキャン中...
found-roms = { $count } 個のROMが見つかりました
processing-file = 処理中: { $filename }
extracting-archive = アーカイブを展開中: { $progress }%
calculating-crc32 = CRC32を計算中
scan-complete = スキャン完了

# システム検出
systems-detected = 検出されたシステム:
rom-count = { $count } 個のROM
master-playlist = マスタープレイリスト

# エラー
error-invalid-path = 無効なパス: { $path }
error-no-roms-found = ディレクトリにROMが見つかりません: { $path }
error-platform-unsupported = サポートされていないプラットフォーム: { $platform }

# インタラクティブプロンプト
prompt-roms-dir = ROMディレクトリを選択してください:
prompt-source-platform = ソースプラットフォームを選択:
prompt-target-platform = ターゲットプラットフォームを選択:
prompt-output-dir = 出力ディレクトリを選択:
prompt-create-dir = ディレクトリが存在しません。作成しますか？ (y/n)

# 成功メッセージ
playlists-created = プレイリストが正常に作成されました
cache-cleared = キャッシュがクリアされました
cache-stats = キャッシュ統計
validation-complete = 検証完了
deduplication-complete = 重複除去完了
