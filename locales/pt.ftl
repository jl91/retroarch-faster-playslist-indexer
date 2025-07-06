# Localização em Português Brasileiro
app-name = RetroArch Fast Playlist Indexer
app-description = Indexador universal de ROMs de alta performance para RetroArch

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

# Escaneamento
scanning-roms = Escaneando ROMs...
found-roms = Encontrados { $count } ROMs
processing-file = Processando: { $filename }
extracting-archive = Extraindo arquivo: { $progress }%
calculating-crc32 = Calculando CRC32
scan-complete = Escaneamento completo

# Detecção de sistemas
systems-detected = Sistemas detectados:
rom-count = { $count } ROMs
master-playlist = Playlist Principal

# Erros
error-invalid-path = Caminho inválido: { $path }
error-no-roms-found = Nenhum ROM encontrado no diretório: { $path }
error-platform-unsupported = Plataforma não suportada: { $platform }

# Prompts interativos
prompt-roms-dir = Selecione os diretórios de ROMs:
prompt-source-platform = Selecione a plataforma de origem:
prompt-target-platform = Selecione a plataforma de destino:
prompt-output-dir = Selecione o diretório de saída:
prompt-create-dir = Diretório não existe. Criar? (s/n)

# Mensagens de sucesso
playlists-created = Playlists criadas com sucesso
cache-cleared = Cache limpo
cache-stats = Estatísticas do cache
validation-complete = Validação completa
deduplication-complete = Deduplicação completa
