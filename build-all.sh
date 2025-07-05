#!/bin/bash
# Build script for cross-platform binaries
# RetroArch Fast Playlist Indexer

echo "🏗️  Building RetroArch Fast Playlist Indexer for multiple platforms..."
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Build for Windows x64
echo -e "${GREEN}🏗️  Building for Windows x64...${NC}"
cargo build --release --target x86_64-pc-windows-gnu
if [ $? -eq 0 ]; then
    cp "target/x86_64-pc-windows-gnu/release/retroarch-indexer.exe" "bin/windows/x64/retroarch-indexer.exe"
    echo -e "${GREEN}✅ Windows x64 binary ready${NC}"
else
    echo -e "${YELLOW}❌ Windows x64 build failed (target may not be installed)${NC}"
fi

# Build for Linux x64
echo -e "${GREEN}🏗️  Building for Linux x64...${NC}"
cargo build --release --target x86_64-unknown-linux-gnu
if [ $? -eq 0 ]; then
    cp "target/x86_64-unknown-linux-gnu/release/retroarch-indexer" "bin/linux/x64/retroarch-indexer"
    echo -e "${GREEN}✅ Linux x64 binary ready${NC}"
else
    echo -e "${YELLOW}❌ Linux x64 build failed (target may not be installed)${NC}"
fi

# Build for Linux ARM64
echo -e "${GREEN}🏗️  Building for Linux ARM64...${NC}"
cargo build --release --target aarch64-unknown-linux-gnu
if [ $? -eq 0 ]; then
    cp "target/aarch64-unknown-linux-gnu/release/retroarch-indexer" "bin/linux/arm64/retroarch-indexer"
    echo -e "${GREEN}✅ Linux ARM64 binary ready${NC}"
else
    echo -e "${YELLOW}❌ Linux ARM64 build failed (target may not be installed)${NC}"
fi

# Build for Linux ARMv7
echo -e "${GREEN}🏗️  Building for Linux ARMv7...${NC}"
cargo build --release --target armv7-unknown-linux-gnueabihf
if [ $? -eq 0 ]; then
    cp "target/armv7-unknown-linux-gnueabihf/release/retroarch-indexer" "bin/linux/armv7/retroarch-indexer"
    echo -e "${GREEN}✅ Linux ARMv7 binary ready${NC}"
else
    echo -e "${YELLOW}❌ Linux ARMv7 build failed (target may not be installed)${NC}"
fi

# Build for macOS Intel
echo -e "${GREEN}🏗️  Building for macOS Intel...${NC}"
cargo build --release --target x86_64-apple-darwin
if [ $? -eq 0 ]; then
    cp "target/x86_64-apple-darwin/release/retroarch-indexer" "bin/macos/intel/retroarch-indexer"
    echo -e "${GREEN}✅ macOS Intel binary ready${NC}"
else
    echo -e "${YELLOW}❌ macOS Intel build failed (target may not be installed)${NC}"
fi

# Build for macOS Apple Silicon
echo -e "${GREEN}🏗️  Building for macOS Apple Silicon...${NC}"
cargo build --release --target aarch64-apple-darwin
if [ $? -eq 0 ]; then
    cp "target/aarch64-apple-darwin/release/retroarch-indexer" "bin/macos/apple/retroarch-indexer"
    echo -e "${GREEN}✅ macOS Apple Silicon binary ready${NC}"
else
    echo -e "${YELLOW}❌ macOS Apple Silicon build failed (target may not be installed)${NC}"
fi

echo ""
echo -e "${CYAN}📋 Build Summary:${NC}"
echo "Check bin/ directory for compiled binaries"
echo ""
echo -e "${YELLOW}To install missing targets:${NC}"
echo "rustup target add x86_64-pc-windows-gnu"
echo "rustup target add x86_64-unknown-linux-gnu"
echo "rustup target add aarch64-unknown-linux-gnu"
echo "rustup target add armv7-unknown-linux-gnueabihf"
echo "rustup target add x86_64-apple-darwin"
echo "rustup target add aarch64-apple-darwin"

# Make binaries executable
chmod +x bin/linux/*/retroarch-indexer 2>/dev/null
chmod +x bin/macos/*/retroarch-indexer 2>/dev/null

echo ""
echo -e "${GREEN}🎉 Build process completed!${NC}"
