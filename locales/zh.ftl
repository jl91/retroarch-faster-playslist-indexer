# 中文本地化
app-name = RetroArch 快速播放列表索引器
app-description = 适用于 RetroArch 的高性能通用 ROM 索引器

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

# 扫描
scanning-roms = 正在扫描 ROM...
found-roms = 找到 { $count } 个 ROM
processing-file = 正在处理: { $filename }
extracting-archive = 正在提取存档: { $progress }%
calculating-crc32 = 正在计算 CRC32
scan-complete = 扫描完成

# 系统检测
systems-detected = 检测到的系统:
rom-count = { $count } 个 ROM
master-playlist = 主播放列表

# 错误
error-invalid-path = 无效路径: { $path }
error-no-roms-found = 在目录中未找到 ROM: { $path }
error-platform-unsupported = 不支持的平台: { $platform }

# 交互式提示
prompt-roms-dir = 请选择 ROM 目录:
prompt-source-platform = 选择源平台:
prompt-target-platform = 选择目标平台:
prompt-output-dir = 选择输出目录:
prompt-create-dir = 目录不存在。是否创建? (是/否)

# 成功消息
playlists-created = 播放列表创建成功
cache-cleared = 缓存已清除
cache-stats = 缓存统计
validation-complete = 验证完成
deduplication-complete = 去重完成
