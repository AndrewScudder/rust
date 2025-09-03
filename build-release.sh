#!/bin/bash

# TimeCard Release Build Script
set -e

VERSION="1.0.0"
RELEASE_DIR="releases"
BUILD_DIR="target"

echo "🚀 Building TimeCard Release v$VERSION..."

# Create release directory
mkdir -p "$RELEASE_DIR"

# Build for current platform
echo "🔨 Building for current platform..."
cargo build --release

# Get current platform info
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Determine binary name
if [[ "$PLATFORM" == "darwin" ]]; then
    PLATFORM_NAME="macos"
    BINARY_NAME="timecard"
elif [[ "$PLATFORM" == "linux" ]]; then
    PLATFORM_NAME="linux"
    BINARY_NAME="timecard"
elif [[ "$PLATFORM" == "msys_nt" ]] || [[ "$PLATFORM" == "cygwin" ]]; then
    PLATFORM_NAME="windows"
    BINARY_NAME="timecard.exe"
else
    PLATFORM_NAME="unknown"
    BINARY_NAME="timecard"
fi

# Copy binary to releases folder
RELEASE_NAME="timecard-v${VERSION}-${PLATFORM_NAME}-${ARCH}"
echo "📦 Creating release: $RELEASE_NAME"

cp "$BUILD_DIR/release/$BINARY_NAME" "$RELEASE_DIR/$RELEASE_NAME"
chmod +x "$RELEASE_DIR/$RELEASE_NAME"

# Create checksum
echo "🔍 Creating checksum..."
cd "$RELEASE_DIR"
sha256sum "$RELEASE_NAME" > "$RELEASE_NAME.sha256"
cd ..

# Show release info
echo ""
echo "✅ Release created successfully!"
echo "📁 Binary: $RELEASE_DIR/$RELEASE_NAME"
echo "🔍 Checksum: $RELEASE_DIR/$RELEASE_NAME.sha256"
echo "📊 Size: $(ls -lh $RELEASE_DIR/$RELEASE_NAME | awk '{print $5}')"

# Optional: Cross-compilation for other platforms
if command -v rustup &> /dev/null; then
    echo ""
    echo "🌍 Building for other platforms..."
    
    # macOS Intel
    if [[ "$PLATFORM_NAME" != "macos" ]] || [[ "$ARCH" != "x86_64" ]]; then
        echo "🍎 Building for macOS Intel..."
        rustup target add x86_64-apple-darwin
        cargo build --release --target x86_64-apple-darwin
        cp "$BUILD_DIR/x86_64-apple-darwin/release/timecard" "$RELEASE_DIR/timecard-v${VERSION}-macos-x86_64"
        chmod +x "$RELEASE_DIR/timecard-v${VERSION}-macos-x86_64"
    fi
    
    # macOS Apple Silicon
    if [[ "$PLATFORM_NAME" != "macos" ]] || [[ "$ARCH" != "aarch64" ]]; then
        echo "🍎 Building for macOS Apple Silicon..."
        rustup target add aarch64-apple-darwin
        cargo build --release --target aarch64-apple-darwin
        cp "$BUILD_DIR/aarch64-apple-darwin/release/timecard" "$RELEASE_DIR/timecard-v${VERSION}-macos-aarch64"
        chmod +x "$RELEASE_DIR/timecard-v${VERSION}-macos-aarch64"
    fi
    
    # Windows
    if [[ "$PLATFORM_NAME" != "windows" ]]; then
        echo "🪟 Building for Windows..."
        rustup target add x86_64-pc-windows-gnu
        cargo build --release --target x86_64-pc-windows-gnu
        cp "$BUILD_DIR/x86_64-pc-windows-gnu/release/timecard.exe" "$RELEASE_DIR/timecard-v${VERSION}-windows-x86_64.exe"
    fi
    
    # Create checksums for all
    echo "🔍 Creating checksums for all platforms..."
    cd "$RELEASE_DIR"
    sha256sum timecard-v${VERSION}-* > checksums.sha256
    cd ..
fi

echo ""
echo "📋 Release Summary:"
echo "=================="
ls -lh "$RELEASE_DIR/"
echo ""
echo "🎯 Usage:"
echo "  ./$RELEASE_DIR/$RELEASE_NAME --help"
echo "  ./$RELEASE_DIR/$RELEASE_NAME gui"
echo ""
echo "✅ Release build complete!"
