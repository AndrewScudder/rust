#!/bin/bash

# Create macOS App Bundle for TimeCard
set -e

echo "🍎 Creating macOS App Bundle for TimeCard..."

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ This script should be run on macOS"
    echo "💡 For cross-platform builds, use GitHub Actions"
    exit 1
fi

# Build the application first
echo "🔨 Building TimeCard..."
cargo build --release

# Create app bundle structure
echo "📦 Creating app bundle..."
APP_NAME="TimeCard.app"
CONTENTS_DIR="$APP_NAME/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

# Create directories
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Copy the binary
echo "📋 Copying binary..."
cp target/release/timecard "$MACOS_DIR/TimeCard"

# Make executable
chmod +x "$MACOS_DIR/TimeCard"

# Create Info.plist
echo "📄 Creating Info.plist..."
cat > "$CONTENTS_DIR/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>TimeCard</string>
    <key>CFBundleIdentifier</key>
    <string>com.andrewscudder.timecard</string>
    <key>CFBundleName</key>
    <string>TimeCard</string>
    <key>CFBundleDisplayName</key>
    <string>TimeCard</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.productivity</string>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
</dict>
</plist>
EOF

# Create PkgInfo
echo "📄 Creating PkgInfo..."
echo "APPL????" > "$CONTENTS_DIR/PkgInfo"

# Create a simple app icon placeholder (optional)
echo "🎨 Creating app icon placeholder..."
cat > "$RESOURCES_DIR/AppIcon.icns" << 'EOF'
# This is a placeholder - you can replace with a real .icns file
EOF

echo ""
echo "✅ macOS App Bundle created!"
echo "📁 App location: $APP_NAME"
echo ""
echo "🚀 Next steps:"
echo "1. Test the app: open $APP_NAME"
echo "2. Move to Applications: sudo mv $APP_NAME /Applications/"
echo "3. Launch from Applications folder"
echo ""
echo "🎯 The app will now appear in:"
echo "   - Applications folder"
echo "   - Spotlight search"
echo "   - Dock (when running)"
echo "   - Launchpad"
echo ""
echo "🍎 TimeCard is now a proper macOS application!"
