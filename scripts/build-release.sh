#!/bin/bash
set -e

# Build script for cross-platform releases
# Requires: rustup, cross (cargo install cross)

echo "==================================="
echo "Engram CLI - Cross-Platform Builder"
echo "==================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Targets to build
TARGETS=(
    "x86_64-pc-windows-msvc"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
)

# Output directory
OUTPUT_DIR="dist"
mkdir -p "$OUTPUT_DIR"

echo -e "${YELLOW}Installing required targets...${NC}"
for target in "${TARGETS[@]}"; do
    rustup target add "$target" 2>/dev/null || true
done

echo ""
echo -e "${YELLOW}Building for all platforms...${NC}"
echo ""

build_target() {
    local target=$1
    local bin_name="engram"
    local ext=""
    local platform_name=""

    # Determine extension and platform name
    case "$target" in
        *windows*)
            ext=".exe"
            platform_name="Windows-x86_64"
            ;;
        *darwin*)
            if [[ "$target" == *"aarch64"* ]]; then
                platform_name="Darwin-aarch64"
            else
                platform_name="Darwin-x86_64"
            fi
            ;;
        *linux*)
            if [[ "$target" == *"musl"* ]]; then
                platform_name="Linux-x86_64-musl"
            elif [[ "$target" == *"aarch64"* ]]; then
                platform_name="Linux-aarch64"
            else
                platform_name="Linux-x86_64"
            fi
            ;;
    esac

    echo -e "${GREEN}Building $platform_name ($target)...${NC}"

    # Build with cargo or cross
    if [[ "$target" == *"windows"* ]] || [[ "$target" == *"darwin"* && "$(uname)" == "Darwin" ]]; then
        # Use cargo for native or macOS targets on macOS
        cargo build --release --target "$target"
    else
        # Use cross for cross-compilation
        if command -v cross &> /dev/null; then
            cross build --release --target "$target"
        else
            echo -e "${RED}Warning: 'cross' not found. Install with: cargo install cross${NC}"
            echo "Trying with cargo (may fail for cross-compilation)..."
            cargo build --release --target "$target"
        fi
    fi

    # Copy binary to dist
    local binary_path="target/$target/release/$bin_name$ext"
    local output_name="$OUTPUT_DIR/engram-$platform_name$ext"

    if [[ -f "$binary_path" ]]; then
        cp "$binary_path" "$output_name"

        # Strip binary (Unix only)
        if [[ "$ext" != ".exe" ]]; then
            strip "$output_name" 2>/dev/null || true
        fi

        # Generate SHA-256 checksum
        if command -v sha256sum &> /dev/null; then
            sha256sum "$output_name" | awk '{print $1}' > "$output_name.sha256"
        elif command -v shasum &> /dev/null; then
            shasum -a 256 "$output_name" | awk '{print $1}' > "$output_name.sha256"
        fi

        echo -e "${GREEN}✓ Built: $output_name${NC}"
    else
        echo -e "${RED}✗ Failed: $binary_path not found${NC}"
    fi

    echo ""
}

# Build all targets
for target in "${TARGETS[@]}"; do
    build_target "$target"
done

echo -e "${GREEN}==================================="
echo "Build complete!"
echo "===================================${NC}"
echo ""
echo "Binaries available in: $OUTPUT_DIR/"
ls -lh "$OUTPUT_DIR/"
