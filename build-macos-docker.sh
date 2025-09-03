#!/bin/bash

# macOS Docker Cross-Compilation Build Script
set -e

echo "🍎 macOS Docker Cross-Compilation Build"

# Check if we have the macOS SDK
SDK_PATH="./MacOSX.sdk"
if [ ! -d "$SDK_PATH" ]; then
    echo "❌ macOS SDK not found at $SDK_PATH"
    echo ""
    echo "📋 To get the macOS SDK:"
    echo "1. On a Mac, install Xcode"
    echo "2. Copy the SDK:"
    echo "   sudo cp -r /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk ./"
    echo "3. Or download from: https://github.com/phracker/MacOSX-SDKs"
    echo ""
    echo "🔧 Alternative: Use GitHub Actions for automatic builds"
    exit 1
fi

echo "✅ Found macOS SDK at $SDK_PATH"

# Build the Docker image
echo "🐳 Building Docker image..."
docker build -f Dockerfile.macos-cross -t timecard-macos-cross .

# Run the build
echo "🔨 Running cross-compilation..."
docker run --rm \
    -v "$(pwd):/app" \
    -v "$(pwd)/MacOSX.sdk:/opt/osxcross/SDK" \
    timecard-macos-cross

echo ""
echo "✅ Docker cross-compilation complete!"
echo "📁 Check the releases/ folder for macOS binaries"
