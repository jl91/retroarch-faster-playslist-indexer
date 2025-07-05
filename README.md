# RetroArch Fast Playlist Indexer v1.1.0

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.1.0-brightgreen.svg)](https://github.com/seu-usuario/retroarch-fast-indexer)

Um indexador universal de ROMs de alta performance escrito em Rust, projetado para gerar playlists RetroArch (`.lpl`) com velocidade superior ao scanner nativo.

## ⭐ Novidades v1.1.0
- 🔧 **Dependências Atualizadas**: Migração para as versões mais recentes (Rayon 1.10, Tokio 1.40, Clap 4.5, etc.)
- ⚡ **Performance Melhorada**: Otimizações de processamento paralelo e manipulação de arquivos
- 🧹 **Código Limpo**: Redução de warnings e imports desnecessários
- 🔒 **Estabilidade**: 100% compatível com ecossistema Rust atual

## 🚀 Início Rápido

### Instalação

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/retroarch-fast-indexer
cd retroarch-fast-indexer

# Compilação em modo release
cargo build --release

# Executar
./target/release/retroarch-indexer --help
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

## ✨ Características

- **🚀 15-17x mais rápido** que o scanner nativo do RetroArch
- **🔄 Conversão automática** de caminhos entre plataformas
- **📋 Playlist master unificada** com todas as ROMs
- **🎮 Suporte universal** a todos os formatos de ROMs
- **🏷️ Nomenclatura inteligente** via arquivos DAT
- **📊 Processamento paralelo** otimizado

## 📖 Documentação Completa

Veja [CLAUDE.md](CLAUDE.md) para documentação técnica detalhada, incluindo:

- Arquitetura do sistema
- Guia de configuração
- Exemplos de uso avançado
- Benchmarks de performance
- Roadmap de desenvolvimento

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
