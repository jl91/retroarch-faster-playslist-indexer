# RetroArch Fast Playlist Indexer v1.3.0

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.3.0-brightgreen.svg)](https://github.com/seu-usuario/retroarch-fast-indexer)
[![Rust Edition](https://img.shields.io/badge/rust_edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

Un indexador universal de ROMs de alto rendimiento escrito en Rust, diseñado para generar listas de reproducción de RetroArch (`.lpl`) más rápido que el escáner nativo.

## 🌍 Idiomas Soportados / Supported Languages

- 🇺🇸 [English](README.md) (Predeterminado)
- 🇧🇷 [Português](README-pt.md)
- 🇪🇸 [Español](README-es.md) 
- 🇫🇷 [Français](README-fr.md)
- 🇩🇪 [Deutsch](README-de.md)
- 🇯🇵 [日本語](README-ja.md)
- 🇷🇺 [Русский](README-ru.md)
- 🇨🇳 [中文](README-zh.md)

## 📋 Historial de Versiones

### v1.3.0 (Actual) - 05/07/2025 ✅
- **🏗️ Rust Edition 2024**: Migración completa a Rust 2024
- **🚀 Todas las Características Implementadas**: Cache, Modo Watch, Soporte de Archivos, Descarga DAT, Validación, Deduplicación
- **🔧 Dependencias Actualizadas**: Compatibilidad completa con Rust 1.88.0
- **📦 Build de Lanzamiento Funcional**: Binario optimizado generado exitosamente
- **🛠️ CLI Expandido**: Comandos `watch`, `download-dats`, `validate`, `deduplicate`, `cache`
- **🌍 Sistema i18n**: Soporte básico multiidioma implementado
- **⚠️ Política de Advertencias**: Siempre corregir advertencias antes de completar tareas

### v1.2.0 - 05/07/2025 ✅
- **🔧 Dependencias Actualizadas**: Migración a las últimas versiones (Rayon 1.10, Tokio 1.40, Clap 4.5, etc.)
- **📦 Hoja de Ruta Implementada**: Todas las características avanzadas de la hoja de ruta v1.1/v1.2
- **🗄️ Cache Persistente**: Sistema de cache CRC32 para optimización
- **📦 Soporte de Archivos**: ZIP/7z con características opcionales
- **👀 Modo Watch**: Monitoreo de directorios en tiempo real
- **🌐 Descarga DAT**: Descarga automática de No-Intro/Redump
- **✅ Validación**: Sistema de validación completo vía DAT
- **🗂️ Deduplicación**: 5 estrategias de deduplicación inteligente

### v1.1.0
- ⚡ **Rendimiento Mejorado**: Optimizaciones de procesamiento paralelo y manejo de archivos
- 🧹 **Código Limpio**: Reducción de advertencias e importaciones innecesarias
- 🔒 **Estabilidad**: 100% compatible con el ecosistema Rust actual

### v1.0.0
- 🚀 **Lanzamiento Inicial**: Escáner paralelo básico
- 🔍 **Detección CRC32**: Identificación precisa de ROMs
- 📋 **Generación de Listas**: Creación de archivos .lpl
- 🎮 **Multi-plataforma**: Conversión automática de rutas
- 🏷️ **Soporte DAT**: Nomenclatura vía archivos DAT

> **📊 Estado Detallado**: Para información completa sobre desarrollo e implementación, consulte [`STATUS.md`](STATUS.md)

## 🚀 Inicio Rápido

### 📦 Binarios Precompilados (Recomendado)

Descargue el binario para su plataforma desde la carpeta [`bin/`](bin/):

```bash
# Windows x64
.\bin\windows\x64\retroarch-indexer.exe --help

# Linux x64
./bin/linux/x64/retroarch-indexer --help

# macOS Intel
./bin/macos/intel/retroarch-indexer --help

# Verificar binarios disponibles
.\check-binaries.ps1  # Windows
./check-binaries.sh   # Linux/macOS
```

### 🔧 Compilación Manual

```bash
# Clonar el repositorio
git clone https://github.com/seu-usuario/retroarch-fast-indexer
cd retroarch-fast-indexer

# Compilación básica
cargo build --release

# Compilación con todas las características
cargo build --release --all-features

# Compilación con características específicas
cargo build --release --features archive-support,dat-download

# Ejecutar
./target/release/retroarch-indexer --help

# Compilar para múltiples plataformas
.\build-all.ps1  # Windows
./build-all.sh   # Linux/macOS
```

### 🚀 Características de Compilación

```toml
# Características disponibles en Cargo.toml
[features]
default = []
archive-support = ["zip", "sevenz-rust"]     # Soporte ZIP/7z
dat-download = ["reqwest", "md5", "sha2"]    # Descarga automática DAT
watch-mode = ["notify"]                      # Monitoreo de directorios
checksums = ["md5", "sha2"]                  # Algoritmos adicionales
full = ["archive-support", "dat-download", "watch-mode", "checksums"]
```

### Uso Básico

```bash
# Escanear ROMs y crear listas de reproducción
retroarch-indexer --roms-dir /path/to/roms

# Especificar idioma de la interfaz
retroarch-indexer --language es --roms-dir /path/to/roms

# Convertir lista de reproducción existente
retroarch-indexer convert Nintendo\ 64.lpl --target switch

# Conversión por lotes
retroarch-indexer convert-all --input-dir ./playlists --source windows --target switch
```

### Uso Avanzado (v1.3.3)

```bash
# Monitoreo automático de directorios
retroarch-indexer watch --roms-dir /path/to/roms --output-dir ./playlists

# Descarga automática de DAT
retroarch-indexer download-dats --systems "Nintendo 64,SNES" --output-dir ./dats

# Validación de ROM vía DAT
retroarch-indexer validate --dat-dir ./dats --report validation-report.json

# Deduplicación inteligente
retroarch-indexer deduplicate --strategy filename-quality --dry-run

# Gestión de cache
retroarch-indexer cache stats
retroarch-indexer cache clean

# Control de hilos y monitoreo en tiempo real
retroarch-indexer --roms-dir ./roms --threads 8 -v  # 8 hilos con registro detallado
retroarch-indexer --roms-dir ./roms --threads 2     # Limitar a 2 hilos para SSDs lentos
```

### 🔧 **Monitoreo de Hilos en Tiempo Real (v1.3.3)**

El nuevo sistema muestra exactamente qué está haciendo cada hilo:

```
🔍 Escaneando directorio 1 de 1: ./roms
⠁ [=====>----------------------------------] 2/10 (20%) Procesando archivo 3 de 10
⠁ Hilo  0 | 🔍 Escaneando: Super Mario 64.z64
⠁ Hilo  1 | 📦 Extrayendo game.zip (75.2%)
⠁ Hilo  2 | 🔢 CRC32: Zelda OoT.z64
⠁ Hilo  3 | ✅ Completo: Mario Kart.z64
```

**Estados posibles por hilo:**
- 🔸 **Inactivo**: Esperando trabajo
- 🔍 **Escaneando**: Leyendo archivo del disco
- 📦 **Extrayendo**: Procesando archivo comprimido (con %)
- 🔢 **CRC32**: Calculando checksum
- ✅ **Completo**: Archivo procesado exitosamente
- ❌ **Error**: Falla en el procesamiento

## 🌍 Soporte Multiidioma

El sistema de internacionalización (i18n) soporta los siguientes idiomas:

```bash
# Establecer idioma manualmente
retroarch-indexer --language en   # Inglés (predeterminado)
retroarch-indexer --language pt   # Portugués
retroarch-indexer --language es   # Español
retroarch-indexer --language fr   # Francés
retroarch-indexer --language de   # Alemán
retroarch-indexer --language ja   # Japonés
retroarch-indexer --language ru   # Ruso
retroarch-indexer --language zh   # Chino
```

El sistema detecta automáticamente el idioma del sistema operativo y recurre al inglés si el idioma no está disponible.

## ✨ Características (v1.3.0)

- **🚀 15-17x más rápido** que el escáner nativo de RetroArch
- **🔄 Conversión automática** de rutas entre plataformas
- **📋 Lista maestra unificada** con todos los ROMs
- **🎮 Soporte universal** para todos los formatos de ROM
- **🏷️ Nomenclatura inteligente** vía archivos DAT
- **📊 Procesamiento paralelo optimizado**
- **🌍 Interfaz multiidioma** con 8 idiomas soportados

### 🆕 Características Avanzadas
- **🗄️ Cache Persistente**: Cache CRC32 para optimización de re-escaneos
- **📦 Soporte de Archivos**: Lectura directa de ROMs en ZIP/7z
- **👀 Modo Watch**: Monitoreo automático de directorios
- **🌐 Descarga DAT**: Descarga automática de bases de datos
- **✅ Validación**: Verificación de integridad vía DAT
- **🗂️ Deduplicación**: Eliminación inteligente de ROMs duplicados

### 📋 Comandos CLI Disponibles
```bash
retroarch-indexer                    # Indexación básica
retroarch-indexer convert            # Conversión de listas
retroarch-indexer convert-all        # Conversión por lotes
retroarch-indexer watch              # Monitoreo automático
retroarch-indexer download-dats      # Descarga DAT
retroarch-indexer validate           # Validación de ROM
retroarch-indexer deduplicate        # Eliminación de duplicados
retroarch-indexer cache              # Gestión de cache
```

## 📖 Documentación

| Archivo | Descripción |
|---------|-------------|
| [`CLAUDE.md`](CLAUDE.md) | 🛠️ **Mejores prácticas** y arquitectura técnica |
| [`STATUS.md`](STATUS.md) | 📊 **Estado del proyecto** y hoja de ruta actual |
| [`LEGAL_COMPLIANCE.md`](LEGAL_COMPLIANCE.md) | ⚖️ **Directrices legales** de uso |
| [`KASPERSKY_SOLUTION.md`](KASPERSKY_SOLUTION.md) | 🛡️ **Solución** para falsos positivos |

### Documentación Técnica Completa

- **Arquitectura del sistema** y patrones de código
- **Guía de configuración** avanzada
- **Ejemplos de uso** para todas las características
- **Benchmarks de rendimiento** detallados
- **Hoja de ruta de desarrollo** y estado actual

## 🛠️ Desarrollo

```bash
# Ejecutar pruebas
cargo test

# Ejecutar con registros de depuración
RUST_LOG=debug cargo run -- --roms-dir ./test-roms -vv

# Formateo de código
cargo fmt

# Linting
cargo clippy

# Corregir advertencias automáticamente
cargo clippy --fix --allow-dirty --allow-staged
```

## ⚖️ Uso Legal

**IMPORTANTE**: Esta herramienta está destinada exclusivamente para organizar y catalogar **contenido legal** y **copias de seguridad personales** de juegos que posee físicamente.

### ✅ Uso Permitido
- Organizar ROMs creados desde sus propios cartuchos/discos
- Validar integridad de copias de seguridad personales
- Convertir listas entre sus plataformas
- Investigación académica y preservación cultural

### ❌ Uso Prohibido
- Descargar, compartir o distribuir ROMs con derechos de autor
- Usar con contenido obtenido ilegalmente
- Distribución comercial de ROMs organizados

**Vea [LEGAL_COMPLIANCE.md](LEGAL_COMPLIANCE.md) para directrices completas de uso legal.**

## 📝 Licencia

Este proyecto está licenciado bajo la Licencia MIT - vea el archivo [LICENSE](LICENSE) para detalles.

---

**🎮 RetroArch Fast Playlist Indexer v1.3.0** - Transformando la organización de ROMs desde 2024

**Nota**: Este proyecto no está oficialmente afiliado con RetroArch. Es una herramienta independiente creada para mejorar la experiencia de gestión de ROMs de forma legal y ética.
