# Localisation en Français
app-name = RetroArch Fast Playlist Indexer
app-description = Indexeur universel de ROMs haute performance pour RetroArch
app-header = 🚀 RetroArch Fast Playlist Indexer v1.3.3
app-separator = ═══════════════════════════════════════════════

# Commandes
cmd-scan = Scanner les ROMs et générer des listes de lecture
cmd-convert = Convertir une liste de lecture existante vers une autre plateforme
cmd-convert-all = Convertir toutes les listes de lecture d'un répertoire
cmd-watch = Surveiller les répertoires pour les changements
cmd-download-dats = Télécharger les fichiers DAT
cmd-validate = Valider les ROMs contre les fichiers DAT
cmd-deduplicate = Supprimer les ROMs dupliquées
cmd-cache = Gérer le cache CRC32

# Commun
path = Chemin
platform = Plateforme
output = Sortie
threads = Threads
verbose = Verbeux
help = Aide
source = source
target = cible

# Scan
scanning-roms = Scan des ROMs...
found-roms = { $count } ROMs trouvées
processing-file = Traitement : { $filename }
extracting-archive = Extraction d'archive : { $progress }%
calculating-crc32 = Calcul CRC32
scan-complete = Scan terminé
scanning-directory = 📂 Scan : { $path }
scanning-directory-progress = 🔍 Scan du répertoire { $current } sur { $total } : { $path }
scanning-directory-indexed = 🔍 Scan du répertoire { $current } sur { $total } : { $path }

# Détection de systèmes
systems-detected = Systèmes détectés :
rom-count = { $count } ROMs
master-playlist = Liste de lecture principale
master-playlist-info = └─ roms.lpl (liste principale avec { $count } ROMs)

# Configuration des répertoires
rom-directories-config = 📂 Configuration des Répertoires de ROMs
roms-dir-prompt = Entrez le chemin du répertoire de ROMs
roms-dir-prompt-additional = Entrez un autre répertoire de ROMs (ou Entrée pour continuer)
directory-not-found = ⚠️ Répertoire non trouvé : { $path }
not-a-directory = ⚠️ N'est pas un répertoire : { $path }
directory-added = ✅ Ajouté : { $path }
directory-created = ✅ Répertoire créé : { $path }
max-directories-reached = ⚠️ Limite maximale de répertoires atteinte
directories-scanned = ├─ Répertoires scannés : { $count }
directories-count = { $count } répertoires

# Configuration des plateformes
platforms-configuration = 🔄 Configuration des Plateformes
select-source-platform = Sélectionnez la plateforme { $type } (où vous exécutez) :
select-target-platform = Sélectionnez la plateforme { $type } (où cela sera utilisé) :

# Configuration de sortie
output-directory-config = 📁 Configuration du Répertoire de Sortie
output-dir-prompt = Entrez le répertoire de sortie pour les listes
create-directory-prompt = Le répertoire n'existe pas. Créer '{ $path }' ?
use-default-output-dir = Utiliser le répertoire de sortie par défaut './playlists' ?
output-directory = └─ Répertoire de sortie : { $path }

# Conversion
conversion = 🔄 Conversion : { $source } → { $target }
playlist-conversion-mode = 🔄 Mode de Conversion de Liste
batch-conversion-mode = 🔄 Mode de Conversion par Lots
converting-to = 🎯 Conversion vers : { $platform }
platform-detected = ✅ Plateforme détectée : { $platform }
detecting-source-platform = 🔍 Détection de la plateforme source...

# Statistiques et résumés
scan-summary = 📈 Résumé du Scan :
total-roms = ├─ Total de ROMs : { $count }
total-roms-found = 🔍 Total de ROMs : { $count }
total-roms-stat = ├─ Total de ROMs : { $count }
roms-processed = ├─ ROMs traitées : { $count }
archives-found = ├─ Archives trouvées : { $count }
scan-time = └─ Temps de scan : { $time }s
total-cache-entries = ├─ Total d'entrées : { $count }

# Validation
validation-total = ├─ Total : { $count }
validation-valid = ├─ ✅ Valides : { $count } ({ $percentage }%)
validation-need-rename = ├─ 🔄 À Renommer : { $count }
validation-unknown = ├─ ❓ Inconnues : { $count }
validation-homebrew = ├─ 🏠 Homebrew/Hack : { $count }
validation-bad-dumps = ├─ ❌ Bad Dumps : { $count }
validation-corrupted = └─ 💥 Corrompues : { $count }
validation-complete = ✅ Validation terminée

# Sélection de consoles/systèmes
console-cores-selection = 🎯 Sélection de Consoles/Cores
available-systems-consoles = Systèmes/Consoles disponibles :
select-system-or-finish = Sélectionnez un système à configurer (ou 'Terminer la sélection' pour finaliser)
finish-selection = 🏁 Terminer la sélection
system-already-selected = ⚠️ Le système { $system } a déjà été sélectionné !

# Messages de cache
cache-cleared = Cache vidé
cache-stats = Statistiques du cache
cache-system-loaded = └─ { $system } : { $count } entrées
cache-total-entries = ├─ Total des entrées : { $count }
loading-cache = 📦 Chargement du cache pour { $platform }...
saving-cache = 💾 Sauvegarde du cache pour { $platform }...
watch-mode-active = ✅ Mode surveillance actif ! Appuyez sur Ctrl+C pour arrêter...

# Erreurs
error-invalid-path = Chemin invalide : { $path }
error-no-roms-found = Aucune ROM trouvée dans le répertoire : { $path }
error-platform-unsupported = Plateforme non supportée : { $platform }
error-roms-dir-required = ❌ Erreur : Au moins un répertoire de ROMs doit être spécifié
error-config-load = ❌ Erreur lors du chargement de la configuration : { $error }
error-scanner-create = ❌ Erreur lors de la création du scanner : { $error }
error-getting-roms-dirs = ❌ Erreur lors de l'obtention des répertoires de ROMs : { $error }
error-getting-platforms = ❌ Erreur lors de l'obtention des plateformes : { $error }
error-getting-output-dir = ❌ Erreur lors de l'obtention du répertoire de sortie : { $error }
error-building-playlists = ❌ Erreur lors de la construction des listes : { $error }
error-loading-playlist = ❌ Erreur lors du chargement de la liste : { $error }
error-converting-playlist = ❌ Erreur lors de la conversion de la liste : { $error }
error-saving-playlist = ❌ Erreur lors de la sauvegarde de la liste : { $error }
error-starting-watch = ❌ Erreur lors du démarrage de la surveillance : { $error }
error-downloading-dats = ❌ Erreur lors du téléchargement des DATs : { $error }
error-validating-roms = ❌ Erreur lors de la validation des ROMs : { $error }
error-deduplicating-roms = ❌ Erreur lors de la suppression des ROMs dupliquées : { $error }
error-managing-cache = ❌ Erreur lors de la gestion du cache : { $error }
error-processing-system = ❌ Erreur lors du traitement du système : { $error }
directory-not-exist-warning = ⚠️ Le répertoire n'existe pas : { $path }
no-roms-found = ⚠️ Aucune ROM trouvée dans les répertoires spécifiés
no-lpl-files-found = ⚠️ Aucun fichier .lpl trouvé dans le répertoire spécifié
no-available-systems = ⚠️ Aucun système disponible trouvé

# Prompts interactifs
prompt-roms-dir = Sélectionnez les répertoires de ROMs :
prompt-source-platform = Sélectionnez la plateforme source :
prompt-target-platform = Sélectionnez la plateforme cible :
prompt-output-dir = Sélectionnez le répertoire de sortie :
prompt-create-dir = Le répertoire n'existe pas. Le créer ? (o/n)

# Messages de succès
playlists-created = Listes de lecture créées avec succès
indexing-complete = 🎉 Indexation terminée avec succès !
processing-all-consoles-complete = 🎉 Traitement de toutes les consoles terminé !
batch-conversion-complete = ✅ Conversion par lots terminée :
successful-conversions = ├─ Réussies : { $count }
failed-conversions = ├─ Échouées : { $count }
report-generated = 📄 Rapport généré : { $path }

# Chargement
loading-playlist = 📄 Chargement : { $filename }

# Modes d'exécution
execution-mode = Mode d'Exécution
choose-indexer-execution = Choisissez le mode d'exécution de l'indexeur :
interactive-mode-console-selection = Mode Interactif (Sélection de Consoles)
interactive-mode-desc = Sélectionnez des systèmes spécifiques à indexer
automatic-mode-scan-all = Mode Automatique (Scanner Tout)
automatic-mode-desc = Détecte et indexe automatiquement tous les systèmes
select-mode = Sélectionnez le mode

# Messages système
no-system-selected = ⚠️  Aucun système sélectionné. Sortie...
initialization-warning = Avertissement: Échec de l'initialisation de la localisation: {$error}
usage-instruction = Utilisation: {$command} --roms-dir <CHEMIN>
error-processing-failed = ❌ {$system}: {$error}
roms-found-summary = 📊 {$count} ROMs trouvées en {$time}s
archives-detected = 📦 {$count} fichiers compressés détectés
systems-selected-so-far = Systèmes sélectionnés jusqu'à présent : { $count }
system-added = ✅ Système { $system } ajouté !

# Configuration spécifique de console
configuration-for-system = ⚙️ Configuration pour : { $system }
roms-directory-for-system = Répertoire ROM pour { $system }
output-directory-for-system = Répertoire de sortie pour les listes de lecture { $system }
create-output-directory = Créer le répertoire de sortie '{ $path }' ?

# Système forcé
forced-system = 🎯 Système Forcé : { $system }
forced-system-scan = ├─ Système Forcé : { $system }
forcing-rom-to-system = 🎯 Forçage de { $rom } vers le système : { $system }

# Mode Watch
watch-active-press-ctrl-c = ✅ Surveillance active ! Appuyez sur Ctrl+C pour arrêter...

# Téléchargement de systèmes
systems-for-download = 🎯 Systèmes à télécharger : { $systems }

# Cache
cache-size = ├─ Taille du cache : { $size }
cache-entries = ├─ Entrées : { $count }
cache-hit-rate = └─ Taux de réussite : { $rate }%
clearing-cache = 🗑️ Nettoyage du cache...
deduplication-complete = Déduplication terminée
