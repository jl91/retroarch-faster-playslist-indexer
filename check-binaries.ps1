# Quick check script for available binaries
# RetroArch Fast Playlist Indexer

Write-Host "📦 RetroArch Fast Playlist Indexer - Binary Status" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

function Check-Binary {
    param(
        [string]$Platform,
        [string]$Arch,
        [string]$Filename
    )
    
    $Path = "bin\$Platform\$Arch\$Filename"
    
    if (Test-Path $Path) {
        $Size = (Get-Item $Path).Length
        $SizeFormatted = if ($Size -gt 1MB) { 
            "{0:N1} MB" -f ($Size / 1MB) 
        } elseif ($Size -gt 1KB) { 
            "{0:N1} KB" -f ($Size / 1KB) 
        } else { 
            "$Size bytes" 
        }
        
        Write-Host "✅ ${Platform}/${Arch}: " -NoNewline
        Write-Host "Available" -ForegroundColor Green -NoNewline
        Write-Host " ($SizeFormatted)"
        
        # Show version if possible
        try {
            if ($Filename.EndsWith(".exe")) {
                $Version = & $Path --version 2>$null | Select-Object -First 1
                if ($Version) {
                    Write-Host "   Version: $Version" -ForegroundColor Gray
                }
            }
        } catch {
            # Ignore errors
        }
    } else {
        Write-Host "❌ ${Platform}/${Arch}: " -NoNewline
        Write-Host "Not available" -ForegroundColor Red
    }
}

Write-Host "Windows Binaries:" -ForegroundColor Yellow
Check-Binary "windows" "x64" "retroarch-indexer.exe"
Check-Binary "windows" "x86" "retroarch-indexer.exe"

Write-Host ""
Write-Host "Linux Binaries:" -ForegroundColor Yellow
Check-Binary "linux" "x64" "retroarch-indexer"
Check-Binary "linux" "arm64" "retroarch-indexer"
Check-Binary "linux" "armv7" "retroarch-indexer"

Write-Host ""
Write-Host "macOS Binaries:" -ForegroundColor Yellow
Check-Binary "macos" "intel" "retroarch-indexer"
Check-Binary "macos" "apple" "retroarch-indexer"

Write-Host ""
Write-Host "📋 Quick Commands:" -ForegroundColor Cyan
Write-Host "Windows:   .\bin\windows\x64\retroarch-indexer.exe --help" -ForegroundColor White
Write-Host "Linux:     ./bin/linux/x64/retroarch-indexer --help" -ForegroundColor White
Write-Host "macOS:     ./bin/macos/intel/retroarch-indexer --help" -ForegroundColor White
Write-Host ""
Write-Host "🔧 Build all: .\build-all.ps1 (Windows) or ./build-all.sh (Linux/macOS)" -ForegroundColor Gray
