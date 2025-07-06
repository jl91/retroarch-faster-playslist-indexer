# Localización en Español
app-name = RetroArch Fast Playlist Indexer
app-description = Indexador universal de ROMs de alto rendimiento para RetroArch
app-header = 🚀 RetroArch Fast Playlist Indexer v1.3.3
app-separator = ═══════════════════════════════════════════════

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
verbose = Verboso
help = Ayuda
source = origen
target = destino

# Escaneo
scanning-roms = Escaneando ROMs...
found-roms = Encontradas { $count } ROMs
processing-file = Procesando: { $filename }
extracting-archive = Extrayendo archivo: { $progress }%
calculating-crc32 = Calculando CRC32
scan-complete = Escaneo completo
scanning-directory = 📂 Escaneando: { $path }
scanning-directory-progress = 🔍 Escaneando directorio { $current } de { $total }: { $path }
scanning-directory-indexed = 🔍 Escaneando directorio { $current } de { $total }: { $path }

# Detección de sistemas
systems-detected = Sistemas detectados:
rom-count = { $count } ROMs
master-playlist = Lista de reproducción principal
master-playlist-info = └─ roms.lpl (lista principal con { $count } ROMs)

# Configuración de directorios
rom-directories-config = 📂 Configuración de Directorios de ROMs
roms-dir-prompt = Ingrese la ruta del directorio de ROMs
roms-dir-prompt-additional = Ingrese otro directorio de ROMs (o Enter para continuar)
directory-not-found = ⚠️ Directorio no encontrado: { $path }
not-a-directory = ⚠️ No es un directorio: { $path }
directory-added = ✅ Agregado: { $path }
directory-created = ✅ Directorio creado: { $path }
max-directories-reached = ⚠️ Límite máximo de directorios alcanzado
directories-scanned = ├─ Directorios escaneados: { $count }
directories-count = { $count } directorios

# Configuración de plataformas
platforms-configuration = 🔄 Configuración de Plataformas
select-source-platform = Seleccione la plataforma { $type } (donde está ejecutando):
select-target-platform = Seleccione la plataforma { $type } (donde será usado):

# Configuración de salida
output-directory-config = 📁 Configuración del Directorio de Salida
output-dir-prompt = Ingrese el directorio de salida para las listas
create-directory-prompt = El directorio no existe. ¿Crear '{ $path }'?
use-default-output-dir = ¿Usar directorio de salida por defecto './playlists'?
output-directory = └─ Directorio de salida: { $path }

# Conversión
conversion = 🔄 Conversión: { $source } → { $target }
playlist-conversion-mode = 🔄 Modo de Conversión de Lista
batch-conversion-mode = 🔄 Modo de Conversión por Lotes
converting-to = 🎯 Convirtiendo a: { $platform }
platform-detected = ✅ Plataforma detectada: { $platform }
detecting-source-platform = 🔍 Detectando plataforma de origen...

# Estadísticas y resúmenes
scan-summary = 📈 Resumen del Escaneo:
total-roms = ├─ Total de ROMs: { $count }
total-roms-found = 🔍 Total de ROMs: { $count }
total-roms-stat = ├─ Total de ROMs: { $count }
roms-processed = ├─ ROMs procesadas: { $count }
archives-found = ├─ Archivos encontrados: { $count }
scan-time = └─ Tiempo de escaneo: { $time }s
total-cache-entries = ├─ Total de entradas: { $count }
roms-found-summary = 📊 {$count} ROMs encontradas en {$time}s
archives-detected = 📦 {$count} archivos comprimidos detectados

# Validación
validation-total = ├─ Total: { $count }
validation-valid = ├─ ✅ Válidas: { $count } ({ $percentage }%)
validation-need-rename = ├─ 🔄 Necesitan Renombrar: { $count }
validation-unknown = ├─ ❓ Desconocidas: { $count }
validation-homebrew = ├─ 🏠 Homebrew/Hack: { $count }
validation-bad-dumps = ├─ ❌ Bad Dumps: { $count }
validation-corrupted = └─ 💥 Corruptas: { $count }

# Selección de consolas/sistemas
console-cores-selection = 🎯 Selección de Consolas/Cores
available-systems-consoles = Sistemas/Consolas disponibles:
select-system-or-finish = Seleccione un sistema para configurar (o 'Finalizar selección' para terminar)
finish-selection = 🏁 Finalizar selección
system-already-selected = ⚠️ ¡El sistema { $system } ya fue seleccionado!

# Mensajes de caché
cache-cleared = Caché limpiado
cache-stats = Estadísticas de caché
cache-system-loaded = └─ { $system }: { $count } entradas
cache-total-entries = ├─ Total de entradas: { $count }
loading-cache = 📦 Cargando caché para { $platform }...
saving-cache = 💾 Guardando caché para { $platform }...

# Errores
error-invalid-path = Ruta inválida: { $path }
error-no-roms-found = No se encontraron ROMs en el directorio: { $path }
error-platform-unsupported = Plataforma no soportada: { $platform }
error-roms-dir-required = ❌ Error: Al menos un directorio de ROMs debe ser especificado
error-config-load = ❌ Error cargando configuración: { $error }
error-scanner-create = ❌ Error creando escáner: { $error }
error-getting-roms-dirs = ❌ Error obteniendo directorios de ROMs: { $error }
error-getting-platforms = ❌ Error obteniendo plataformas: { $error }
error-getting-output-dir = ❌ Error obteniendo directorio de salida: { $error }
error-building-playlists = ❌ Error construyendo listas: { $error }
error-loading-playlist = ❌ Error cargando lista: { $error }
error-converting-playlist = ❌ Error convirtiendo lista: { $error }
error-saving-playlist = ❌ Error guardando lista: { $error }
error-starting-watch = ❌ Error iniciando monitoreo: { $error }
error-downloading-dats = ❌ Error descargando DATs: { $error }
error-validating-roms = ❌ Error validando ROMs: { $error }
error-deduplicating-roms = ❌ Error eliminando ROMs duplicadas: { $error }
error-managing-cache = ❌ Error gestionando caché: { $error }
no-roms-found = ⚠️ No se encontraron ROMs en los directorios especificados
no-lpl-files-found = ⚠️ No se encontraron archivos .lpl en el directorio especificado

# Prompts interactivos
prompt-roms-dir = Seleccione los directorios de ROMs:
prompt-source-platform = Seleccione la plataforma de origen:
prompt-target-platform = Seleccione la plataforma de destino:
prompt-output-dir = Seleccione el directorio de salida:
prompt-create-dir = El directorio no existe. ¿Crearlo? (s/n)

# Mensajes de éxito
playlists-created = Listas de reproducción creadas exitosamente
indexing-complete = 🎉 ¡Indexación completada exitosamente!
processing-all-consoles-complete = 🎉 ¡Procesamiento de todas las consolas completado!
batch-conversion-complete = ✅ Conversión por lotes completada:
successful-conversions = ├─ Exitosas: { $count }
failed-conversions = ├─ Fallidas: { $count }
report-generated = 📄 Reporte generado: { $path }

# Carga
loading-playlist = 📄 Cargando: { $filename }

# Modos de ejecución
execution-mode = Modo de Ejecución
choose-indexer-execution = Elija el modo de ejecución del indexador:
interactive-mode-console-selection = Modo Interactivo (Selección de Consolas)
interactive-mode-desc = Seleccione sistemas específicos para indexar
automatic-mode-scan-all = Modo Automático (Escanear Todo)
automatic-mode-desc = Detecta e indexa automáticamente todos los sistemas
select-mode = Seleccione el modo

# Mensajes del sistema
no-system-selected = ⚠️  Ningún sistema seleccionado. Saliendo...
initialization-warning = Advertencia: Error al inicializar la localización: {$error}
usage-instruction = Uso: {$command} --roms-dir <RUTA>
error-processing-failed = ❌ {$system}: {$error}
