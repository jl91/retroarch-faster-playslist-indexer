# Localização em Português Brasileiro
app-name = RetroArch Fast Playlist Indexer
app-description = Indexador universal de ROMs de alta performance para RetroArch
app-header = 🚀 RetroArch Fast Playlist Indexer v1.3.3
app-separator = ═══════════════════════════════════════════════

# Comandos
cmd-scan = Escanear ROMs e gerar playlists
cmd-convert = Converter playlist existente para outra plataforma
cmd-convert-all = Converter todas as playlists de um diretório
cmd-watch = Monitorar diretórios para mudanças
cmd-download-dats = Baixar arquivos DAT
cmd-validate = Validar ROMs contra arquivos DAT
cmd-deduplicate = Remover ROMs duplicados
cmd-cache = Gerenciar cache CRC32

# Comum
path = Caminho
platform = Plataforma
output = Saída
threads = Threads
verbose = Verboso
help = Ajuda
source = origem
target = destino

# Escaneamento
scanning-roms = Escaneando ROMs...
found-roms = Encontradas { $count } ROMs
processing-file = Processando: { $filename }
extracting-archive = Extraindo arquivo: { $progress }%
calculating-crc32 = Calculando CRC32
scan-complete = Escaneamento completo
scanning-directory = 📂 Escaneando: { $path }
scanning-directory-progress = 🔍 Escaneando diretório { $current } de { $total }: { $path }
scanning-directory-indexed = 🔍 Escaneando diretório { $current } de { $total }: { $path }

# Detecção de sistemas
systems-detected = Sistemas detectados:
rom-count = { $count } ROMs
master-playlist = Playlist Principal
master-playlist-info = └─ roms.lpl (playlist master com { $count } ROMs)

# Configuração de diretórios
rom-directories-config = 📂 Configuração de Diretórios de ROMs
roms-dir-prompt = Digite o caminho do diretório de ROMs
roms-dir-prompt-additional = Digite outro diretório de ROMs (ou Enter para continuar)
directory-not-found = ⚠️ Diretório não encontrado: { $path }
not-a-directory = ⚠️ Não é um diretório: { $path }
directory-added = ✅ Adicionado: { $path }
directory-created = ✅ Diretório criado: { $path }
max-directories-reached = ⚠️ Limite máximo de diretórios atingido
directories-scanned = ├─ Diretórios escaneados: { $count }
directories-count = { $count } diretórios

# Configuração de plataformas
platforms-configuration = 🔄 Configuração de Plataformas
select-source-platform = Selecione a plataforma { $type } (onde você está executando):
select-target-platform = Selecione a plataforma { $type } (onde será usado):

# Configuração de saída
output-directory-config = 📁 Configuração do Diretório de Saída
output-dir-prompt = Digite o diretório de saída para as playlists
create-directory-prompt = Diretório não existe. Criar '{ $path }'?
use-default-output-dir = Usar diretório de saída padrão './playlists'?
output-directory = └─ Diretório de saída: { $path }

# Conversão
conversion = 🔄 Conversão: { $source } → { $target }
playlist-conversion-mode = 🔄 Modo de Conversão de Playlist
batch-conversion-mode = 🔄 Modo de Conversão em Lote
converting-to = 🎯 Convertendo para: { $platform }
platform-detected = ✅ Plataforma detectada: { $platform }
detecting-source-platform = 🔍 Detectando plataforma de origem...

# Estatísticas e resumos
scan-summary = 📈 Resumo do Escaneamento:
total-roms = ├─ Total de ROMs: { $count }
total-roms-found = 🔍 Total de ROMs: { $count }
total-roms-stat = ├─ Total de ROMs: { $count }
roms-processed = ├─ ROMs processadas: { $count }
archives-found = ├─ Arquivos encontrados: { $count }
scan-time = └─ Tempo de escaneamento: { $time }s
total-cache-entries = ├─ Total de entradas: { $count }
roms-found-summary = 📊 {$count} ROMs encontradas em {$time}s
archives-detected = 📦 {$count} arquivos comprimidos detectados

# Validação
validation-total = ├─ Total: { $count }
validation-valid = ├─ ✅ Válidas: { $count } ({ $percentage }%)
validation-need-rename = ├─ 🔄 Precisam Renomear: { $count }
validation-unknown = ├─ ❓ Desconhecidas: { $count }
validation-homebrew = ├─ 🏠 Homebrew/Hack: { $count }
validation-bad-dumps = ├─ ❌ Bad Dumps: { $count }
validation-corrupted = └─ 💥 Corrompidas: { $count }
validation-complete = ✅ Validação concluída

# Seleção de consoles/sistemas
console-cores-selection = 🎯 Seleção de Consoles/Cores
available-systems-consoles = Sistemas/Consoles disponíveis:
select-system-or-finish = Selecione um sistema para configurar (ou 'Concluir seleção' para finalizar)
finish-selection = 🏁 Concluir seleção
system-already-selected = ⚠️ Sistema { $system } já foi selecionado!
systems-selected-so-far = Sistemas selecionados até agora: { $count }
system-added = ✅ Sistema { $system } adicionado!
no-available-systems = ⚠️ Nenhum sistema disponível encontrado

# Configuração específica de console
configuration-for-system = ⚙️ Configuração para: { $system }
roms-directory-for-system = Diretório de ROMs para { $system }
output-directory-for-system = Diretório de saída para playlists de { $system }
create-output-directory = Criar diretório de saída '{ $path }'?

# Sistema forçado
forced-system = 🎯 Sistema Forçado: { $system }
forced-system-scan = ├─ Sistema Forçado: { $system }
forcing-rom-to-system = 🎯 Forçando { $rom } para sistema: { $system }

# Modos de execução
execution-mode = Modo de Execução
choose-indexer-execution = Escolha o modo de execução do indexador:
interactive-mode-console-selection = Modo Interativo (Seleção de Consoles)
interactive-mode-desc = Selecione sistemas específicos para indexar
automatic-mode-scan-all = Modo Automático (Escanear Tudo)
automatic-mode-desc = Detecta e indexa automaticamente todos os sistemas
select-mode = Selecione o modo

# Watch mode
watch-mode-active = ✅ Modo de vigilância ativo! Pressione Ctrl+C para parar...
watch-active-press-ctrl-c = ✅ Vigilância ativa! Pressione Ctrl+C para parar...

# Download de sistemas
systems-for-download = 🎯 Sistemas para download: { $systems }

# Cache
cache-stats = 📊 Estatísticas do Cache:
cache-size = ├─ Tamanho do cache: { $size }
cache-entries = ├─ Entradas: { $count }
cache-hit-rate = └─ Taxa de acerto: { $rate }%
clearing-cache = 🗑️ Limpando cache...
cache-cleared = ✅ Cache limpo com sucesso
cache-system-loaded = └─ { $system }: { $count } entradas
cache-total-entries = ├─ Total de entradas: { $count }
loading-cache = 📦 Carregando cache para { $platform }...
saving-cache = 💾 Salvando cache para { $platform }...

# Erros
error-invalid-path = Caminho inválido: { $path }
error-no-roms-found = Nenhum ROM encontrado no diretório: { $path }
error-platform-unsupported = Plataforma não suportada: { $platform }
error-roms-dir-required = ❌ Erro: Pelo menos um diretório de ROMs deve ser especificado
error-config-load = ❌ Erro carregando configuração: { $error }
error-scanner-create = ❌ Erro criando scanner: { $error }
error-getting-roms-dirs = ❌ Erro obtendo diretórios de ROMs: { $error }
error-getting-platforms = ❌ Erro obtendo plataformas: { $error }
error-getting-output-dir = ❌ Erro obtendo diretório de saída: { $error }
error-building-playlists = ❌ Erro construindo playlists: { $error }
error-loading-playlist = ❌ Erro carregando playlist: { $error }
error-converting-playlist = ❌ Erro convertendo playlist: { $error }
error-saving-playlist = ❌ Erro salvando playlist: { $error }
error-starting-watch = ❌ Erro iniciando monitoramento: { $error }
error-downloading-dats = ❌ Erro baixando DATs: { $error }
error-validating-roms = ❌ Erro validando ROMs: { $error }
error-deduplicating-roms = ❌ Erro removendo ROMs duplicadas: { $error }
error-managing-cache = ❌ Erro gerenciando cache: { $error }
error-processing-system = ❌ Erro processando sistema: { $error }
directory-not-exist-warning = ⚠️ Diretório não existe: { $path }
no-roms-found = ⚠️ Nenhuma ROM encontrada nos diretórios especificados
no-lpl-files-found = ⚠️ Nenhum arquivo .lpl encontrado no diretório especificado

# Prompts interativos
prompt-roms-dir = Selecione os diretórios de ROMs:
prompt-source-platform = Selecione a plataforma de origem:
prompt-target-platform = Selecione a plataforma de destino:
prompt-output-dir = Selecione o diretório de saída:
prompt-create-dir = Diretório não existe. Criar? (s/n)

# Mensagens de sucesso
playlists-created = Playlists criadas com sucesso
indexing-complete = 🎉 Indexação concluída com sucesso!
processing-all-consoles-complete = 🎉 Processamento de todos os consoles concluído!
batch-conversion-complete = ✅ Conversão em lote concluída:
successful-conversions = ├─ Bem-sucedidas: { $count }
failed-conversions = ├─ Falharam: { $count }
report-generated = 📄 Relatório gerado: { $path }

# Carregamento
loading-playlist = 📄 Carregando: { $filename }

# Mensagens de sistema
no-system-selected = ⚠️  Nenhum sistema selecionado. Saindo...
initialization-warning = Aviso: Falha ao inicializar localização: {$error}
usage-instruction = Use: {$command} --roms-dir <CAMINHO>
error-processing-failed = ❌ {$system}: {$error}
deduplication-complete = Desduplicação concluída
