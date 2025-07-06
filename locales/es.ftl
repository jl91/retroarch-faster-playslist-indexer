# Localización en Español
app-name = RetroArch Fast Playlist Indexer
app-description = Indexador universal de ROMs de alto rendimiento para RetroArch

# Comandos
cmd-scan = Escanear ROMs y generar listas de reproducción
cmd-convert = Convertir lista de reproducción existente a otra plataforma
cmd-convert-all = Convertir todas las listas de reproducción de un directorio
cmd-watch = Monitorear directorios para cambios
cmd-download-dats = Descargar archivos DAT
cmd-validate = Validar ROMs contra archivos DAT
cmd-deduplicate = Eliminar ROMs duplicados
cmd-cache = Gestionar caché CRC32

# Común
path = Ruta
platform = Plataforma
output = Salida
threads = Hilos
verbose = Detallado
help = Ayuda

# Escaneo
scanning-roms = Escaneando ROMs...
found-roms = Encontrados { $count } ROMs
processing-file = Procesando: { $filename }
extracting-archive = Extrayendo archivo: { $progress }%
calculating-crc32 = Calculando CRC32
scan-complete = Escaneo completo

# Detección de sistemas
systems-detected = Sistemas detectados:
rom-count = { $count } ROMs
master-playlist = Lista Principal

# Errores
error-invalid-path = Ruta inválida: { $path }
error-no-roms-found = No se encontraron ROMs en el directorio: { $path }
error-platform-unsupported = Plataforma no soportada: { $platform }

# Prompts interactivos
prompt-roms-dir = Seleccione los directorios de ROMs:
prompt-source-platform = Seleccione la plataforma de origen:
prompt-target-platform = Seleccione la plataforma de destino:
prompt-output-dir = Seleccione el directorio de salida:
prompt-create-dir = El directorio no existe. ¿Crear? (s/n)

# Mensajes de éxito
playlists-created = Listas de reproducción creadas exitosamente
cache-cleared = Caché limpiado
cache-stats = Estadísticas del caché
validation-complete = Validación completa
deduplication-complete = Deduplicación completa
