# RetroArch Fast Playlist Indexer v1.3.0

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.3.0-brightgreen.svg)](https://github.com/seu-usuario/retroarch-fast-indexer)
[![Rust Edition](https://img.shields.io/badge/rust_edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

Ein hochperformanter universeller ROM-Indexer, geschrieben in Rust, entwickelt um RetroArch-Playlists (`.lpl`) schneller als der native Scanner zu generieren.

## 🌍 Unterstützte Sprachen / Supported Languages

- 🇺🇸 [English](README.md) (Standard)
- 🇧🇷 [Português](README-pt.md)
- 🇪🇸 [Español](README-es.md) 
- 🇫🇷 [Français](README-fr.md)
- 🇩🇪 [Deutsch](README-de.md)
- 🇯🇵 [日本語](README-ja.md)
- 🇷🇺 [Русский](README-ru.md)
- 🇨🇳 [中文](README-zh.md)

## 📋 Versionshistorie

### v1.3.0 (Aktuell) - 07/05/2025 ✅
- **🏗️ Rust Edition 2024**: Vollständige Migration zu Rust 2024
- **🚀 Alle Funktionen Implementiert**: Cache, Watch-Modus, Archiv-Unterstützung, DAT-Download, Validierung, Deduplizierung
- **🔧 Aktualisierte Abhängigkeiten**: Vollständige Kompatibilität mit Rust 1.88.0
- **📦 Funktionale Release-Build**: Optimierte Binärdatei erfolgreich generiert
- **🛠️ Erweiterte CLI**: Befehle `watch`, `download-dats`, `validate`, `deduplicate`, `cache`
- **🌍 i18n-System**: Grundlegende mehrsprachige Unterstützung implementiert
- **⚠️ Warnungs-Richtlinie**: Immer Warnungen beheben vor Aufgabenabschluss

### v1.2.0 - 07/05/2025 ✅
- **🔧 Aktualisierte Abhängigkeiten**: Migration zu neuesten Versionen (Rayon 1.10, Tokio 1.40, Clap 4.5, usw.)
- **📦 Roadmap Implementiert**: Alle erweiterten Funktionen aus Roadmap v1.1/v1.2
- **🗄️ Persistenter Cache**: CRC32-Cache-System für Optimierung
- **📦 Archiv-Unterstützung**: ZIP/7z mit optionalen Features
- **👀 Watch-Modus**: Echtzeit-Verzeichnisüberwachung
- **🌐 DAT-Download**: Automatischer Download von No-Intro/Redump
- **✅ Validierung**: Vollständiges Validierungssystem über DAT
- **🗂️ Deduplizierung**: 5 intelligente Deduplizierungsstrategien

### v1.1.0
- ⚡ **Verbesserte Performance**: Parallelverarbeitung und Dateibehandlungsoptimierungen
- 🧹 **Sauberer Code**: Reduzierte Warnungen und unnötige Importe
- 🔒 **Stabilität**: 100% kompatibel mit aktuellem Rust-Ökosystem

### v1.0.0
- 🚀 **Erste Veröffentlichung**: Grundlegender Parallel-Scanner
- 🔍 **CRC32-Erkennung**: Präzise ROM-Identifikation
- 📋 **Playlist-Generierung**: .lpl-Dateierstellung
- 🎮 **Multi-Plattform**: Automatische Pfadkonvertierung
- 🏷️ **DAT-Unterstützung**: Benennung über DAT-Dateien

> **📊 Detaillierter Status**: Für vollständige Informationen zur Entwicklung und Implementierung siehe [`STATUS.md`](STATUS.md)

## 🚀 Schnellstart

### 📦 Vorkompilierte Binärdateien (Empfohlen)

Laden Sie die Binärdatei für Ihre Plattform aus dem [`bin/`](bin/) Ordner herunter:

```bash
# Windows x64
.\bin\windows\x64\retroarch-indexer.exe --help

# Linux x64
./bin/linux/x64/retroarch-indexer --help

# macOS Intel
./bin/macos/intel/retroarch-indexer --help

# Verfügbare Binärdateien prüfen
.\check-binaries.ps1  # Windows
./check-binaries.sh   # Linux/macOS
```

### 🔧 Manuelle Kompilierung

```bash
# Repository klonen
git clone https://github.com/seu-usuario/retroarch-fast-indexer
cd retroarch-fast-indexer

# Grundlegende Kompilierung
cargo build --release

# Kompilierung mit allen Features
cargo build --release --all-features

# Kompilierung mit spezifischen Features
cargo build --release --features archive-support,dat-download

# Ausführen
./target/release/retroarch-indexer --help

# Für mehrere Plattformen kompilieren
.\build-all.ps1  # Windows
./build-all.sh   # Linux/macOS
```

### 🚀 Kompilierungs-Features

```toml
# Verfügbare Features in Cargo.toml
[features]
default = []
archive-support = ["zip", "sevenz-rust"]     # ZIP/7z-Unterstützung
dat-download = ["reqwest", "md5", "sha2"]    # Automatischer DAT-Download
watch-mode = ["notify"]                      # Verzeichnisüberwachung
checksums = ["md5", "sha2"]                  # Zusätzliche Algorithmen
full = ["archive-support", "dat-download", "watch-mode", "checksums"]
```

### Grundlegende Verwendung

```bash
# ROMs scannen und Playlists erstellen
retroarch-indexer --roms-dir /pfad/zu/roms

# Interface-Sprache angeben
retroarch-indexer --language de --roms-dir /pfad/zu/roms

# Bestehende Playlist konvertieren
retroarch-indexer convert Nintendo\ 64.lpl --target switch

# Batch-Konvertierung
retroarch-indexer convert-all --input-dir ./playlists --source windows --target switch
```

### Erweiterte Verwendung (v1.3.3)

```bash
# Automatische Verzeichnisüberwachung
retroarch-indexer watch --roms-dir /pfad/zu/roms --output-dir ./playlists

# Automatischer DAT-Download
retroarch-indexer download-dats --systems "Nintendo 64,SNES" --output-dir ./dats

# ROM-Validierung über DAT
retroarch-indexer validate --dat-dir ./dats --report validierung-bericht.json

# Intelligente Deduplizierung
retroarch-indexer deduplicate --strategy filename-quality --dry-run

# Cache-Verwaltung
retroarch-indexer cache stats
retroarch-indexer cache clean

# Thread-Kontrolle und Echtzeit-Überwachung
retroarch-indexer --roms-dir ./roms --threads 8 -v  # 8 Threads mit detailliertem Log
retroarch-indexer --roms-dir ./roms --threads 2     # Auf 2 Threads begrenzen für langsame SSDs
```

### 🔧 **Echtzeit-Thread-Überwachung (v1.3.3)**

Das neue System zeigt genau, was jeder Thread macht:

```
🔍 Verzeichnis 1 von 1 scannen: ./roms
⠁ [=====>----------------------------------] 2/10 (20%) Datei 3 von 10 verarbeiten
⠁ Thread  0 | 🔍 Scannen: Super Mario 64.z64
⠁ Thread  1 | 📦 Extrahieren game.zip (75.2%)
⠁ Thread  2 | 🔢 CRC32: Zelda OoT.z64
⠁ Thread  3 | ✅ Komplett: Mario Kart.z64
```

**Mögliche Thread-Status:**
- 🔸 **Leerlauf**: Wartet auf Arbeit
- 🔍 **Scannen**: Datei von Festplatte lesen
- 📦 **Extrahieren**: Komprimierte Datei verarbeiten (mit %)
- 🔢 **CRC32**: Prüfsumme berechnen
- ✅ **Komplett**: Datei erfolgreich verarbeitet
- ❌ **Fehler**: Verarbeitungsfehler

## 🌍 Mehrsprachige Unterstützung

Das Internationalisierungssystem (i18n) unterstützt folgende Sprachen:

```bash
# Sprache manuell setzen
retroarch-indexer --language en   # Englisch (Standard)
retroarch-indexer --language pt   # Portugiesisch
retroarch-indexer --language es   # Spanisch
retroarch-indexer --language fr   # Französisch
retroarch-indexer --language de   # Deutsch
retroarch-indexer --language ja   # Japanisch
retroarch-indexer --language ru   # Russisch
retroarch-indexer --language zh   # Chinesisch
```

Das System erkennt automatisch die Betriebssystemsprache und fällt auf Englisch zurück, wenn die Sprache nicht verfügbar ist.

## ✨ Funktionen (v1.3.0)

- **🚀 15-17x schneller** als RetroArchs nativer Scanner
- **🔄 Automatische Konvertierung** von Pfaden zwischen Plattformen
- **📋 Einheitliche Master-Playlist** mit allen ROMs
- **🎮 Universelle Unterstützung** für alle ROM-Formate
- **🏷️ Intelligente Benennung** über DAT-Dateien
- **📊 Optimierte Parallelverarbeitung**
- **🌍 Mehrsprachige Benutzeroberfläche** mit 8 unterstützten Sprachen

### 🆕 Erweiterte Funktionen
- **🗄️ Persistenter Cache**: CRC32-Cache für Re-Scan-Optimierung
- **📦 Archiv-Unterstützung**: Direktes Lesen von ROMs in ZIP/7z
- **👀 Watch-Modus**: Automatische Verzeichnisüberwachung
- **🌐 DAT-Download**: Automatischer Datenbankdownload
- **✅ Validierung**: Integritätsprüfung über DAT
- **🗂️ Deduplizierung**: Intelligente Entfernung doppelter ROMs

### 📋 Verfügbare CLI-Befehle
```bash
retroarch-indexer                    # Grundlegende Indizierung
retroarch-indexer convert            # Playlist-Konvertierung
retroarch-indexer convert-all        # Batch-Konvertierung
retroarch-indexer watch              # Automatische Überwachung
retroarch-indexer download-dats      # DAT-Download
retroarch-indexer validate           # ROM-Validierung
retroarch-indexer deduplicate        # Duplikate entfernen
retroarch-indexer cache              # Cache-Verwaltung
```

## 📖 Dokumentation

| Datei | Beschreibung |
|-------|-------------|
| [`CLAUDE.md`](CLAUDE.md) | 🛠️ **Best Practices** und technische Architektur |
| [`STATUS.md`](STATUS.md) | 📊 **Projektstatus** und aktuelle Roadmap |
| [`LEGAL_COMPLIANCE.md`](LEGAL_COMPLIANCE.md) | ⚖️ **Rechtliche Richtlinien** für die Nutzung |

### Vollständige technische Dokumentation

- **Systemarchitektur** und Code-Muster
- **Erweiterte Konfiguration** Anleitung
- **Verwendungsbeispiele** für alle Funktionen
- **Detaillierte Performance-Benchmarks**
- **Entwicklungs-Roadmap** und aktueller Status

## 🛠️ Entwicklung

```bash
# Tests ausführen
cargo test

# Mit Debug-Logs ausführen
RUST_LOG=debug cargo run -- --roms-dir ./test-roms -vv

# Code-Formatierung
cargo fmt

# Linting
cargo clippy

# Warnungen automatisch beheben
cargo clippy --fix --allow-dirty --allow-staged
```

## ⚖️ Rechtliche Nutzung

**WICHTIG**: Dieses Tool ist ausschließlich für die Organisation und Katalogisierung von **rechtmäßigen Inhalten** und **persönlichen Backups** von Spielen, die Sie physisch besitzen, bestimmt.

### ✅ Erlaubte Nutzung
- ROMs organisieren, die von Ihren eigenen Cartridges/Discs erstellt wurden
- Integrität persönlicher Backups validieren
- Playlists zwischen Ihren Plattformen konvertieren
- Akademische Forschung und kulturelle Bewahrung

### ❌ Verbotene Nutzung
- Urheberrechtlich geschützte ROMs herunterladen, teilen oder verbreiten
- Nutzung mit illegal erworbenem Inhalt
- Kommerzielle Verbreitung organisierter ROMs

**Siehe [LEGAL_COMPLIANCE.md](LEGAL_COMPLIANCE.md) für vollständige rechtliche Nutzungsrichtlinien.**

## 📝 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe die [LICENSE](LICENSE)-Datei für Details.

---

**🎮 RetroArch Fast Playlist Indexer v1.3.0** - ROM-Organisation revolutioniert seit 2024

**Hinweis**: Dieses Projekt ist nicht offiziell mit RetroArch verbunden. Es ist ein unabhängiges Tool, das erstellt wurde, um die ROM-Verwaltungserfahrung legal und ethisch zu verbessern.
