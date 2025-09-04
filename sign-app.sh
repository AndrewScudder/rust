#!/bin/bash

# Self-sign macOS App for Development
set -e

echo "🔐 Self-signing TimeCard.app for development..."

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ This script should be run on macOS"
    exit 1
fi

# Check if TimeCard.app exists
if [ ! -d "TimeCard.app" ]; then
    echo "❌ TimeCard.app not found"
    echo "💡 Run create-macos-app.sh first to create the app bundle"
    exit 1
fi

echo "🔨 Self-signing the app bundle..."
codesign --force --deep --sign - TimeCard.app

echo "✅ Verification..."
codesign --verify --verbose=4 TimeCard.app

echo ""
echo "✅ Self-signing complete!"
echo "📁 App location: TimeCard.app"
echo ""
echo "🚀 Next steps:"
echo "1. Test the app: open TimeCard.app"
echo "2. Move to Applications: sudo mv TimeCard.app /Applications/"
echo ""
echo "🎯 Benefits of self-signing:"
echo "   - Reduced security warnings"
echo "   - Easier to run during development"
echo "   - Still shows some warnings (normal for self-signed)"
echo ""
echo "🍎 TimeCard.app is now self-signed for development!"
