# Localisation en Français
app-name = RetroArch Fast Playlist Indexer
app-description = Indexeur universel de ROMs haute performance pour RetroArch

# Commandes
cmd-scan = Scanner les ROMs et générer des listes de lecture
cmd-convert = Convertir une liste de lecture existante vers une autre plateforme
cmd-convert-all = Convertir toutes les listes de lecture d'un répertoire
cmd-watch = Surveiller les répertoires pour les changements
cmd-download-dats = Télécharger les fichiers DAT
cmd-validate = Valider les ROMs contre les fichiers DAT
cmd-deduplicate = Supprimer les ROMs dupliqués
cmd-cache = Gérer le cache CRC32

# Commun
path = Chemin
platform = Plateforme
output = Sortie
threads = Threads
verbose = Détaillé
help = Aide

# Analyse
scanning-roms = Analyse des ROMs...
found-roms = { $count } ROMs trouvés
processing-file = Traitement : { $filename }
extracting-archive = Extraction de l'archive : { $progress }%
calculating-crc32 = Calcul du CRC32
scan-complete = Analyse terminée

# Détection des systèmes
systems-detected = Systèmes détectés :
rom-count = { $count } ROMs
master-playlist = Liste Principale

# Erreurs
error-invalid-path = Chemin invalide : { $path }
error-no-roms-found = Aucun ROM trouvé dans le répertoire : { $path }
error-platform-unsupported = Plateforme non supportée : { $platform }

# Invites interactives
prompt-roms-dir = Sélectionnez les répertoires de ROMs :
prompt-source-platform = Sélectionnez la plateforme source :
prompt-target-platform = Sélectionnez la plateforme cible :
prompt-output-dir = Sélectionnez le répertoire de sortie :
prompt-create-dir = Le répertoire n'existe pas. Le créer ? (o/n)

# Messages de succès
playlists-created = Listes de lecture créées avec succès
cache-cleared = Cache vidé
cache-stats = Statistiques du cache
validation-complete = Validation terminée
deduplication-complete = Déduplication terminée
