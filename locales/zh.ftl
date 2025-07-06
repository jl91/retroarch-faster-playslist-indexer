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

# 平台配置
platforms-configuration = 🔄 平台配置
select-source-platform = 选择{ $type }平台（您正在运行的地方）
select-target-platform = 选择{ $type }平台（将要使用的地方）

# 输出配置
output-directory-config = 📁 输出目录配置
output-dir-prompt = 输入播放列表的输出目录
create-directory-prompt = 目录不存在。创建'{ $path }'？
use-default-output-dir = 使用默认输出目录'./playlists'？
output-directory = └─ 输出目录：{ $path }

# 转换
conversion = 🔄 转换：{ $source } → { $target }
playlist-conversion-mode = 🔄 播放列表转换模式
batch-conversion-mode = 🔄 批量转换模式
converting-to = 🎯 转换到：{ $platform }
platform-detected = ✅ 检测到平台：{ $platform }
detecting-source-platform = 🔍 检测源平台...

# 统计和摘要
scan-summary = 📈 扫描摘要：
total-roms = ├─ 总ROM数：{ $count }
total-roms-found = 🔍 总ROM数：{ $count }
total-roms-stat = ├─ 总ROM数：{ $count }
roms-processed = ├─ 已处理ROM：{ $count }
archives-found = ├─ 找到的存档：{ $count }
scan-time = └─ 扫描时间：{ $time }秒
total-cache-entries = ├─ 总条目：{ $count }

# 验证
validation-total = ├─ 总计：{ $count }
validation-valid = ├─ ✅ 有效：{ $count }（{ $percentage }%）
validation-need-rename = ├─ 🔄 需要重命名：{ $count }
validation-unknown = ├─ ❓ 未知：{ $count }
validation-homebrew = ├─ 🏠 自制/破解：{ $count }
validation-bad-dumps = ├─ ❌ 坏转储：{ $count }
validation-corrupted = └─ 💥 损坏：{ $count }
validation-complete = ✅ 验证完成

# 控制台/系统选择
console-cores-selection = 🎯 控制台/核心选择
available-systems-consoles = 可用的系统/控制台：
select-system-or-finish = 选择要配置的系统（或'完成选择'结束）
finish-selection = 🏁 完成选择
system-already-selected = ⚠️ 系统{ $system }已被选择！
systems-selected-so-far = 目前已选择的系统：{ $count }
system-added = ✅ 系统{ $system }已添加！

# 特定控制台配置
configuration-for-system = ⚙️ { $system }的配置
roms-directory-for-system = { $system }的ROM目录
output-directory-for-system = { $system }播放列表的输出目录
create-output-directory = 创建输出目录'{ $path }'？

# 强制系统
forced-system = 🎯 强制系统：{ $system }
forced-system-scan = ├─ 强制系统：{ $system }
forcing-rom-to-system = 🎯 强制{ $rom }到系统：{ $system }

# 监视模式
watch-mode-active = ✅ 监视模式已激活！按 Ctrl+C 停止...
watch-active-press-ctrl-c = ✅ 监视已激活！按 Ctrl+C 停止...

# 系统下载
systems-for-download = 🎯 待下载系统：{ $systems }

# 缓存
cache-stats = 📊 缓存统计：
cache-size = ├─ 缓存大小：{ $size }
cache-entries = ├─ 条目：{ $count }
cache-hit-rate = └─ 命中率：{ $rate }%
clearing-cache = 🗑️ 清理缓存...
cache-cleared = ✅ 缓存已成功清理
cache-system-loaded = └─ { $system }：{ $count } 条目
cache-total-entries = ├─ 总条目数：{ $count }
loading-cache = 📦 正在加载 { $platform } 的缓存...
saving-cache = 💾 正在保存 { $platform } 的缓存...

# 错误
error-invalid-path = 无效路径: { $path }
error-no-roms-found = 在目录中未找到 ROM: { $path }
error-platform-unsupported = 不支持的平台: { $platform }
error-roms-dir-required = ❌ 错误: 必须指定至少一个 ROM 目录
no-roms-found = ⚠️ 在指定目录中未找到 ROM
error-config-load = ❌ 加载配置错误：{ $error }
error-scanner-create = ❌ 创建扫描器错误：{ $error }
error-getting-roms-dirs = ❌ 获取ROM目录错误：{ $error }
error-getting-platforms = ❌ 获取平台错误：{ $error }
error-getting-output-dir = ❌ 获取输出目录错误：{ $error }
error-building-playlists = ❌ 构建播放列表错误：{ $error }
error-loading-playlist = ❌ 加载播放列表错误：{ $error }
error-converting-playlist = ❌ 转换播放列表错误：{ $error }
error-saving-playlist = ❌ 保存播放列表错误：{ $error }
error-starting-watch = ❌ 启动监视错误：{ $error }
error-downloading-dats = ❌ 下载DAT错误：{ $error }
error-validating-roms = ❌ 验证ROM错误：{ $error }
error-deduplicating-roms = ❌ 去除重复ROM错误：{ $error }
error-managing-cache = ❌ 管理缓存错误：{ $error }
error-processing-system = ❌ 处理系统错误：{ $error }
directory-not-exist-warning = ⚠️ 目录不存在：{ $path }
no-lpl-files-found = ⚠️ 在指定目录中未找到.lpl文件
error-processing-failed = ❌ 处理失败：{ $error }

# 交互提示
prompt-roms-dir = 选择ROM目录：
prompt-source-platform = 选择源平台：
prompt-target-platform = 选择目标平台：
prompt-output-dir = 选择输出目录：
prompt-create-dir = 目录不存在。创建？(y/n)

# 成功消息
playlists-created = 播放列表创建成功
indexing-complete = 🎉 索引完成成功！
processing-all-consoles-complete = 🎉 所有控制台处理完成！
batch-conversion-complete = ✅ 批量转换完成：
successful-conversions = ├─ 成功：{ $count }
failed-conversions = ├─ 失败：{ $count }
report-generated = 📄 报告已生成：{ $path }

archives-detected = 检测到压缩文件
deduplication-complete = 去重完成
initialization-warning = ⚠️ 初始化警告：{ $message }
loading-playlist = 📂 正在加载播放列表：{ $path }
no-available-systems = ⚠️ 未找到可用系统
no-system-selected = ⚠️ 未选择系统
roms-found-summary = 找到ROM摘要：{ $count } 个文件
usage-instruction = 使用说明：运行程序时请指定参数
