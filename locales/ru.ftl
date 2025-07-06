# Русская локализация
app-name = RetroArch Fast Playlist Indexer
app-description = Высокопроизводительный универсальный индексатор ROM для RetroArch

# Команды
cmd-scan = Сканировать ROM и создать плейлисты
cmd-convert = Конвертировать существующий плейлист для другой платформы
cmd-convert-all = Конвертировать все плейлисты в каталоге
cmd-watch = Отслеживать изменения в каталогах
cmd-download-dats = Загрузить DAT файлы
cmd-validate = Проверить ROM против DAT файлов
cmd-deduplicate = Удалить дублирующиеся ROM
cmd-cache = Управлять CRC32 кэшем

# Общее
path = Путь
platform = Платформа
output = Вывод
threads = Потоки
verbose = Подробно
help = Справка

# Сканирование
scanning-roms = Сканирование ROM...
found-roms = Найдено { $count } ROM
processing-file = Обработка: { $filename }
extracting-archive = Извлечение архива: { $progress }%
calculating-crc32 = Вычисление CRC32
scan-complete = Сканирование завершено

# Обнаружение систем
systems-detected = Обнаружены системы:
rom-count = { $count } ROM
master-playlist = Главный Плейлист

# Ошибки
error-invalid-path = Неверный путь: { $path }
error-no-roms-found = ROM не найдены в каталоге: { $path }
error-platform-unsupported = Неподдерживаемая платформа: { $platform }

# Интерактивные запросы
prompt-roms-dir = Выберите каталоги ROM:
prompt-source-platform = Выберите исходную платформу:
prompt-target-platform = Выберите целевую платформу:
prompt-output-dir = Выберите выходной каталог:
prompt-create-dir = Каталог не существует. Создать? (д/н)

# Сообщения об успехе
playlists-created = Плейлисты успешно созданы
cache-cleared = Кэш очищен
cache-stats = Статистика кэша
validation-complete = Проверка завершена
deduplication-complete = Дедупликация завершена
