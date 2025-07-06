# Русская локализация
app-name = RetroArch Fast Playlist Indexer
app-description = Высокопроизводительный универсальный индексатор ROM для RetroArch
app-header = 🚀 RetroArch Fast Playlist Indexer v1.3.3
app-separator = ═══════════════════════════════════════════════

# Режимы выполнения
execution-mode = Режим Выполнения
choose-indexer-execution = Выберите режим выполнения индексатора:
interactive-mode-console-selection = Интерактивный Режим (Выбор Консолей)
interactive-mode-desc = Выберите конкретные системы для индексации
automatic-mode-scan-all = Автоматический Режим (Сканировать Все)
automatic-mode-desc = Автоматически обнаруживает и индексирует все системы
select-mode = Выберите режим

# Команды
cmd-scan = Сканировать ROM и создать плейлисты
cmd-convert = Конвертировать существующий плейлист для другой платформы
cmd-convert-all = Конвертировать все плейлисты в каталоге
cmd-watch = Отслеживать изменения в каталогах
cmd-download-dats = Загрузить DAT файлы
cmd-validate = Проверить ROM против DAT файлов
cmd-deduplicate = Удалить дублирующиеся ROM
cmd-cache = Управлять кэшем CRC32

# Общие
path = Путь
platform = Платформа
output = Вывод
threads = Потоки
verbose = Подробный
help = Справка
source = источник
target = цель

# Сканирование
scanning-roms = Сканирование ROM...
found-roms = Найдено { $count } ROM
processing-file = Обработка: { $filename }
extracting-archive = Извлечение архива: { $progress }%
calculating-crc32 = Вычисление CRC32
scan-complete = Сканирование завершено
scanning-directory = 📂 Сканирование: { $path }
scanning-directory-progress = 🔍 Сканирование каталога { $current } из { $total }: { $path }
scanning-directory-indexed = 🔍 Сканирование каталога { $current } из { $total }: { $path }

# Обнаружение систем
systems-detected = Обнаружены системы:
rom-count = { $count } ROM
master-playlist = Главный Плейлист
master-playlist-info = └─ roms.lpl (главный плейлист с { $count } ROM)

# Настройка каталогов
rom-directories-config = 📂 Настройка Каталогов ROM
roms-dir-prompt = Введите путь к каталогу ROM
roms-dir-prompt-additional = Введите другой каталог ROM (или Enter для продолжения)
directory-not-found = ⚠️ Каталог не найден: { $path }
not-a-directory = ⚠️ Не каталог: { $path }
directory-added = ✅ Добавлен: { $path }
directory-created = ✅ Каталог создан: { $path }
max-directories-reached = ⚠️ Достигнут максимум каталогов
directories-scanned = ├─ Просканированных каталогов: { $count }
directories-count = { $count } каталогов

# Ошибки
error-invalid-path = Неверный путь: { $path }
error-no-roms-found = ROM не найдены в каталоге: { $path }
error-platform-unsupported = Неподдерживаемая платформа: { $platform }
error-roms-dir-required = ❌ Ошибка: Необходимо указать хотя бы один каталог ROM
no-roms-found = ⚠️ ROM не найдены в указанных каталогах

# Сообщения об успехе
playlists-created = Плейлисты успешно созданы
indexing-complete = 🎉 Индексация успешно завершена!

# Загрузка
loading-playlist = 📄 Загрузка: { $filename }
