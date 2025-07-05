#!/usr/bin/env pwsh

# Script de exemplo para testar o RetroArch Fast Indexer
# Execute este script no PowerShell para testar a aplicação

Write-Host "🎮 Testando RetroArch Fast Playlist Indexer" -ForegroundColor Cyan
Write-Host "=" * 50

# Criar diretórios de teste
$testDir = "test-roms"
$outputDir = "test-playlists"

Write-Host "📁 Criando estrutura de teste..."

# Criar diretórios por sistema
@("nes", "snes", "n64", "gba", "genesis") | ForEach-Object {
    $systemDir = Join-Path $testDir $_
    New-Item -ItemType Directory -Path $systemDir -Force | Out-Null
    
    # Criar algumas ROMs de exemplo (arquivos vazios)
    1..3 | ForEach-Object {
        $romName = "Game_$_.$(if ($_ -eq 'nes') { 'nes' } elseif ($_ -eq 'snes') { 'sfc' } elseif ($_ -eq 'n64') { 'z64' } elseif ($_ -eq 'gba') { 'gba' } else { 'md' })"
        $romPath = Join-Path $systemDir $romName
        New-Item -ItemType File -Path $romPath -Force | Out-Null
        Add-Content -Path $romPath -Value "Test ROM data for $romName"
    }
}

Write-Host "✅ Estrutura de teste criada em '$testDir'"

# Mostrar help da aplicação
Write-Host "`n📖 Ajuda da aplicação:"
cargo run --no-default-features -- --help

# Executar teste básico
Write-Host "`n🚀 Executando teste básico..."
Write-Host "Comando: cargo run --no-default-features -- --roms-dir $testDir --output-dir $outputDir --source windows --target switch --dry-run"

try {
    cargo run --no-default-features -- --roms-dir $testDir --output-dir $outputDir --source windows --target switch --dry-run
    Write-Host "✅ Teste executado com sucesso!" -ForegroundColor Green
} catch {
    Write-Host "❌ Erro durante execução: $_" -ForegroundColor Red
}

Write-Host "`n🧹 Limpando arquivos de teste..."
Remove-Item -Path $testDir -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item -Path $outputDir -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "✅ Teste concluído!" -ForegroundColor Green
