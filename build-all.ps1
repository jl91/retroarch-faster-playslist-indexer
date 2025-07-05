# Build script for cross-platform binaries
# RetroArch Fast Playlist Indexer

# Build for Windows x64
Write-Host "🏗️  Building for Windows x64..." -ForegroundColor Green
cargo build --release --target x86_64-pc-windows-msvc
if ($LASTEXITCODE -eq 0) {
    Copy-Item "target\x86_64-pc-windows-msvc\release\retroarch-indexer.exe" "bin\windows\x64\retroarch-indexer.exe" -Force
    Write-Host "✅ Windows x64 binary ready" -ForegroundColor Green
} else {
    Write-Host "❌ Windows x64 build failed" -ForegroundColor Red
}

# Build for Windows x86 (32-bit)
Write-Host "🏗️  Building for Windows x86..." -ForegroundColor Green
cargo build --release --target i686-pc-windows-msvc
if ($LASTEXITCODE -eq 0) {
    Copy-Item "target\i686-pc-windows-msvc\release\retroarch-indexer.exe" "bin\windows\x86\retroarch-indexer.exe" -Force
    Write-Host "✅ Windows x86 binary ready" -ForegroundColor Green
} else {
    Write-Host "❌ Windows x86 build failed (target may not be installed)" -ForegroundColor Yellow
}

# Build for Linux x64
Write-Host "🏗️  Building for Linux x64..." -ForegroundColor Green
cargo build --release --target x86_64-unknown-linux-gnu
if ($LASTEXITCODE -eq 0) {
    Copy-Item "target\x86_64-unknown-linux-gnu\release\retroarch-indexer" "bin\linux\x64\retroarch-indexer" -Force
    Write-Host "✅ Linux x64 binary ready" -ForegroundColor Green
} else {
    Write-Host "❌ Linux x64 build failed (target may not be installed)" -ForegroundColor Yellow
}

# Build for Linux ARM64
Write-Host "🏗️  Building for Linux ARM64..." -ForegroundColor Green
cargo build --release --target aarch64-unknown-linux-gnu
if ($LASTEXITCODE -eq 0) {
    Copy-Item "target\aarch64-unknown-linux-gnu\release\retroarch-indexer" "bin\linux\arm64\retroarch-indexer" -Force
    Write-Host "✅ Linux ARM64 binary ready" -ForegroundColor Green
} else {
    Write-Host "❌ Linux ARM64 build failed (target may not be installed)" -ForegroundColor Yellow
}

# Build for macOS Intel
Write-Host "🏗️  Building for macOS Intel..." -ForegroundColor Green
cargo build --release --target x86_64-apple-darwin
if ($LASTEXITCODE -eq 0) {
    Copy-Item "target\x86_64-apple-darwin\release\retroarch-indexer" "bin\macos\intel\retroarch-indexer" -Force
    Write-Host "✅ macOS Intel binary ready" -ForegroundColor Green
} else {
    Write-Host "❌ macOS Intel build failed (target may not be installed)" -ForegroundColor Yellow
}

# Build for macOS Apple Silicon
Write-Host "🏗️  Building for macOS Apple Silicon..." -ForegroundColor Green
cargo build --release --target aarch64-apple-darwin
if ($LASTEXITCODE -eq 0) {
    Copy-Item "target\aarch64-apple-darwin\release\retroarch-indexer" "bin\macos\apple\retroarch-indexer" -Force
    Write-Host "✅ macOS Apple Silicon binary ready" -ForegroundColor Green
} else {
    Write-Host "❌ macOS Apple Silicon build failed (target may not be installed)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "📋 Build Summary:" -ForegroundColor Cyan
Write-Host "Check bin/ directory for compiled binaries" -ForegroundColor White
Write-Host ""
Write-Host "To install missing targets:" -ForegroundColor Yellow
Write-Host "rustup target add i686-pc-windows-msvc" -ForegroundColor Gray
Write-Host "rustup target add x86_64-unknown-linux-gnu" -ForegroundColor Gray
Write-Host "rustup target add aarch64-unknown-linux-gnu" -ForegroundColor Gray
Write-Host "rustup target add x86_64-apple-darwin" -ForegroundColor Gray
Write-Host "rustup target add aarch64-apple-darwin" -ForegroundColor Gray
