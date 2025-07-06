# English localization
app-name = RetroArch Fast Playlist Indexer
app-description = High-performance universal ROM indexer for RetroArch
app-header = 🚀 RetroArch Fast Playlist Indexer v1.3.3
app-separator = ═══════════════════════════════════════════════

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
source = source
target = target

# Scanning
scanning-roms = Scanning ROMs...
found-roms = Found { $count } ROMs
processing-file = Processing: { $filename }
extracting-archive = Extracting archive: { $progress }%
calculating-crc32 = Calculating CRC32
scan-complete = Scan complete
scanning-directory = 📂 Scanning: { $path }
scanning-directory-progress = 🔍 Scanning directory { $current } of { $total }: { $path }
scanning-directory-indexed = 🔍 Scanning directory { $current } of { $total }: { $path }

# System detection
systems-detected = Systems detected:
rom-count = { $count } ROMs
master-playlist = Master Playlist
master-playlist-info = └─ roms.lpl (master playlist with { $count } ROMs)

# Directory configuration
rom-directories-config = 📂 ROM Directories Configuration
roms-dir-prompt = Enter the ROM directory path
roms-dir-prompt-additional = Enter another ROM directory (or Enter to continue)
directory-not-found = ⚠️ Directory not found: { $path }
not-a-directory = ⚠️ Not a directory: { $path }
directory-added = ✅ Added: { $path }
directory-created = ✅ Directory created: { $path }
max-directories-reached = ⚠️ Maximum directories limit reached
directories-scanned = ├─ Directories scanned: { $count }
directories-count = { $count } directories

# Platform configuration
platforms-configuration = 🔄 Platform Configuration
select-source-platform = Select the { $type } platform (where you are running):
select-target-platform = Select the { $type } platform (where it will be used):

# Output configuration
output-directory-config = 📁 Output Directory Configuration
output-dir-prompt = Enter the output directory for the playlists
create-directory-prompt = Directory doesn't exist. Create '{ $path }'?
use-default-output-dir = Use default output directory './playlists'?
output-directory = └─ Output directory: { $path }

# Conversion
conversion = 🔄 Conversion: { $source } → { $target }
playlist-conversion-mode = 🔄 Playlist Conversion Mode
batch-conversion-mode = 🔄 Batch Conversion Mode
converting-to = 🎯 Converting to: { $platform }
platform-detected = ✅ Platform detected: { $platform }
detecting-source-platform = 🔍 Detecting source platform...

# Statistics and summaries
scan-summary = 📈 Scan Summary:
total-roms = ├─ Total ROMs: { $count }
total-roms-found = 🔍 Total ROMs: { $count }
total-roms-stat = ├─ Total ROMs: { $count }
roms-processed = ├─ ROMs processed: { $count }
archives-found = ├─ Archives found: { $count }
scan-time = └─ Scan time: { $time }s
total-cache-entries = ├─ Total entries: { $count }

# Validation
validation-total = ├─ Total: { $count }
validation-valid = ├─ ✅ Valid: { $count } ({ $percentage }%)
validation-need-rename = ├─ 🔄 Need Rename: { $count }
validation-unknown = ├─ ❓ Unknown: { $count }
validation-homebrew = ├─ 🏠 Homebrew/Hack: { $count }
validation-bad-dumps = ├─ ❌ Bad Dumps: { $count }
validation-corrupted = └─ 💥 Corrupted: { $count }

# Console/system selection
console-cores-selection = 🎯 Console/Core Selection
available-systems-consoles = Available systems/consoles:
select-system-or-finish = Select a system to configure (or 'Finish selection' to finalize)
finish-selection = 🏁 Finish selection
system-already-selected = ⚠️ System { $system } has already been selected!

# Cache messages
cache-cleared = Cache cleared
cache-stats = Cache statistics
cache-system-loaded = └─ { $system }: { $count } entries
cache-total-entries = ├─ Total entries: { $count }
loading-cache = 📦 Loading cache for { $platform }...
saving-cache = 💾 Saving cache for { $platform }...

# Errors
error-invalid-path = Invalid path: { $path }
error-no-roms-found = No ROMs found in directory: { $path }
error-platform-unsupported = Unsupported platform: { $platform }
error-roms-dir-required = ❌ Error: At least one ROM directory must be specified
error-config-load = ❌ Error loading configuration: { $error }
error-scanner-create = ❌ Error creating scanner: { $error }
error-getting-roms-dirs = ❌ Error getting ROM directories: { $error }
error-getting-platforms = ❌ Error getting platforms: { $error }
error-getting-output-dir = ❌ Error getting output directory: { $error }
error-building-playlists = ❌ Error building playlists: { $error }
error-loading-playlist = ❌ Error loading playlist: { $error }
error-converting-playlist = ❌ Error converting playlist: { $error }
error-saving-playlist = ❌ Error saving playlist: { $error }
error-starting-watch = ❌ Error starting watch: { $error }
error-downloading-dats = ❌ Error downloading DATs: { $error }
error-validating-roms = ❌ Error validating ROMs: { $error }
error-deduplicating-roms = ❌ Error deduplicating ROMs: { $error }
error-managing-cache = ❌ Error managing cache: { $error }
no-roms-found = ⚠️ No ROMs found in the specified directories
no-lpl-files-found = ⚠️ No .lpl files found in the specified directory

# Interactive prompts
prompt-roms-dir = Select the ROM directories:
prompt-source-platform = Select source platform:
prompt-target-platform = Select target platform:
prompt-output-dir = Select output directory:
prompt-create-dir = Directory doesn't exist. Create it? (y/n)

# Success messages
playlists-created = Playlists created successfully
indexing-complete = 🎉 Indexing completed successfully!
processing-all-consoles-complete = 🎉 Processing of all consoles completed!
batch-conversion-complete = ✅ Batch conversion completed:
successful-conversions = ├─ Successful: { $count }
failed-conversions = ├─ Failed: { $count }
report-generated = 📄 Report generated: { $path }

# Loading
loading-playlist = 📄 Loading: { $filename }
validation-complete = Validation complete
deduplication-complete = Deduplication complete

# Execution modes
execution-mode = Execution Mode
choose-indexer-execution = Choose indexer execution mode:
interactive-mode-console-selection = Interactive Mode (Console Selection)
interactive-mode-desc = Select specific systems to index
automatic-mode-scan-all = Automatic Mode (Scan All)
automatic-mode-desc = Automatically detects and indexes all systems
select-mode = Select mode

# System messages
no-system-selected = ⚠️  No system selected. Exiting...
initialization-warning = Warning: Failed to initialize localization: {$error}
usage-instruction = Use: {$command} --roms-dir <PATH>
error-processing-failed = ❌ {$system}: {$error}
roms-found-summary = 📊 {$count} ROMs found in {$time}s
archives-detected = 📦 {$count} compressed files detected

# Watch mode
watch-mode-active = ✅ Watch mode active! Press Ctrl+C to stop...
watch-active-press-ctrl-c = ✅ Watch active! Press Ctrl+C to stop...

# System download
systems-for-download = 🎯 Systems for download: { $systems }
systems-selected-so-far = Systems selected so far: { $count }
system-added = ✅ System { $system } added!

# Specific console configuration
configuration-for-system = ⚙️ Configuration for: { $system }
roms-directory-for-system = ROM directory for { $system }
output-directory-for-system = Output directory for { $system } playlists
create-output-directory = Create output directory '{ $path }'?

# Forced system
forced-system = 🎯 Forced System: { $system }
forced-system-scan = ├─ Forced System: { $system }
forcing-rom-to-system = 🎯 Forcing { $rom } to system: { $system }

# Cache
cache-size = ├─ Cache size: { $size }
cache-entries = ├─ Entries: { $count }
cache-hit-rate = └─ Hit rate: { $rate }%
clearing-cache = 🗑️ Clearing cache...

# Additional messages
no-available-systems = ⚠️ No available systems found
error-processing-system = ❌ Error processing system: { $error }
directory-not-exist-warning = ⚠️ Directory does not exist: { $path }
