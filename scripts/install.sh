#!/bin/bash
set -e

# Installation script for Engram CLI
# Usage: curl -fsSL https://raw.githubusercontent.com/blackfall-labs/engram-cli/main/scripts/install.sh | bash

REPO="blackfall-labs/engram-cli"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
BINARY_NAME="engram"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}===================================${NC}"
echo -e "${BLUE}   Engram CLI Installer${NC}"
echo -e "${BLUE}===================================${NC}"
echo ""

# Detect OS and Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        PLATFORM="Linux"
        ;;
    Darwin*)
        PLATFORM="Darwin"
        ;;
    *)
        echo -e "${RED}Unsupported operating system: $OS${NC}"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64|amd64)
        ARCH_NAME="x86_64"
        ;;
    aarch64|arm64)
        ARCH_NAME="aarch64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

# Determine binary name
if [[ "$PLATFORM" == "Linux" ]]; then
    # Prefer MUSL build for Linux (static, no dependencies)
    if [[ "$ARCH_NAME" == "x86_64" ]]; then
        BINARY_FILE="engram-Linux-x86_64-musl"
    else
        BINARY_FILE="engram-Linux-${ARCH_NAME}"
    fi
else
    BINARY_FILE="engram-${PLATFORM}-${ARCH_NAME}"
fi

echo -e "${YELLOW}Detected platform: $PLATFORM $ARCH_NAME${NC}"
echo -e "${YELLOW}Binary to download: $BINARY_FILE${NC}"
echo ""

# Get latest release
echo -e "${BLUE}Fetching latest release...${NC}"
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [[ -z "$LATEST_RELEASE" ]]; then
    echo -e "${RED}Failed to fetch latest release${NC}"
    exit 1
fi

echo -e "${GREEN}Latest version: $LATEST_RELEASE${NC}"
echo ""

# Download URL
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/$BINARY_FILE"
CHECKSUM_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/$BINARY_FILE.sha256"

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download binary
echo -e "${BLUE}Downloading $BINARY_FILE...${NC}"
if command -v curl &> /dev/null; then
    curl -fsSL -o "/tmp/$BINARY_NAME" "$DOWNLOAD_URL"
elif command -v wget &> /dev/null; then
    wget -q -O "/tmp/$BINARY_NAME" "$DOWNLOAD_URL"
else
    echo -e "${RED}Neither curl nor wget found. Please install one of them.${NC}"
    exit 1
fi

# Download checksum
echo -e "${BLUE}Downloading checksum...${NC}"
if command -v curl &> /dev/null; then
    curl -fsSL -o "/tmp/$BINARY_NAME.sha256" "$CHECKSUM_URL"
elif command -v wget &> /dev/null; then
    wget -q -O "/tmp/$BINARY_NAME.sha256" "$CHECKSUM_URL"
fi

# Verify checksum
if [[ -f "/tmp/$BINARY_NAME.sha256" ]]; then
    echo -e "${BLUE}Verifying checksum...${NC}"
    EXPECTED_HASH=$(cat "/tmp/$BINARY_NAME.sha256")

    if command -v sha256sum &> /dev/null; then
        ACTUAL_HASH=$(sha256sum "/tmp/$BINARY_NAME" | awk '{print $1}')
    elif command -v shasum &> /dev/null; then
        ACTUAL_HASH=$(shasum -a 256 "/tmp/$BINARY_NAME" | awk '{print $1}')
    else
        echo -e "${YELLOW}Warning: No SHA-256 tool found, skipping verification${NC}"
        ACTUAL_HASH="$EXPECTED_HASH"
    fi

    if [[ "$EXPECTED_HASH" == "$ACTUAL_HASH" ]]; then
        echo -e "${GREEN}✓ Checksum verified${NC}"
    else
        echo -e "${RED}✗ Checksum mismatch!${NC}"
        echo -e "${RED}Expected: $EXPECTED_HASH${NC}"
        echo -e "${RED}Got:      $ACTUAL_HASH${NC}"
        exit 1
    fi
fi

# Install binary
echo -e "${BLUE}Installing to $INSTALL_DIR/$BINARY_NAME...${NC}"
mv "/tmp/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Clean up
rm -f "/tmp/$BINARY_NAME.sha256"

echo ""
echo -e "${GREEN}===================================${NC}"
echo -e "${GREEN}   Installation successful!${NC}"
echo -e "${GREEN}===================================${NC}"
echo ""
echo -e "${YELLOW}Binary installed to: $INSTALL_DIR/$BINARY_NAME${NC}"
echo ""

# Check if directory is in PATH
if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
    echo -e "${GREEN}✓ $INSTALL_DIR is in your PATH${NC}"
    echo -e "${BLUE}You can now run: engram --help${NC}"
else
    echo -e "${YELLOW}⚠ $INSTALL_DIR is not in your PATH${NC}"
    echo ""
    echo -e "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo -e "${BLUE}export PATH=\"\$PATH:$INSTALL_DIR\"${NC}"
    echo ""
    echo -e "Or run directly: $INSTALL_DIR/engram --help"
fi

echo ""
