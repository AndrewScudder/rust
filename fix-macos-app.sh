#!/bin/bash

# Fix macOS App Issues - Comprehensive Solution
set -e

echo "🔧 Fixing TimeCard.app for macOS..."

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ This script should be run on macOS"
    exit 1
fi

# Check if TimeCard.app exists
if [ ! -d "TimeCard.app" ]; then
    echo "❌ TimeCard.app not found"
    echo "💡 Download the app from GitHub Actions first"
    exit 1
fi

echo "🔍 Step 1: Removing quarantine attributes..."
xattr -dr com.apple.quarantine TimeCard.app

echo "🔍 Step 2: Removing all extended attributes..."
xattr -cr TimeCard.app

echo "🔍 Step 3: Self-signing the app bundle..."
codesign --force --deep --sign - TimeCard.app

echo "🔍 Step 4: Verifying the signature..."
codesign --verify --verbose=4 TimeCard.app

echo "🔍 Step 5: Checking app structure..."
ls -la TimeCard.app/Contents/MacOS/

echo "🔍 Step 6: Making sure executable is... executable..."
chmod +x TimeCard.app/Contents/MacOS/TimeCard

echo ""
echo "✅ Fix complete!"
echo "📁 App location: TimeCard.app"
echo ""
echo "🚀 Try these methods to run:"
echo ""
echo "Method 1 - Right-click:"
echo "   Right-click TimeCard.app → Open"
echo ""
echo "Method 2 - Terminal:"
echo "   open TimeCard.app"
echo ""
echo "Method 3 - Direct execution:"
echo "   ./TimeCard.app/Contents/MacOS/TimeCard gui"
echo ""
echo "Method 4 - Move to Applications:"
echo "   sudo mv TimeCard.app /Applications/"
echo "   open /Applications/TimeCard.app"
echo ""
echo "🎯 If it still shows 'damaged':"
echo "   - Go to System Preferences → Security & Privacy"
echo "   - Click 'Allow Anyway' for TimeCard"
echo ""
echo "🍎 TimeCard.app should now work properly!"
