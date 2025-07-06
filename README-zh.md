# RetroArch Fast Playlist Indexer v1.3.0

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.3.0-brightgreen.svg)](https://github.com/seu-usuario/retroarch-fast-indexer)
[![Rust Edition](https://img.shields.io/badge/rust_edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

用Rust编写的高性能通用ROM索引器，旨在比原生扫描器更快地生成RetroArch播放列表（`.lpl`）。

## 🌍 支持的语言 / Supported Languages

- 🇺🇸 [English](README.md) (默认)
- 🇧🇷 [Português](README-pt.md)
- 🇪🇸 [Español](README-es.md) 
- 🇫🇷 [Français](README-fr.md)
- 🇩🇪 [Deutsch](README-de.md)
- 🇯🇵 [日本語](README-ja.md)
- 🇷🇺 [Русский](README-ru.md)
- 🇨🇳 [中文](README-zh.md)

## 📋 版本历史

### v1.3.0 (当前) - 07/05/2025 ✅
- **🏗️ Rust Edition 2024**: 完全迁移到Rust 2024
- **🚀 所有功能已实现**: 缓存、监视模式、存档支持、DAT下载、验证、去重
- **🔧 更新依赖**: 与Rust 1.88.0完全兼容
- **📦 功能性发布构建**: 成功生成优化的二进制文件
- **🛠️ 扩展CLI**: `watch`、`download-dats`、`validate`、`deduplicate`、`cache`命令
- **🌍 i18n系统**: 实现基本多语言支持
- **⚠️ 警告策略**: 任务完成前始终修复警告

### v1.2.0 - 07/05/2025 ✅
- **🔧 更新依赖**: 迁移到最新版本（Rayon 1.10、Tokio 1.40、Clap 4.5等）
- **📦 路线图已实现**: 路线图v1.1/v1.2中的所有高级功能
- **🗄️ 持久缓存**: 用于优化的CRC32缓存系统
- **📦 存档支持**: 带可选功能的ZIP/7z
- **👀 监视模式**: 实时目录监控
- **🌐 DAT下载**: 从No-Intro/Redump自动下载
- **✅ 验证**: 通过DAT的完整验证系统
- **🗂️ 去重**: 5种智能去重策略

### v1.1.0
- ⚡ **改进性能**: 并行处理和文件处理优化
- 🧹 **清洁代码**: 减少警告和不必要的导入
- 🔒 **稳定性**: 与当前Rust生态系统100%兼容

### v1.0.0
- 🚀 **首次发布**: 基本并行扫描器
- 🔍 **CRC32检测**: 精确ROM识别
- 📋 **播放列表生成**: .lpl文件创建
- 🎮 **多平台**: 自动路径转换
- 🏷️ **DAT支持**: 通过DAT文件命名

> **📊 详细状态**: 有关开发和实现的完整信息，请参见[`STATUS.md`](STATUS.md)

## 🚀 快速开始

### 📦 预编译二进制文件（推荐）

从[`bin/`](bin/)文件夹下载适用于您平台的二进制文件：

```bash
# Windows x64
.\bin\windows\x64\retroarch-indexer.exe --help

# Linux x64
./bin/linux/x64/retroarch-indexer --help

# macOS Intel
./bin/macos/intel/retroarch-indexer --help

# 检查可用的二进制文件
.\check-binaries.ps1  # Windows
./check-binaries.sh   # Linux/macOS
```

### 🔧 手动编译

```bash
# 克隆仓库
git clone https://github.com/seu-usuario/retroarch-fast-indexer
cd retroarch-fast-indexer

# 基本编译
cargo build --release

# 带所有功能的编译
cargo build --release --all-features

# 带特定功能的编译
cargo build --release --features archive-support,dat-download

# 运行
./target/release/retroarch-indexer --help

# 为多个平台编译
.\build-all.ps1  # Windows
./build-all.sh   # Linux/macOS
```

### 🚀 编译功能

```toml
# Cargo.toml中的可用功能
[features]
default = []
archive-support = ["zip", "sevenz-rust"]     # ZIP/7z支持
dat-download = ["reqwest", "md5", "sha2"]    # 自动DAT下载
watch-mode = ["notify"]                      # 目录监控
checksums = ["md5", "sha2"]                  # 附加算法
full = ["archive-support", "dat-download", "watch-mode", "checksums"]
```

### 基本用法

```bash
# 扫描ROM并创建播放列表
retroarch-indexer --roms-dir /path/to/roms

# 指定界面语言
retroarch-indexer --language zh --roms-dir /path/to/roms

# 转换现有播放列表
retroarch-indexer convert Nintendo\ 64.lpl --target switch

# 批量转换
retroarch-indexer convert-all --input-dir ./playlists --source windows --target switch
```

### 高级用法 (v1.3.3)

```bash
# 自动目录监控
retroarch-indexer watch --roms-dir /path/to/roms --output-dir ./playlists

# 自动DAT下载
retroarch-indexer download-dats --systems "Nintendo 64,SNES" --output-dir ./dats

# 通过DAT验证ROM
retroarch-indexer validate --dat-dir ./dats --report validation-report.json

# 智能去重
retroarch-indexer deduplicate --strategy filename-quality --dry-run

# 缓存管理
retroarch-indexer cache stats
retroarch-indexer cache clean

# 线程控制和实时监控
retroarch-indexer --roms-dir ./roms --threads 8 -v  # 8线程带详细日志
retroarch-indexer --roms-dir ./roms --threads 2     # 为慢速SSD限制为2线程
```

### 🔧 **实时线程监控 (v1.3.3)**

新系统准确显示每个线程正在做什么：

```
🔍 扫描目录 1 / 1: ./roms
⠁ [=====>----------------------------------] 2/10 (20%) 处理文件 3 / 10
⠁ 线程   0 | 🔍 扫描中: Super Mario 64.z64
⠁ 线程   1 | 📦 解压中 game.zip (75.2%)
⠁ 线程   2 | 🔢 CRC32: Zelda OoT.z64
⠁ 线程   3 | ✅ 完成: Mario Kart.z64
```

**可能的线程状态:**
- 🔸 **空闲**: 等待工作
- 🔍 **扫描中**: 从磁盘读取文件
- 📦 **解压中**: 处理压缩文件（带百分比）
- 🔢 **CRC32**: 计算校验和
- ✅ **完成**: 文件处理成功
- ❌ **错误**: 处理失败

## 🌍 多语言支持

国际化（i18n）系统支持以下语言：

```bash
# 手动设置语言
retroarch-indexer --language en   # 英语（默认）
retroarch-indexer --language pt   # 葡萄牙语
retroarch-indexer --language es   # 西班牙语
retroarch-indexer --language fr   # 法语
retroarch-indexer --language de   # 德语
retroarch-indexer --language ja   # 日语
retroarch-indexer --language ru   # 俄语
retroarch-indexer --language zh   # 中文
```

系统自动检测操作系统语言，如果语言不可用则回退到英语。

## ✨ 功能 (v1.3.0)

- **🚀 比RetroArch原生扫描器快15-17倍**
- **🔄 自动转换**平台间路径转换
- **📋 统一主播放列表**包含所有ROM
- **🎮 通用支持**所有ROM格式
- **🏷️ 智能命名**通过DAT文件
- **📊 优化并行处理**
- **🌍 多语言界面**支持8种语言

### 🆕 高级功能
- **🗄️ 持久缓存**: 用于重新扫描优化的CRC32缓存
- **📦 存档支持**: 直接读取ZIP/7z中的ROM
- **👀 监视模式**: 自动目录监控
- **🌐 DAT下载**: 自动数据库下载
- **✅ 验证**: 通过DAT进行完整性验证
- **🗂️ 去重**: 智能删除重复ROM

### 📋 可用CLI命令
```bash
retroarch-indexer                    # 基本索引
retroarch-indexer convert            # 播放列表转换
retroarch-indexer convert-all        # 批量转换
retroarch-indexer watch              # 自动监控
retroarch-indexer download-dats      # DAT下载
retroarch-indexer validate           # ROM验证
retroarch-indexer deduplicate        # 去重
retroarch-indexer cache              # 缓存管理
```

## 📖 文档

| 文件 | 描述 |
|------|-----|
| [`CLAUDE.md`](CLAUDE.md) | 🛠️ **最佳实践**和技术架构 |
| [`STATUS.md`](STATUS.md) | 📊 **项目状态**和当前路线图 |
| [`LEGAL_COMPLIANCE.md`](LEGAL_COMPLIANCE.md) | ⚖️ **法律指导**使用指南 |
| [`KASPERSKY_SOLUTION.md`](KASPERSKY_SOLUTION.md) | 🛡️ **解决方案**误报问题 |

### 完整技术文档

- **系统架构**和代码模式
- **高级配置**指南
- **使用示例**所有功能
- **详细性能基准**
- **开发路线图**和当前状态

## 🛠️ 开发

```bash
# 运行测试
cargo test

# 带调试日志运行
RUST_LOG=debug cargo run -- --roms-dir ./test-roms -vv

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 自动修复警告
cargo clippy --fix --allow-dirty --allow-staged
```

## ⚖️ 合法使用

**重要**: 此工具专门用于组织和编目您物理拥有的游戏的**合法内容**和**个人备份**。

### ✅ 允许的使用
- 组织从您自己的卡带/光盘创建的ROM
- 验证个人备份的完整性
- 在您的平台之间转换播放列表
- 学术研究和文化保护

### ❌ 禁止的使用
- 下载、分享或分发受版权保护的ROM
- 与非法获得的内容一起使用
- 组织ROM的商业分发

**有关完整的法律使用指南，请参见[LEGAL_COMPLIANCE.md](LEGAL_COMPLIANCE.md)。**

## 📝 许可证

此项目根据MIT许可证授权 - 有关详细信息，请参见[LICENSE](LICENSE)文件。

---

**🎮 RetroArch Fast Playlist Indexer v1.3.0** - 自2024年以来改变ROM组织

**注意**: 此项目与RetroArch无官方关联。这是一个独立工具，旨在合法和道德地改善ROM管理体验。
