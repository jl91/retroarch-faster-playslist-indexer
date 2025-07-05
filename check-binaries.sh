#!/bin/bash
# Quick check script for available binaries
# RetroArch Fast Playlist Indexer

echo "📦 RetroArch Fast Playlist Indexer - Binary Status"
echo "=================================================="
echo

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

check_binary() {
    local platform=$1
    local arch=$2
    local filename=$3
    local path="bin/$platform/$arch/$filename"
    
    if [ -f "$path" ]; then
        local size=$(ls -lh "$path" | awk '{print $5}')
        echo -e "✅ $platform/$arch: ${GREEN}Available${NC} ($size)"
        
        # Show version if possible
        if [[ "$filename" == *.exe ]]; then
            # Windows binary - can't execute on Linux/macOS
            echo "   (Version check requires Windows)"
        else
            if [ -x "$path" ]; then
                local version=$(./"$path" --version 2>/dev/null | head -1)
                if [ ! -z "$version" ]; then
                    echo "   Version: $version"
                fi
            fi
        fi
    else
        echo -e "❌ $platform/$arch: ${RED}Not available${NC}"
    fi
}

echo "Windows Binaries:"
check_binary "windows" "x64" "retroarch-indexer.exe"
check_binary "windows" "x86" "retroarch-indexer.exe"

echo
echo "Linux Binaries:"
check_binary "linux" "x64" "retroarch-indexer"
check_binary "linux" "arm64" "retroarch-indexer"
check_binary "linux" "armv7" "retroarch-indexer"

echo
echo "macOS Binaries:"
check_binary "macos" "intel" "retroarch-indexer"
check_binary "macos" "apple" "retroarch-indexer"

echo
echo "📋 Quick Commands:"
echo "Windows:   .\\bin\\windows\\x64\\retroarch-indexer.exe --help"
echo "Linux:     ./bin/linux/x64/retroarch-indexer --help"
echo "macOS:     ./bin/macos/intel/retroarch-indexer --help"
echo
echo "🔧 Build all: ./build-all.sh (Linux/macOS) or .\\build-all.ps1 (Windows)"
