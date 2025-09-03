#!/bin/bash

# TimeCard macOS Cross-Compilation Script
# Builds for both Intel and Apple Silicon Macs from Linux

set -e

VERSION="1.0.0"
RELEASE_DIR="releases"
BUILD_DIR="target"

echo "🍎 Building TimeCard for macOS from Linux..."

# Ensure targets are installed
echo "🔧 Checking macOS targets..."
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Create release directory
mkdir -p "$RELEASE_DIR"

# Build for Intel Mac (x86_64)
echo "🔨 Building for Intel Mac (x86_64)..."
cargo build --release --target x86_64-apple-darwin

# Build for Apple Silicon Mac (aarch64)
echo "🔨 Building for Apple Silicon Mac (aarch64)..."
cargo build --release --target aarch64-apple-darwin

# Copy binaries to releases folder
echo "📦 Creating macOS releases..."

# Intel Mac
INTEL_RELEASE="timecard-v${VERSION}-macos-x86_64"
cp "$BUILD_DIR/x86_64-apple-darwin/release/timecard" "$RELEASE_DIR/$INTEL_RELEASE"
chmod +x "$RELEASE_DIR/$INTEL_RELEASE"

# Apple Silicon Mac
SILICON_RELEASE="timecard-v${VERSION}-macos-aarch64"
cp "$BUILD_DIR/aarch64-apple-darwin/release/timecard" "$RELEASE_DIR/$SILICON_RELEASE"
chmod +x "$RELEASE_DIR/$SILICON_RELEASE"

# Create checksums
echo "🔍 Creating checksums..."
cd "$RELEASE_DIR"
sha256sum "$INTEL_RELEASE" > "$INTEL_RELEASE.sha256"
sha256sum "$SILICON_RELEASE" > "$SILICON_RELEASE.sha256"
cd ..

# Show results
echo ""
echo "✅ macOS builds complete!"
echo "📁 Intel Mac: $RELEASE_DIR/$INTEL_RELEASE"
echo "📁 Apple Silicon: $RELEASE_DIR/$SILICON_RELEASE"
echo "🔍 Checksums created for both"
echo ""
echo "📊 File sizes:"
ls -lh "$RELEASE_DIR"/timecard-v${VERSION}-macos-*

echo ""
echo "🍎 macOS Installation Instructions:"
echo "=================================="
echo ""
echo "1. Copy the appropriate binary to your Mac:"
echo "   - Intel Mac: $INTEL_RELEASE"
echo "   - Apple Silicon Mac: $SILICON_RELEASE"
echo ""
echo "2. Make it executable:"
echo "   chmod +x timecard-v${VERSION}-macos-*"
echo ""
echo "3. Run the application:"
echo "   ./timecard-v${VERSION}-macos-* --help"
echo "   ./timecard-v${VERSION}-macos-* gui"
echo ""
echo "4. Optional: Move to Applications folder:"
echo "   sudo mv timecard-v${VERSION}-macos-* /Applications/TimeCard"
echo ""
echo "✅ macOS cross-compilation complete!"
