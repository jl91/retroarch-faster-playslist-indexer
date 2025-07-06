# RetroArch Fast Playlist Indexer v1.3.0

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.3.0-brightgreen.svg)](https://github.com/seu-usuario/retroarch-fast-indexer)
[![Rust Edition](https://img.shields.io/badge/rust_edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

A high-performance universal ROM indexer written in Rust, designed to generate RetroArch playlists (`.lpl`) faster than the native scanner.

## 🌍 Supported Languages / Idiomas Suportados

- 🇺🇸 [English](README.md) (Default)
- 🇧🇷 [Português](README-pt.md)
- 🇪🇸 [Español](README-es.md) 
- 🇫🇷 [Français](README-fr.md)
- 🇩🇪 [Deutsch](README-de.md)
- 🇯🇵 [日本語](README-ja.md)
- 🇷🇺 [Русский](README-ru.md)
- 🇨🇳 [中文](README-zh.md)

## 📋 Version History

### v1.3.0 (Current) - 07/05/2025 ✅
- **🏗️ Rust Edition 2024**: Complete migration to Rust 2024
- **🚀 All Features Implemented**: Cache, Watch Mode, Archive Support, DAT Download, Validation, Deduplication
- **🔧 Updated Dependencies**: Full compatibility with Rust 1.88.0
- **📦 Functional Release Build**: Optimized binary generated successfully
- **🛠️ Expanded CLI**: Commands `watch`, `download-dats`, `validate`, `deduplicate`, `cache`
- **🌍 i18n System**: Basic multilingual support implemented
- **⚠️ Warning Policy**: Always fix warnings before completing tasks

### v1.2.0 - 07/05/2025 ✅
- **🔧 Updated Dependencies**: Migration to latest versions (Rayon 1.10, Tokio 1.40, Clap 4.5, etc.)
- **📦 Roadmap Implemented**: All advanced features from roadmap v1.1/v1.2
- **🗄️ Persistent Cache**: CRC32 cache system for optimization
- **📦 Archive Support**: ZIP/7z with optional features
- **👀 Watch Mode**: Real-time directory monitoring
- **🌐 DAT Download**: Automatic download from No-Intro/Redump
- **✅ Validation**: Complete validation system via DAT
- **🗂️ Deduplication**: 5 intelligent deduplication strategies

### v1.1.0
- ⚡ **Improved Performance**: Parallel processing and file handling optimizations
- 🧹 **Clean Code**: Reduced warnings and unnecessary imports
- 🔒 **Stability**: 100% compatible with current Rust ecosystem

### v1.0.0
- 🚀 **Initial Release**: Basic parallel scanner
- 🔍 **CRC32 Detection**: Precise ROM identification
- 📋 **Playlist Generation**: .lpl file creation
- 🎮 **Multi-platform**: Automatic path conversion
- 🏷️ **DAT Support**: Naming via DAT files

> **📊 Detailed Status**: For complete information on development and implementation, see [`STATUS.md`](STATUS.md)

## 🚀 Quick Start

### 📦 Pre-compiled Binaries (Recommended)

Download the binary for your platform from the [`bin/`](bin/) folder:

```bash
# Windows x64
.\bin\windows\x64\retroarch-indexer.exe --help

# Linux x64
./bin/linux/x64/retroarch-indexer --help

# macOS Intel
./bin/macos/intel/retroarch-indexer --help

# Check available binaries
.\check-binaries.ps1  # Windows
./check-binaries.sh   # Linux/macOS
```

### 🔧 Manual Compilation

```bash
# Clone the repository
git clone https://github.com/seu-usuario/retroarch-fast-indexer
cd retroarch-fast-indexer

# Basic compilation
cargo build --release

# Compilation with all features
cargo build --release --all-features

# Compilation with specific features
cargo build --release --features archive-support,dat-download

# Run
./target/release/retroarch-indexer --help

# Compile for multiple platforms
.\build-all.ps1  # Windows
./build-all.sh   # Linux/macOS
```

### 🚀 Compilation Features

```toml
# Available features in Cargo.toml
[features]
default = []
archive-support = ["zip", "sevenz-rust"]     # ZIP/7z support
dat-download = ["reqwest", "md5", "sha2"]    # Automatic DAT download
watch-mode = ["notify"]                      # Directory monitoring
checksums = ["md5", "sha2"]                  # Additional algorithms
full = ["archive-support", "dat-download", "watch-mode", "checksums"]
```

### Basic Usage

```bash
# Scan ROMs and create playlists
retroarch-indexer --roms-dir /path/to/roms

# Specify interface language
retroarch-indexer --language en --roms-dir /path/to/roms

# Convert existing playlist
retroarch-indexer convert Nintendo\ 64.lpl --target switch

# Batch conversion
retroarch-indexer convert-all --input-dir ./playlists --source windows --target switch
```

### Advanced Usage (v1.3.3)

```bash
# Automatic directory monitoring
retroarch-indexer watch --roms-dir /path/to/roms --output-dir ./playlists

# Automatic DAT download
retroarch-indexer download-dats --systems "Nintendo 64,SNES" --output-dir ./dats

# ROM validation via DAT
retroarch-indexer validate --dat-dir ./dats --report validation-report.json

# Intelligent deduplication
retroarch-indexer deduplicate --strategy filename-quality --dry-run

# Cache management
retroarch-indexer cache stats
retroarch-indexer cache clean

# Thread control and real-time monitoring
retroarch-indexer --roms-dir ./roms --threads 8 -v  # 8 threads with detailed log
retroarch-indexer --roms-dir ./roms --threads 2     # Limit to 2 threads for slow SSDs
```

### 🔧 **Real-time Thread Monitoring (v1.3.3)**

The new system shows exactly what each thread is doing:

```
🔍 Scanning directory 1 of 1: ./roms
⠁ [=====>----------------------------------] 2/10 (20%) Processing file 3 of 10
⠁ Thread  0 | 🔍 Scanning: Super Mario 64.z64
⠁ Thread  1 | 📦 Extracting game.zip (75.2%)
⠁ Thread  2 | 🔢 CRC32: Zelda OoT.z64
⠁ Thread  3 | ✅ Complete: Mario Kart.z64
```

**Possible thread statuses:**
- 🔸 **Idle**: Waiting for work
- 🔍 **Scanning**: Reading file from disk
- 📦 **Extracting**: Processing compressed file (with %)
- 🔢 **CRC32**: Calculating checksum
- ✅ **Complete**: File processed successfully
- ❌ **Error**: Processing failure

## 🌍 Multilingual Support

The internationalization (i18n) system supports the following languages:

```bash
# Set language manually
retroarch-indexer --language en   # English (default)
retroarch-indexer --language pt   # Portuguese
retroarch-indexer --language es   # Spanish
retroarch-indexer --language fr   # French
retroarch-indexer --language de   # German
retroarch-indexer --language ja   # Japanese
retroarch-indexer --language ru   # Russian
retroarch-indexer --language zh   # Chinese
```

The system automatically detects the operating system language and falls back to English if the language is not available.

## ✨ Features (v1.3.0)

- **🚀 15-17x faster** than RetroArch's native scanner
- **🔄 Automatic conversion** of paths between platforms
- **📋 Unified master playlist** with all ROMs
- **🎮 Universal support** for all ROM formats
- **🏷️ Intelligent naming** via DAT files
- **📊 Optimized parallel processing**
- **🌍 Multilingual interface** with 8 supported languages

### 🆕 Advanced Features
- **🗄️ Persistent Cache**: CRC32 cache for re-scan optimization
- **📦 Archive Support**: Direct reading of ROMs in ZIP/7z
- **👀 Watch Mode**: Automatic directory monitoring
- **🌐 DAT Download**: Automatic database download
- **✅ Validation**: Integrity verification via DAT
- **🗂️ Deduplication**: Intelligent removal of duplicate ROMs

### 📋 Available CLI Commands
```bash
retroarch-indexer                    # Basic indexing
retroarch-indexer convert            # Playlist conversion
retroarch-indexer convert-all        # Batch conversion
retroarch-indexer watch              # Automatic monitoring
retroarch-indexer download-dats      # DAT download
retroarch-indexer validate           # ROM validation
retroarch-indexer deduplicate        # Duplicate removal
retroarch-indexer cache              # Cache management
```

## 📖 Documentation

| File | Description |
|------|-------------|
| [`CLAUDE.md`](CLAUDE.md) | 🛠️ **Best practices** and technical architecture |
| [`STATUS.md`](STATUS.md) | 📊 **Project status** and current roadmap |
| [`LEGAL_COMPLIANCE.md`](LEGAL_COMPLIANCE.md) | ⚖️ **Legal guidelines** for use |
| [`KASPERSKY_SOLUTION.md`](KASPERSKY_SOLUTION.md) | 🛡️ **Solution** for false positives |

### Complete Technical Documentation

- **System architecture** and code patterns
- **Advanced configuration** guide
- **Usage examples** for all features
- **Detailed performance benchmarks**
- **Development roadmap** and current status

## 🛠️ Development

```bash
# Run tests
cargo test

# Run with debug logs
RUST_LOG=debug cargo run -- --roms-dir ./test-roms -vv

# Code formatting
cargo fmt

# Linting
cargo clippy

# Automatically fix warnings
cargo clippy --fix --allow-dirty --allow-staged
```

## ⚖️ Legal Use

**IMPORTANT**: This tool is intended exclusively for organizing and cataloging **legal content** and **personal backups** of games you physically own.

### ✅ Permitted Use
- Organize ROMs created from your own cartridges/discs
- Validate integrity of personal backups
- Convert playlists between your platforms
- Academic research and cultural preservation

### ❌ Prohibited Use
- Download, share, or distribute copyrighted ROMs
- Use with illegally obtained content
- Commercial distribution of organized ROMs

**See [LEGAL_COMPLIANCE.md](LEGAL_COMPLIANCE.md) for complete legal usage guidelines.**

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**🎮 RetroArch Fast Playlist Indexer v1.3.0** - Transforming ROM organization since 2024

**Note**: This project is not officially affiliated with RetroArch. It is an independent tool created to improve the ROM management experience legally and ethically.
