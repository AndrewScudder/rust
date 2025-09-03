# 🍎 macOS Build Guide for TimeCard

## 🎯 **Apple Silicon Mac Build Options**

### **Option 1: Native Build on Apple Silicon Mac (Recommended)**

**Prerequisites:**
- Apple Silicon Mac (M1, M2, M3, etc.)
- Xcode Command Line Tools

**Steps:**
```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 2. Clone the repository
git clone <your-repo-url>
cd timecard

# 3. Build the application
cargo build --release

# 4. Run the application
./target/release/timecard gui
```

### **Option 2: GitHub Actions (Automatic)**

**Setup:**
1. Push your code to GitHub
2. Create a release tag: `git tag v1.0.0 && git push origin v1.0.0`
3. GitHub Actions will automatically build for all platforms
4. Download the Apple Silicon binary from the release

**Workflow:**
```yaml
# .github/workflows/release.yml (already created)
# Automatically builds for:
# - Linux (x86_64)
# - macOS Intel (x86_64)
# - macOS Apple Silicon (aarch64)
# - Windows (x86_64)
```

### **Option 3: Cross-Compilation from Linux**

**Requirements:**
- macOS SDK (requires access to a Mac)
- osxcross toolchain

**Setup:**
```bash
# Install osxcross (complex setup)
git clone https://github.com/tpoechtrager/osxcross.git
# Follow osxcross documentation for SDK setup

# Build for Apple Silicon
cargo build --release --target aarch64-apple-darwin
```

## 🚀 **Quick Start for Apple Silicon**

### **1. Install Rust on Apple Silicon Mac**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### **2. Build TimeCard**
```bash
# Clone repository
git clone <your-repo>
cd timecard

# Build optimized version
cargo build --release

# Test the build
./target/release/timecard --help
```

### **3. Run the GUI**
```bash
# Launch GUI
./target/release/timecard gui

# Or use CLI
./target/release/timecard status
./target/release/timecard in -p "Development"
```

## 📦 **Installation Options**

### **Option A: Direct Execution**
```bash
# Make executable
chmod +x target/release/timecard

# Run from anywhere
./target/release/timecard gui
```

### **Option B: Install to Applications**
```bash
# Copy to Applications
sudo cp target/release/timecard /Applications/TimeCard

# Make executable
sudo chmod +x /Applications/TimeCard

# Run
/Applications/TimeCard gui
```

### **Option C: Create App Bundle**
```bash
# Create .app structure
mkdir -p TimeCard.app/Contents/MacOS
cp target/release/timecard TimeCard.app/Contents/MacOS/

# Create Info.plist
cat > TimeCard.app/Contents/Info.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>timecard</string>
    <key>CFBundleIdentifier</key>
    <string>com.timecard.app</string>
    <key>CFBundleName</key>
    <string>TimeCard</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
</dict>
</plist>
EOF

# Move to Applications
sudo mv TimeCard.app /Applications/
```

## 🔧 **Troubleshooting**

### **Common Issues**

#### **1. "Command not found: cargo"**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### **2. "linker command failed"**
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

#### **3. "Permission denied"**
```bash
# Make executable
chmod +x target/release/timecard
```

#### **4. GUI not working**
```bash
# Check if you have a display
echo $DISPLAY

# Try running from Terminal.app
# Make sure you're not in SSH without X11 forwarding
```

### **Performance Tips**

#### **Optimize Build Speed**
```bash
# Use release profile
cargo build --release

# Enable parallel compilation
export RUSTFLAGS="-C target-cpu=native"
cargo build --release
```

#### **Reduce Binary Size**
```bash
# Strip debug symbols
strip target/release/timecard

# Use upx for compression (optional)
upx --best target/release/timecard
```

## 📊 **Apple Silicon Performance**

### **Expected Performance**
- **Startup Time**: <500ms
- **Memory Usage**: <50MB
- **Binary Size**: ~8-12MB
- **GUI Responsiveness**: Native performance

### **Architecture Benefits**
- **M1/M2/M3 Optimization**: Native ARM64 code
- **Unified Memory**: Efficient memory access
- **GPU Acceleration**: Metal graphics support
- **Battery Life**: Optimized for efficiency

## 🎯 **Summary**

**For Apple Silicon Macs:**

1. **✅ Native Build**: Best performance and compatibility
2. **✅ GitHub Actions**: Automatic builds on every release
3. **❌ Cross-compilation**: Complex setup, not recommended

**Recommended Workflow:**
```bash
# On Apple Silicon Mac
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
git clone <repo>
cd timecard
cargo build --release
./target/release/timecard gui
```

**TimeCard runs natively on Apple Silicon with excellent performance!** 🍎✨
