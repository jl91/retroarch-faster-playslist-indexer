# English localization
app-name = RetroArch Fast Playlist Indexer
app-description = High-performance universal ROM indexer for RetroArch

# Commands
cmd-scan = Scan ROMs and generate playlists
cmd-convert = Convert existing playlist to another platform
cmd-convert-all = Convert all playlists in a directory
cmd-watch = Monitor directories for changes
cmd-download-dats = Download DAT files
cmd-validate = Validate ROMs against DAT files
cmd-deduplicate = Remove duplicate ROMs
cmd-cache = Manage CRC32 cache

# Common
path = Path
platform = Platform
output = Output
threads = Threads
verbose = Verbose
help = Help

# Scanning
scanning-roms = Scanning ROMs...
found-roms = Found { $count } ROMs
processing-file = Processing: { $filename }
extracting-archive = Extracting archive: { $progress }%
calculating-crc32 = Calculating CRC32
scan-complete = Scan complete

# System detection
systems-detected = Systems detected:
rom-count = { $count } ROMs
master-playlist = Master Playlist

# Errors
error-invalid-path = Invalid path: { $path }
error-no-roms-found = No ROMs found in directory: { $path }
error-platform-unsupported = Unsupported platform: { $platform }

# Interactive prompts
prompt-roms-dir = Please select the ROM directories:
prompt-source-platform = Select source platform:
prompt-target-platform = Select target platform:
prompt-output-dir = Select output directory:
prompt-create-dir = Directory doesn't exist. Create it? (y/n)

# Success messages
playlists-created = Playlists created successfully
cache-cleared = Cache cleared
cache-stats = Cache statistics
validation-complete = Validation complete
deduplication-complete = Deduplication complete
