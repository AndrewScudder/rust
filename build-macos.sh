#!/bin/bash

# TimeCard macOS Build Script
echo "🕐 Building TimeCard for macOS..."

# Check if we're on macOS or have cross-compilation tools
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "✅ Building natively on macOS..."
    cargo build --release
    echo "✅ Build complete! Binary is at: ./target/release/timecard"
elif command -v rustup &> /dev/null; then
    echo "🔧 Setting up cross-compilation for macOS..."
    
    # Add macOS target
    rustup target add x86_64-apple-darwin
    rustup target add aarch64-apple-darwin
    
    # Build for Intel Mac
    echo "🔨 Building for Intel Mac (x86_64)..."
    cargo build --release --target x86_64-apple-darwin
    
    # Build for Apple Silicon Mac
    echo "🔨 Building for Apple Silicon Mac (aarch64)..."
    cargo build --release --target aarch64-apple-darwin
    
    echo "✅ Builds complete!"
    echo "📁 Intel Mac binary: ./target/x86_64-apple-darwin/release/timecard"
    echo "📁 Apple Silicon binary: ./target/aarch64-apple-darwin/release/timecard"
else
    echo "❌ Error: rustup not found. Please install Rust first."
    exit 1
fi

echo ""
echo "🍎 macOS Installation Instructions:"
echo "1. Copy the binary to your Applications folder or any location"
echo "2. Make it executable: chmod +x timecard"
echo "3. Run: ./timecard --help"
echo "4. For GUI: ./timecard gui"
echo ""
echo "📦 Optional: Create a .app bundle for macOS"
echo "   (Requires additional tools like create-dmg)"
