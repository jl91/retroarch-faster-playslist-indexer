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

# Системные сообщения
no-system-selected = ⚠️  Ни одна система не выбрана. Выход...
initialization-warning = Предупреждение: Не удалось инициализировать локализацию: {$error}
usage-instruction = Использование: {$command} --roms-dir <ПУТЬ>
error-processing-failed = ❌ {$system}: {$error}
roms-found-summary = 📊 {$count} ROM найдено за {$time}с
archives-detected = 📦 {$count} сжатых файлов обнаружено

# Конфигурация платформ
platforms-configuration = 🔄 Конфигурация платформ
select-source-platform = Выберите { $type } платформу (где вы выполняете):
select-target-platform = Выберите { $type } платформу (где будет использоваться):

# Конфигурация вывода
output-directory-config = 📁 Конфигурация выходного каталога
output-dir-prompt = Введите выходной каталог для плейлистов
create-directory-prompt = Каталог не существует. Создать '{ $path }'?
use-default-output-dir = Использовать выходной каталог по умолчанию './playlists'?
output-directory = └─ Выходной каталог: { $path }

# Конвертация
conversion = 🔄 Конвертация: { $source } → { $target }
playlist-conversion-mode = 🔄 Режим конвертации плейлиста
batch-conversion-mode = 🔄 Режим пакетной конвертации
converting-to = 🎯 Конвертация в: { $platform }
platform-detected = ✅ Платформа обнаружена: { $platform }
detecting-source-platform = 🔍 Обнаружение исходной платформы...

# Статистика и сводки
scan-summary = 📈 Сводка сканирования:
total-roms = ├─ Всего ROM: { $count }
total-roms-found = 🔍 Всего ROM: { $count }
total-roms-stat = ├─ Всего ROM: { $count }
roms-processed = ├─ Обработано ROM: { $count }
archives-found = ├─ Найдено архивов: { $count }
scan-time = └─ Время сканирования: { $time }с
total-cache-entries = ├─ Всего записей: { $count }

# Валидация
validation-total = ├─ Всего: { $count }
validation-valid = ├─ ✅ Действительных: { $count } ({ $percentage }%)
validation-need-rename = ├─ 🔄 Нужно переименовать: { $count }
validation-unknown = ├─ ❓ Неизвестных: { $count }
validation-homebrew = ├─ 🏠 Homebrew/Hack: { $count }
validation-bad-dumps = ├─ ❌ Плохие дампы: { $count }
validation-corrupted = └─ 💥 Поврежденных: { $count }

# Выбор консолей/систем
console-cores-selection = 🎯 Выбор консолей/ядер
available-systems-consoles = Доступные системы/консоли:
select-system-or-finish = Выберите систему для настройки (или 'Завершить выбор' для окончания):
finish-selection = 🏁 Завершить выбор
system-already-selected = ⚠️ Система { $system } уже выбрана!
systems-selected-so-far = Выбрано систем на данный момент: { $count }
system-added = ✅ Система { $system } добавлена!

# Конфигурация конкретной консоли
configuration-for-system = ⚙️ Конфигурация для: { $system }
roms-directory-for-system = Каталог ROM для { $system }
output-directory-for-system = Выходной каталог для плейлистов { $system }
create-output-directory = Создать выходной каталог '{ $path }'?

# Принудительная система
forced-system = 🎯 Принудительная система: { $system }
forced-system-scan = ├─ Принудительная система: { $system }
forcing-rom-to-system = 🎯 Принуждение { $rom } к системе: { $system }

# Режим Watch
watch-mode-active = ✅ Режим мониторинга активен! Нажмите Ctrl+C для остановки...
watch-active-press-ctrl-c = ✅ Мониторинг активен! Нажмите Ctrl+C для остановки...

# Загрузка систем
systems-for-download = 🎯 Системы для загрузки: { $systems }

# Кэш
cache-stats = 📊 Статистика кэша:
cache-size = ├─ Размер кэша: { $size }
cache-entries = ├─ Записи: { $count }
cache-hit-rate = └─ Частота попаданий: { $rate }%
clearing-cache = 🗑️ Очистка кэша...
cache-cleared = ✅ Кэш успешно очищен
cache-system-loaded = └─ { $system }: { $count } записей
cache-total-entries = ├─ Общее количество записей: { $count }
loading-cache = 📦 Загрузка кэша для { $platform }...
saving-cache = 💾 Сохранение кэша для { $platform }...

# Ошибки
error-config-load = ❌ Ошибка загрузки конфигурации: { $error }
error-scanner-create = ❌ Ошибка создания сканера: { $error }
error-getting-roms-dirs = ❌ Ошибка получения каталогов ROM: { $error }
error-getting-platforms = ❌ Ошибка получения платформ: { $error }
error-getting-output-dir = ❌ Ошибка получения выходного каталога: { $error }
error-building-playlists = ❌ Ошибка построения плейлистов: { $error }
error-loading-playlist = ❌ Ошибка загрузки плейлиста: { $error }
error-converting-playlist = ❌ Ошибка конвертации плейлиста: { $error }
error-saving-playlist = ❌ Ошибка сохранения плейлиста: { $error }
error-starting-watch = ❌ Ошибка запуска мониторинга: { $error }
error-downloading-dats = ❌ Ошибка загрузки DAT: { $error }
error-validating-roms = ❌ Ошибка валидации ROM: { $error }
error-deduplicating-roms = ❌ Ошибка удаления дубликатов ROM: { $error }
error-managing-cache = ❌ Ошибка управления кэшем: { $error }
no-lpl-files-found = ⚠️ Файлы .lpl не найдены в указанном каталоге
no-available-systems = ⚠️ Доступные системы не найдены
error-processing-system = ❌ Ошибка обработки системы: { $error }
directory-not-exist-warning = ⚠️ Каталог не существует: { $path }
validation-complete = ✅ Валидация завершена
deduplication-complete = ✅ Дедупликация завершена

# Интерактивные запросы
prompt-roms-dir = Выберите каталоги ROM:
prompt-source-platform = Выберите исходную платформу:
prompt-target-platform = Выберите целевую платформу:
prompt-output-dir = Выберите выходной каталог:
prompt-create-dir = Каталог не существует. Создать? (y/n)

# Сообщения об успехе
processing-all-consoles-complete = 🎉 Обработка всех консолей завершена!
batch-conversion-complete = ✅ Пакетная конвертация завершена:
successful-conversions = ├─ Успешных: { $count }
failed-conversions = ├─ Неудачных: { $count }
report-generated = 📄 Отчет создан: { $path }
