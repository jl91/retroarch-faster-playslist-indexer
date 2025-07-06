# RetroArch Fast Playlist Indexer v1.3.0

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.3.0-brightgreen.svg)](https://github.com/seu-usuario/retroarch-fast-indexer)
[![Rust Edition](https://img.shields.io/badge/rust_edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

Um indexador universal de ROMs de alta performance escrito em Rust, projetado para gerar playlists RetroArch (`.lpl`) com velocidade superior ao scanner nativo.

## 📋 Histórico de Versões

### v1.3.0 (Atual) - 05/07/2025 ✅
- **🏗️ Rust Edition 2024**: Migração completa para Rust 2024
- **🚀 Todas as Features Implementadas**: Cache, Watch Mode, Archive Support, DAT Download, Validation, Deduplication
- **🔧 Dependências Atualizadas**: Compatibilidade total com Rust 1.88.0
- **📦 Build Release Funcional**: Binário otimizado gerado com sucesso
- **🛠️ CLI Expandido**: Comandos `watch`, `download-dats`, `validate`, `deduplicate`, `cache`

### v1.2.0 - 05/07/2025 ✅
- **🔧 Dependências Atualizadas**: Migração para as versões mais recentes (Rayon 1.10, Tokio 1.40, Clap 4.5, etc.)
- **📦 Roadmap Implementado**: Todas as features avançadas do roadmap v1.1/v1.2
- **🗄️ Cache Persistente**: Sistema de cache CRC32 para otimização
- **📦 Suporte a Arquivos**: ZIP/7z com features opcionais
- **👀 Modo Watch**: Monitoramento de diretórios em tempo real
- **🌐 Download de DATs**: Download automático de No-Intro/Redump
- **✅ Validação**: Sistema completo de validação via DAT
- **🗂️ Deduplicação**: 5 estratégias de deduplicação inteligente

### v1.1.0
- ⚡ **Performance Melhorada**: Otimizações de processamento paralelo e manipulação de arquivos
- 🧹 **Código Limpo**: Redução de warnings e imports desnecessários
- 🔒 **Estabilidade**: 100% compatível com ecossistema Rust atual

### v1.0.0
- 🚀 **Lançamento Inicial**: Scanner paralelo básico
- 🔍 **Detecção por CRC32**: Identificação precisa de ROMs
- 📋 **Geração de Playlists**: Criação de arquivos .lpl
- 🎮 **Multi-plataforma**: Conversão automática de caminhos
- 🏷️ **Suporte a DAT**: Nomenclatura via arquivos DAT

> **📊 Status Detalhado**: Para informações completas sobre o desenvolvimento e implementação, consulte [`STATUS.md`](STATUS.md)

## 🚀 Início Rápido

### 📦 Binários Pré-compilados (Recomendado)

Baixe o binário para sua plataforma na pasta [`bin/`](bin/):

```bash
# Windows x64
.\bin\windows\x64\retroarch-indexer.exe --help

# Linux x64
./bin/linux/x64/retroarch-indexer --help

# macOS Intel
./bin/macos/intel/retroarch-indexer --help

# Verificar binários disponíveis
.\check-binaries.ps1  # Windows
./check-binaries.sh   # Linux/macOS
```

### 🔧 Compilação Manual

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/retroarch-fast-indexer
cd retroarch-fast-indexer

# Compilação básica
cargo build --release

# Compilação com todas as features
cargo build --release --all-features

# Compilação com features específicas
cargo build --release --features archive-support,dat-download

# Executar
./target/release/retroarch-indexer --help

# Compilar para múltiplas plataformas
.\build-all.ps1  # Windows
./build-all.sh   # Linux/macOS
```

### 🚀 Features de Compilação

```toml
# Features disponíveis no Cargo.toml
[features]
default = []
archive-support = ["zip", "sevenz-rust"]     # Suporte a ZIP/7z
dat-download = ["reqwest", "md5", "sha2"]    # Download automático de DATs
watch-mode = ["notify"]                      # Monitoramento de diretórios
checksums = ["md5", "sha2"]                  # Algoritmos adicionais
full = ["archive-support", "dat-download", "watch-mode", "checksums"]
```

### Uso Básico

```bash
# Escanear ROMs e criar playlists
retroarch-indexer --roms-dir /path/to/roms

# Converter playlist existente
retroarch-indexer convert Nintendo\ 64.lpl --target switch

# Conversão em lote
retroarch-indexer convert-all --input-dir ./playlists --source windows --target switch
```

### Uso Avançado (v1.3.0)

```bash
# Monitoramento automático de diretórios
retroarch-indexer watch --roms-dir /path/to/roms --output-dir ./playlists

# Download automático de DATs
retroarch-indexer download-dats --systems "Nintendo 64,SNES" --output-dir ./dats

# Validação de ROMs via DAT
retroarch-indexer validate --dat-dir ./dats --report validation-report.json

# Deduplicação inteligente
retroarch-indexer deduplicate --strategy filename-quality --dry-run

# Gerenciamento de cache
retroarch-indexer cache stats
retroarch-indexer cache clean
```

## ✨ Características (v1.3.0)

- **🚀 15-17x mais rápido** que o scanner nativo do RetroArch
- **🔄 Conversão automática** de caminhos entre plataformas
- **📋 Playlist master unificada** com todas as ROMs
- **🎮 Suporte universal** a todos os formatos de ROMs
- **🏷️ Nomenclatura inteligente** via arquivos DAT
- **📊 Processamento paralelo** otimizado

### 🆕 Features Avançadas
- **🗄️ Cache Persistente**: Cache CRC32 para otimização de re-scans
- **📦 Suporte a Arquivos**: Leitura direta de ROMs em ZIP/7z
- **👀 Modo Watch**: Monitoramento automático de diretórios
- **🌐 Download de DATs**: Download automático de bancos de dados
- **✅ Validação**: Verificação de integridade via DAT
- **🗂️ Deduplicação**: Remoção inteligente de ROMs duplicados

### 📋 Comandos CLI Disponíveis
```bash
retroarch-indexer                    # Indexação básica
retroarch-indexer convert            # Conversão de playlists
retroarch-indexer convert-all        # Conversão em lote
retroarch-indexer watch              # Monitoramento automático
retroarch-indexer download-dats      # Download de DATs
retroarch-indexer validate           # Validação de ROMs
retroarch-indexer deduplicate        # Remoção de duplicatas
retroarch-indexer cache              # Gerenciamento de cache
```

## 📖 Documentação

| Arquivo | Descrição |
|---------|-----------|
| [`CLAUDE.md`](CLAUDE.md) | 🛠️ **Boas práticas** e arquitetura técnica |
| [`STATUS.md`](STATUS.md) | 📊 **Status do projeto** e roadmap atual |
| [`LEGAL_COMPLIANCE.md`](LEGAL_COMPLIANCE.md) | ⚖️ **Diretrizes legais** de uso |
| [`KASPERSKY_SOLUTION.md`](KASPERSKY_SOLUTION.md) | 🛡️ **Solução** para falsos positivos |

### Documentação Técnica Completa

- **Arquitetura do sistema** e padrões de código
- **Guia de configuração** avançada
- **Exemplos de uso** para todas as features
- **Benchmarks de performance** detalhados
- **Roadmap de desenvolvimento** e status atual

## 🛠️ Desenvolvimento

```bash
# Executar testes
cargo test

# Executar com logs debug
RUST_LOG=debug cargo run -- --roms-dir ./test-roms -vv

# Formatação de código
cargo fmt

# Linting
cargo clippy
```

## ⚖️ Uso Legal

**IMPORTANTE**: Esta ferramenta é destinada exclusivamente para organizar e catalogar **conteúdo legal** e **backups pessoais** de jogos que você possui fisicamente.

### ✅ Uso Permitido
- Organizar ROMs criadas a partir de seus próprios cartuchos/discos
- Validar integridade de backups pessoais
- Converter playlists entre suas plataformas
- Pesquisa acadêmica e preservação cultural

### ❌ Uso Proibido
- Baixar, compartilhar ou distribuir ROMs protegidas por direitos autorais
- Usar com conteúdo obtido ilegalmente
- Comercialização de ROMs organizadas

**Veja [LEGAL_COMPLIANCE.md](LEGAL_COMPLIANCE.md) para diretrizes completas de uso legal.**

## 📝 Licença

Este projeto está licenciado sob a MIT License - veja o arquivo [LICENSE](LICENSE) para detalhes.
