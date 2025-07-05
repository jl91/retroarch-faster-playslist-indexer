# 📦 Binários Pré-compilados

Esta pasta contém os binários pré-compilados do **RetroArch Fast Playlist Indexer** para diferentes plataformas.

## 📁 Estrutura

```
bin/
├── windows/          # Binários para Windows
│   ├── x64/         # Windows 64-bit
│   └── x86/         # Windows 32-bit
├── linux/           # Binários para Linux
│   ├── x64/         # Linux 64-bit
│   ├── arm64/       # Linux ARM64 (Raspberry Pi 4+)
│   └── armv7/       # Linux ARMv7 (Raspberry Pi 2/3)
└── macos/           # Binários para macOS
    ├── intel/       # macOS Intel (x64)
    └── apple/       # macOS Apple Silicon (ARM64)
```

## 🚀 Download e Uso

### Windows
```powershell
# Download do executável
# Executar diretamente
.\bin\windows\x64\retroarch-indexer.exe --help
```

### Linux
```bash
# Dar permissão de execução
chmod +x ./bin/linux/x64/retroarch-indexer

# Executar
./bin/linux/x64/retroarch-indexer --help
```

### macOS
```bash
# Dar permissão de execução
chmod +x ./bin/macos/intel/retroarch-indexer

# Executar
./bin/macos/intel/retroarch-indexer --help
```

## 📋 Versões Disponíveis

| Plataforma | Arquitetura | Arquivo | Tamanho | Status |
|------------|-------------|---------|---------|---------|
| Windows | x64 | `retroarch-indexer.exe` | ~8MB | ✅ Disponível |
| Windows | x86 | `retroarch-indexer.exe` | ~7MB | 🔄 Em breve |
| Linux | x64 | `retroarch-indexer` | ~8MB | ✅ Disponível |
| Linux | ARM64 | `retroarch-indexer` | ~8MB | 🔄 Em breve |
| Linux | ARMv7 | `retroarch-indexer` | ~7MB | 🔄 Em breve |
| macOS | Intel | `retroarch-indexer` | ~9MB | 🔄 Em breve |
| macOS | Apple Silicon | `retroarch-indexer` | ~8MB | 🔄 Em breve |

## 🔧 Compilação Manual

Se preferir compilar você mesmo:

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/retroarch-fast-indexer
cd retroarch-fast-indexer

# Compile em modo release
cargo build --release

# O binário estará em ./target/release/
```

### Cross-compilation

Para compilar para outras plataformas:

```bash
# Instalar targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Compilar para Windows
cargo build --release --target x86_64-pc-windows-gnu

# Compilar para Linux ARM64
cargo build --release --target aarch64-unknown-linux-gnu

# Compilar para macOS Intel
cargo build --release --target x86_64-apple-darwin

# Compilar para macOS Apple Silicon
cargo build --release --target aarch64-apple-darwin
```

## 🔐 Verificação de Integridade

Todos os binários incluem checksums SHA256 para verificação:

```bash
# Verificar integridade (Linux/macOS)
sha256sum bin/linux/x64/retroarch-indexer

# Verificar integridade (Windows PowerShell)
Get-FileHash bin\windows\x64\retroarch-indexer.exe -Algorithm SHA256
```

## 📱 Dispositivos Específicos

### Raspberry Pi
- **Pi 4/5**: Use `linux/arm64/retroarch-indexer`
- **Pi 2/3**: Use `linux/armv7/retroarch-indexer`
- **Pi Zero/1**: Compilação manual necessária

### Steam Deck
- Use `linux/x64/retroarch-indexer`
- Funciona nativamente no SteamOS

### Nintendo Switch (Homebrew)
- Compilação específica necessária
- Veja documentação de homebrew

## ⚠️ Nota Legal

Estes binários são fornecidos para conveniência e devem ser usados apenas com **conteúdo legal** e **backups pessoais**. Veja [LEGAL_COMPLIANCE.md](../LEGAL_COMPLIANCE.md) para diretrizes completas.

## 🔄 Atualizações

Os binários são atualizados a cada release. Para garantir que você tem a versão mais recente:

1. Verifique a versão atual: `./retroarch-indexer --version`
2. Compare com a versão no [GitHub Releases](https://github.com/seu-usuario/retroarch-fast-indexer/releases)
3. Baixe a versão mais recente se necessário

---

**💡 Dica**: Para uso em produção, recomendamos sempre compilar a partir do código-fonte para garantir máxima performance e segurança.
