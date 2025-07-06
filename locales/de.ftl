# Deutsche Lokalisierung
app-name = RetroArch Fast Playlist Indexer
app-description = Hochleistungs-Universal-ROM-Indexer für RetroArch

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

# Scannen
scanning-roms = Scanne ROMs...
found-roms = { $count } ROMs gefunden
processing-file = Verarbeite: { $filename }
extracting-archive = Extrahiere Archiv: { $progress }%
calculating-crc32 = Berechne CRC32
scan-complete = Scan vollständig

# Systemerkennung
systems-detected = Erkannte Systeme:
rom-count = { $count } ROMs
master-playlist = Haupt-Wiedergabeliste

# Fehler
error-invalid-path = Ungültiger Pfad: { $path }
error-no-roms-found = Keine ROMs im Verzeichnis gefunden: { $path }
error-platform-unsupported = Nicht unterstützte Plattform: { $platform }

# Interaktive Eingabeaufforderungen
prompt-roms-dir = Wählen Sie die ROM-Verzeichnisse:
prompt-source-platform = Wählen Sie die Quellplattform:
prompt-target-platform = Wählen Sie die Zielplattform:
prompt-output-dir = Wählen Sie das Ausgabeverzeichnis:
prompt-create-dir = Verzeichnis existiert nicht. Erstellen? (j/n)

# Erfolgsmeldungen
playlists-created = Wiedergabelisten erfolgreich erstellt
cache-cleared = Cache geleert
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
