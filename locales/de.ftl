# Deutsche Lokalisierung
app-name = RetroArch Fast Playlist Indexer
app-description = Hochleistungs-Universal-ROM-Indexer für RetroArch
app-header = 🚀 RetroArch Fast Playlist Indexer v1.3.3
app-separator = ═══════════════════════════════════════════════

# Befehle
cmd-scan = ROMs scannen und Wiedergabelisten erstellen
cmd-convert = Bestehende Wiedergabeliste zu anderer Plattform konvertieren
cmd-convert-all = Alle Wiedergabelisten eines Verzeichnisses konvertieren
cmd-watch = Verzeichnisse auf Änderungen überwachen
cmd-download-dats = DAT-Dateien herunterladen
cmd-validate = ROMs gegen DAT-Dateien validieren
cmd-deduplicate = Doppelte ROMs entfernen
cmd-cache = CRC32-Cache verwalten

# Allgemein
path = Pfad
platform = Plattform
output = Ausgabe
threads = Threads
verbose = Ausführlich
help = Hilfe
source = quelle
target = ziel

# Scannen
scanning-roms = Scanne ROMs...
found-roms = { $count } ROMs gefunden
processing-file = Verarbeite: { $filename }
extracting-archive = Extrahiere Archiv: { $progress }%
calculating-crc32 = Berechne CRC32
scan-complete = Scan vollständig
scanning-directory = 📂 Scanne: { $path }
scanning-directory-progress = 🔍 Scanne Verzeichnis { $current } von { $total }: { $path }
scanning-directory-indexed = 🔍 Scanne Verzeichnis { $current } von { $total }: { $path }

# Systemerkennung
systems-detected = Erkannte Systeme:
rom-count = { $count } ROMs
master-playlist = Haupt-Wiedergabeliste
master-playlist-info = └─ roms.lpl (Haupt-Wiedergabeliste mit { $count } ROMs)

# Verzeichniskonfiguration
rom-directories-config = 📂 ROM-Verzeichnis-Konfiguration
roms-dir-prompt = ROM-Verzeichnispfad eingeben
roms-dir-prompt-additional = Weiteres ROM-Verzeichnis eingeben (oder Enter zum Fortfahren)
directory-not-found = ⚠️ Verzeichnis nicht gefunden: { $path }
not-a-directory = ⚠️ Ist kein Verzeichnis: { $path }
directory-added = ✅ Hinzugefügt: { $path }
directory-created = ✅ Verzeichnis erstellt: { $path }
max-directories-reached = ⚠️ Maximale Verzeichnisanzahl erreicht
directories-scanned = ├─ Gescannte Verzeichnisse: { $count }
directories-count = { $count } Verzeichnisse

# Plattformkonfiguration
platforms-configuration = 🔄 Plattform-Konfiguration
select-source-platform = Wähle { $type }-Plattform (wo Sie ausführen):
select-target-platform = Wähle { $type }-Plattform (wo verwendet wird):

# Ausgabekonfiguration
output-directory-config = 📁 Ausgabeverzeichnis-Konfiguration
output-dir-prompt = Ausgabeverzeichnis für Wiedergabelisten eingeben
create-directory-prompt = Verzeichnis existiert nicht. '{ $path }' erstellen?
use-default-output-dir = Standard-Ausgabeverzeichnis './playlists' verwenden?
output-directory = └─ Ausgabeverzeichnis: { $path }

# Konvertierung
conversion = 🔄 Konvertierung: { $source } → { $target }
playlist-conversion-mode = 🔄 Wiedergabelisten-Konvertierungsmodus
batch-conversion-mode = 🔄 Stapel-Konvertierungsmodus
converting-to = 🎯 Konvertiere zu: { $platform }
platform-detected = ✅ Plattform erkannt: { $platform }
detecting-source-platform = 🔍 Erkenne Quellplattform...

# Statistiken und Zusammenfassungen
scan-summary = 📈 Scan-Zusammenfassung:
total-roms = ├─ Gesamt ROMs: { $count }
total-roms-found = 🔍 Gesamt ROMs: { $count }
total-roms-stat = ├─ Gesamt ROMs: { $count }
roms-processed = ├─ Verarbeitete ROMs: { $count }
archives-found = ├─ Gefundene Archive: { $count }
scan-time = └─ Scan-Zeit: { $time }s
total-cache-entries = ├─ Gesamt Einträge: { $count }

# Validierung
validation-total = ├─ Gesamt: { $count }
validation-valid = ├─ ✅ Gültig: { $count } ({ $percentage }%)
validation-need-rename = ├─ 🔄 Umbenennung nötig: { $count }
validation-unknown = ├─ ❓ Unbekannt: { $count }
validation-homebrew = ├─ 🏠 Homebrew/Hack: { $count }
validation-bad-dumps = ├─ ❌ Schlechte Dumps: { $count }
validation-corrupted = └─ 💥 Beschädigt: { $count }

# Konsolen-/Systemauswahl
console-cores-selection = 🎯 Konsolen-/Core-Auswahl
available-systems-consoles = Verfügbare Systeme/Konsolen:
select-system-or-finish = System zur Konfiguration auswählen (oder 'Auswahl abschließen' zum Beenden)
finish-selection = 🏁 Auswahl abschließen
system-already-selected = ⚠️ System { $system } bereits ausgewählt!
systems-selected-so-far = Bisher ausgewählte Systeme: { $count }
system-added = ✅ System { $system } hinzugefügt!

# Spezifische Konsolenkonfiguration
configuration-for-system = ⚙️ Konfiguration für: { $system }
roms-directory-for-system = ROM-Verzeichnis für { $system }
output-directory-for-system = Ausgabeverzeichnis für { $system }-Wiedergabelisten
create-output-directory = Ausgabeverzeichnis '{ $path }' erstellen?

# Erzwungenes System
forced-system = 🎯 Erzwungenes System: { $system }
forced-system-scan = ├─ Erzwungenes System: { $system }
forcing-rom-to-system = 🎯 Erzwinge { $rom } für System: { $system }

# Ausführungsmodi
execution-mode = Ausführungsmodus
choose-indexer-execution = Wählen Sie den Indexer-Ausführungsmodus:
interactive-mode-console-selection = Interaktiver Modus (Konsolenauswahl)
interactive-mode-desc = Wählen Sie spezifische Systeme zum Indexieren
automatic-mode-scan-all = Automatischer Modus (Alles scannen)
automatic-mode-desc = Erkennt und indexiert automatisch alle Systeme
select-mode = Modus auswählen

# Watch-Modus
watch-active-press-ctrl-c = ✅ Watch aktiv! Drücken Sie Strg+C zum Stoppen...

# System-Download
systems-for-download = 🎯 Systeme zum Download: { $systems }

# Cache
cache-stats = 📊 Cache-Statistiken:
cache-size = ├─ Cache-Größe: { $size }
cache-entries = ├─ Einträge: { $count }
cache-hit-rate = └─ Trefferrate: { $rate }%
clearing-cache = 🗑️ Leere Cache...
cache-cleared = ✅ Cache erfolgreich geleert

# Fehler
error-invalid-path = Ungültiger Pfad: { $path }
error-no-roms-found = Keine ROMs im Verzeichnis gefunden: { $path }
error-platform-unsupported = Nicht unterstützte Plattform: { $platform }
error-roms-dir-required = ❌ Fehler: Mindestens ein ROM-Verzeichnis muss angegeben werden
error-config-load = ❌ Fehler beim Laden der Konfiguration: { $error }
error-scanner-create = ❌ Fehler beim Erstellen des Scanners: { $error }
error-getting-roms-dirs = ❌ Fehler beim Abrufen der ROM-Verzeichnisse: { $error }
error-getting-platforms = ❌ Fehler beim Abrufen der Plattformen: { $error }
error-getting-output-dir = ❌ Fehler beim Abrufen des Ausgabeverzeichnisses: { $error }
error-building-playlists = ❌ Fehler beim Erstellen der Wiedergabelisten: { $error }
error-loading-playlist = ❌ Fehler beim Laden der Wiedergabeliste: { $error }
error-converting-playlist = ❌ Fehler beim Konvertieren der Wiedergabeliste: { $error }
error-saving-playlist = ❌ Fehler beim Speichern der Wiedergabeliste: { $error }
error-starting-watch = ❌ Fehler beim Starten der Überwachung: { $error }
error-downloading-dats = ❌ Fehler beim Herunterladen der DATs: { $error }
error-validating-roms = ❌ Fehler beim Validieren der ROMs: { $error }
error-deduplicating-roms = ❌ Fehler beim Entfernen doppelter ROMs: { $error }
error-managing-cache = ❌ Fehler beim Verwalten des Caches: { $error }
no-roms-found = ⚠️ Keine ROMs in den angegebenen Verzeichnissen gefunden
no-available-systems = ⚠️ Keine verfügbaren Systeme gefunden
no-lpl-files-found = ⚠️ Keine .lpl-Dateien im angegebenen Verzeichnis gefunden
error-processing-system = ❌ Fehler beim Verarbeiten des Systems: { $error }
directory-not-exist-warning = ⚠️ Verzeichnis existiert nicht: { $path }

# Interaktive Eingabeaufforderungen
prompt-roms-dir = Wählen Sie die ROM-Verzeichnisse:
prompt-source-platform = Wählen Sie die Quellplattform:
prompt-target-platform = Wählen Sie die Zielplattform:
prompt-output-dir = Wählen Sie das Ausgabeverzeichnis:
prompt-create-dir = Verzeichnis existiert nicht. Erstellen? (j/n)

# Erfolgsmeldungen
playlists-created = Wiedergabelisten erfolgreich erstellt
indexing-complete = 🎉 Indexierung erfolgreich abgeschlossen!
processing-all-consoles-complete = 🎉 Verarbeitung aller Konsolen abgeschlossen!
batch-conversion-complete = ✅ Stapel-Konvertierung abgeschlossen:
successful-conversions = ├─ Erfolgreich: { $count }
failed-conversions = ├─ Fehlgeschlagen: { $count }
report-generated = 📄 Bericht erstellt: { $path }

# Laden
loading-playlist = 📄 Lade: { $filename }

# Systemnachrichten
no-system-selected = ⚠️  Kein System ausgewählt. Beenden...
initialization-warning = Warnung: Initialisierung der Lokalisierung fehlgeschlagen: {$error}
usage-instruction = Verwendung: {$command} --roms-dir <PFAD>
error-processing-failed = ❌ {$system}: {$error}
roms-found-summary = 📊 {$count} ROMs in {$time}s gefunden
archives-detected = 📦 {$count} komprimierte Dateien erkannt
cache-stats = Cache-Statistiken
validation-complete = Validierung abgeschlossen
deduplication-complete = Deduplizierung abgeschlossen

# Ausführungsmodi
execution-mode = Ausführungsmodus
choose-indexer-execution = Wählen Sie den Indexer-Ausführungsmodus:
interactive-mode-console-selection = Interaktiver Modus (Konsolenauswahl)
interactive-mode-desc = Wählen Sie spezifische Systeme zum Indexieren
automatic-mode-scan-all = Automatischer Modus (Alles scannen)
automatic-mode-desc = Erkennt und indexiert automatisch alle Systeme
select-mode = Modus wählen

# Systemnachrichten
no-system-selected = ⚠️  Kein System ausgewählt. Beenden...
initialization-warning = Warnung: Initialisierung der Lokalisierung fehlgeschlagen: {$error}
usage-instruction = Verwendung: {$command} --roms-dir <PFAD>
error-processing-failed = ❌ {$system}: {$error}
roms-found-summary = 📊 {$count} ROMs in {$time}s gefunden
archives-detected = 📦 {$count} komprimierte Dateien erkannt
