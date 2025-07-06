# 中文本地化
app-name = RetroArch 快速播放列表索引器
app-description = 适用于 RetroArch 的高性能通用 ROM 索引器
app-header = 🚀 RetroArch Fast Playlist Indexer v1.3.3
app-separator = ═══════════════════════════════════════════════

# 执行模式
execution-mode = 执行模式
choose-indexer-execution = 选择索引器执行模式：
interactive-mode-console-selection = 交互模式（控制台选择）
interactive-mode-desc = 选择要索引的特定系统
automatic-mode-scan-all = 自动模式（扫描全部）
automatic-mode-desc = 自动检测并索引所有系统
select-mode = 选择模式

# 命令
cmd-scan = 扫描 ROM 并生成播放列表
cmd-convert = 将现有播放列表转换为其他平台
cmd-convert-all = 转换目录中的所有播放列表
cmd-watch = 监视目录变化
cmd-download-dats = 下载 DAT 文件
cmd-validate = 根据 DAT 文件验证 ROM
cmd-deduplicate = 删除重复的 ROM
cmd-cache = 管理 CRC32 缓存

# 通用
path = 路径
platform = 平台
output = 输出
threads = 线程
verbose = 详细
help = 帮助
source = 源
target = 目标

# 扫描
scanning-roms = 正在扫描 ROM...
found-roms = 找到 { $count } 个 ROM
processing-file = 正在处理: { $filename }
extracting-archive = 正在提取存档: { $progress }%
calculating-crc32 = 正在计算 CRC32
scan-complete = 扫描完成
scanning-directory = 📂 扫描中: { $path }
scanning-directory-progress = 🔍 正在扫描目录 { $current } / { $total }: { $path }
scanning-directory-indexed = 🔍 正在扫描目录 { $current } / { $total }: { $path }

# 系统检测
systems-detected = 检测到的系统:
rom-count = { $count } 个 ROM
master-playlist = 主播放列表
master-playlist-info = └─ roms.lpl (包含 { $count } 个 ROM 的主播放列表)

# 目录配置
rom-directories-config = 📂 ROM 目录配置
roms-dir-prompt = 输入 ROM 目录路径
roms-dir-prompt-additional = 输入另一个 ROM 目录（或按 Enter 继续）
directory-not-found = ⚠️ 目录未找到: { $path }
not-a-directory = ⚠️ 不是目录: { $path }
directory-added = ✅ 已添加: { $path }
directory-created = ✅ 目录已创建: { $path }
max-directories-reached = ⚠️ 已达到最大目录数
directories-scanned = ├─ 已扫描目录: { $count }
directories-count = { $count } 个目录

# 错误
error-invalid-path = 无效路径: { $path }
error-no-roms-found = 在目录中未找到 ROM: { $path }
error-platform-unsupported = 不支持的平台: { $platform }
error-roms-dir-required = ❌ 错误: 必须指定至少一个 ROM 目录
no-roms-found = ⚠️ 在指定目录中未找到 ROM

# 成功消息
playlists-created = 播放列表创建成功
indexing-complete = 🎉 索引完成成功！

# 加载中
loading-playlist = 📄 正在加载: { $filename }
