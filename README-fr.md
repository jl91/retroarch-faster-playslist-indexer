# RetroArch Fast Playlist Indexer v1.3.0

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.3.0-brightgreen.svg)](https://github.com/seu-usuario/retroarch-fast-indexer)
[![Rust Edition](https://img.shields.io/badge/rust_edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

Un indexeur universel de ROMs haute performance écrit en Rust, conçu pour générer des playlists RetroArch (`.lpl`) plus rapidement que le scanner natif.

## 🌍 Langues Supportées / Supported Languages

- 🇺🇸 [English](README.md) (Par défaut)
- 🇧🇷 [Português](README-pt.md)
- 🇪🇸 [Español](README-es.md) 
- 🇫🇷 [Français](README-fr.md)
- 🇩🇪 [Deutsch](README-de.md)
- 🇯🇵 [日本語](README-ja.md)
- 🇷🇺 [Русский](README-ru.md)
- 🇨🇳 [中文](README-zh.md)

## 📋 Historique des Versions

### v1.3.0 (Actuelle) - 05/07/2025 ✅
- **🏗️ Rust Edition 2024**: Migration complète vers Rust 2024
- **🚀 Toutes les Fonctionnalités Implémentées**: Cache, Mode Watch, Support Archives, Téléchargement DAT, Validation, Déduplication
- **🔧 Dépendances Mises à Jour**: Compatibilité complète avec Rust 1.88.0
- **📦 Build de Release Fonctionnel**: Binaire optimisé généré avec succès
- **🛠️ CLI Étendu**: Commandes `watch`, `download-dats`, `validate`, `deduplicate`, `cache`
- **🌍 Système i18n**: Support multilingue basique implémenté
- **⚠️ Politique d'Avertissements**: Toujours corriger les avertissements avant de terminer les tâches

### v1.2.0 - 05/07/2025 ✅
- **🔧 Dépendances Mises à Jour**: Migration vers les dernières versions (Rayon 1.10, Tokio 1.40, Clap 4.5, etc.)
- **📦 Feuille de Route Implémentée**: Toutes les fonctionnalités avancées de la feuille de route v1.1/v1.2
- **🗄️ Cache Persistant**: Système de cache CRC32 pour l'optimisation
- **📦 Support Archives**: ZIP/7z avec fonctionnalités optionnelles
- **👀 Mode Watch**: Surveillance de répertoires en temps réel
- **🌐 Téléchargement DAT**: Téléchargement automatique de No-Intro/Redump
- **✅ Validation**: Système de validation complet via DAT
- **🗂️ Déduplication**: 5 stratégies de déduplication intelligente

### v1.1.0
- ⚡ **Performance Améliorée**: Optimisations de traitement parallèle et gestion de fichiers
- 🧹 **Code Propre**: Réduction des avertissements et imports inutiles
- 🔒 **Stabilité**: 100% compatible avec l'écosystème Rust actuel

### v1.0.0
- 🚀 **Sortie Initiale**: Scanner parallèle basique
- 🔍 **Détection CRC32**: Identification précise des ROMs
- 📋 **Génération de Playlists**: Création de fichiers .lpl
- 🎮 **Multi-plateforme**: Conversion automatique de chemins
- 🏷️ **Support DAT**: Nomenclature via fichiers DAT

> **📊 Statut Détaillé**: Pour des informations complètes sur le développement et l'implémentation, consultez [`STATUS.md`](STATUS.md)

## 🚀 Démarrage Rapide

### 📦 Binaires Précompilés (Recommandé)

Téléchargez le binaire pour votre plateforme depuis le dossier [`bin/`](bin/):

```bash
# Windows x64
.\bin\windows\x64\retroarch-indexer.exe --help

# Linux x64
./bin/linux/x64/retroarch-indexer --help

# macOS Intel
./bin/macos/intel/retroarch-indexer --help

# Vérifier les binaires disponibles
.\check-binaries.ps1  # Windows
./check-binaries.sh   # Linux/macOS
```

### 🔧 Compilation Manuelle

```bash
# Cloner le dépôt
git clone https://github.com/seu-usuario/retroarch-fast-indexer
cd retroarch-fast-indexer

# Compilation basique
cargo build --release

# Compilation avec toutes les fonctionnalités
cargo build --release --all-features

# Compilation avec des fonctionnalités spécifiques
cargo build --release --features archive-support,dat-download

# Exécuter
./target/release/retroarch-indexer --help

# Compiler pour plusieurs plateformes
.\build-all.ps1  # Windows
./build-all.sh   # Linux/macOS
```

### 🚀 Fonctionnalités de Compilation

```toml
# Fonctionnalités disponibles dans Cargo.toml
[features]
default = []
archive-support = ["zip", "sevenz-rust"]     # Support ZIP/7z
dat-download = ["reqwest", "md5", "sha2"]    # Téléchargement automatique DAT
watch-mode = ["notify"]                      # Surveillance de répertoires
checksums = ["md5", "sha2"]                  # Algorithmes supplémentaires
full = ["archive-support", "dat-download", "watch-mode", "checksums"]
```

### Utilisation Basique

```bash
# Scanner les ROMs et créer des playlists
retroarch-indexer --roms-dir /path/to/roms

# Spécifier la langue de l'interface
retroarch-indexer --language fr --roms-dir /path/to/roms

# Convertir une playlist existante
retroarch-indexer convert Nintendo\ 64.lpl --target switch

# Conversion par lots
retroarch-indexer convert-all --input-dir ./playlists --source windows --target switch
```

### Utilisation Avancée (v1.3.3)

```bash
# Surveillance automatique de répertoires
retroarch-indexer watch --roms-dir /path/to/roms --output-dir ./playlists

# Téléchargement automatique de DAT
retroarch-indexer download-dats --systems "Nintendo 64,SNES" --output-dir ./dats

# Validation de ROM via DAT
retroarch-indexer validate --dat-dir ./dats --report validation-report.json

# Déduplication intelligente
retroarch-indexer deduplicate --strategy filename-quality --dry-run

# Gestion du cache
retroarch-indexer cache stats
retroarch-indexer cache clean

# Contrôle des threads et surveillance en temps réel
retroarch-indexer --roms-dir ./roms --threads 8 -v  # 8 threads avec journal détaillé
retroarch-indexer --roms-dir ./roms --threads 2     # Limiter à 2 threads pour SSD lents
```

### 🔧 **Surveillance des Threads en Temps Réel (v1.3.3)**

Le nouveau système montre exactement ce que fait chaque thread :

```
🔍 Scan du répertoire 1 sur 1: ./roms
⠁ [=====>----------------------------------] 2/10 (20%) Traitement du fichier 3 sur 10
⠁ Thread  0 | 🔍 Scan: Super Mario 64.z64
⠁ Thread  1 | 📦 Extraction game.zip (75.2%)
⠁ Thread  2 | 🔢 CRC32: Zelda OoT.z64
⠁ Thread  3 | ✅ Terminé: Mario Kart.z64
```

**Statuts possibles par thread :**
- 🔸 **Inactif**: En attente de travail
- 🔍 **Scan**: Lecture du fichier depuis le disque
- 📦 **Extraction**: Traitement du fichier compressé (avec %)
- 🔢 **CRC32**: Calcul de la somme de contrôle
- ✅ **Terminé**: Fichier traité avec succès
- ❌ **Erreur**: Échec du traitement

## 🌍 Support Multilingue

Le système d'internationalisation (i18n) supporte les langues suivantes :

```bash
# Définir la langue manuellement
retroarch-indexer --language en   # Anglais (par défaut)
retroarch-indexer --language pt   # Portugais
retroarch-indexer --language es   # Espagnol
retroarch-indexer --language fr   # Français
retroarch-indexer --language de   # Allemand
retroarch-indexer --language ja   # Japonais
retroarch-indexer --language ru   # Russe
retroarch-indexer --language zh   # Chinois
```

Le système détecte automatiquement la langue du système d'exploitation et revient à l'anglais si la langue n'est pas disponible.

## ✨ Fonctionnalités (v1.3.0)

- **🚀 15-17x plus rapide** que le scanner natif de RetroArch
- **🔄 Conversion automatique** des chemins entre plateformes
- **📋 Playlist maître unifiée** avec tous les ROMs
- **🎮 Support universel** pour tous les formats de ROM
- **🏷️ Nomenclature intelligente** via fichiers DAT
- **📊 Traitement parallèle optimisé**
- **🌍 Interface multilingue** avec 8 langues supportées

### 🆕 Fonctionnalités Avancées
- **🗄️ Cache Persistant**: Cache CRC32 pour l'optimisation des re-scans
- **📦 Support Archives**: Lecture directe des ROMs dans ZIP/7z
- **👀 Mode Watch**: Surveillance automatique des répertoires
- **🌐 Téléchargement DAT**: Téléchargement automatique de bases de données
- **✅ Validation**: Vérification d'intégrité via DAT
- **🗂️ Déduplication**: Suppression intelligente des ROMs dupliqués

### 📋 Commandes CLI Disponibles
```bash
retroarch-indexer                    # Indexation basique
retroarch-indexer convert            # Conversion de playlists
retroarch-indexer convert-all        # Conversion par lots
retroarch-indexer watch              # Surveillance automatique
retroarch-indexer download-dats      # Téléchargement DAT
retroarch-indexer validate           # Validation de ROM
retroarch-indexer deduplicate        # Suppression de doublons
retroarch-indexer cache              # Gestion du cache
```

## 📖 Documentation

| Fichier | Description |
|---------|-------------|
| [`CLAUDE.md`](CLAUDE.md) | 🛠️ **Meilleures pratiques** et architecture technique |
| [`STATUS.md`](STATUS.md) | 📊 **Statut du projet** et feuille de route actuelle |
| [`LEGAL_COMPLIANCE.md`](LEGAL_COMPLIANCE.md) | ⚖️ **Directives légales** d'utilisation |
| [`KASPERSKY_SOLUTION.md`](KASPERSKY_SOLUTION.md) | 🛡️ **Solution** pour les faux positifs |

### Documentation Technique Complète

- **Architecture du système** et motifs de code
- **Guide de configuration** avancée
- **Exemples d'utilisation** pour toutes les fonctionnalités
- **Benchmarks de performance** détaillés
- **Feuille de route de développement** et statut actuel

## 🛠️ Développement

```bash
# Exécuter les tests
cargo test

# Exécuter avec journaux de débogage
RUST_LOG=debug cargo run -- --roms-dir ./test-roms -vv

# Formatage du code
cargo fmt

# Linting
cargo clippy

# Corriger automatiquement les avertissements
cargo clippy --fix --allow-dirty --allow-staged
```

## ⚖️ Utilisation Légale

**IMPORTANT**: Cet outil est destiné exclusivement à organiser et cataloguer du **contenu légal** et des **sauvegardes personnelles** de jeux que vous possédez physiquement.

### ✅ Utilisation Autorisée
- Organiser des ROMs créés à partir de vos propres cartouches/disques
- Valider l'intégrité des sauvegardes personnelles
- Convertir des playlists entre vos plateformes
- Recherche académique et préservation culturelle

### ❌ Utilisation Interdite
- Télécharger, partager ou distribuer des ROMs protégés par des droits d'auteur
- Utiliser avec du contenu obtenu illégalement
- Distribution commerciale de ROMs organisés

**Voir [LEGAL_COMPLIANCE.md](LEGAL_COMPLIANCE.md) pour des directives complètes d'utilisation légale.**

## 📝 Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour les détails.

---

**🎮 RetroArch Fast Playlist Indexer v1.3.0** - Transformant l'organisation des ROMs depuis 2024

**Note**: Ce projet n'est pas officiellement affilié à RetroArch. C'est un outil indépendant créé pour améliorer l'expérience de gestion des ROMs de manière légale et éthique.
