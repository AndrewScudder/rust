# 🖥️ TimeCard Platform Guide

## 🎯 **Application Type**

TimeCard is a **native desktop application** with both CLI and GUI interfaces:
- **CLI Mode**: Command-line interface (no port needed)
- **GUI Mode**: Native desktop window using Egui (no port needed)

## 🖥️ **Supported Operating Systems**

### ✅ **Linux** (Ubuntu, Debian, CentOS, etc.)
```bash
# Build
cargo build --release

# Run CLI
./target/release/timecard --help

# Run GUI
./target/release/timecard gui
```

### 🍎 **macOS** (Intel & Apple Silicon)
```bash
# Native build on macOS
cargo build --release

# Cross-compilation from Linux
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin  # Intel Mac
cargo build --release --target aarch64-apple-darwin # Apple Silicon
```

### 🪟 **Windows** (10, 11, Server)
```bash
# Cross-compilation from Linux
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Native build on Windows
cargo build --release
```

## 🚀 **Quick Start for Each Platform**

### **Linux**
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build and run
cd timecard
cargo build --release
./target/release/timecard gui
```

### **macOS**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build and run
cd timecard
cargo build --release
./target/release/timecard gui
```

### **Windows**
```bash
# Install Rust from https://rustup.rs/
# Then in Command Prompt or PowerShell:
cd timecard
cargo build --release
.\target\release\timecard.exe gui
```

## 📦 **Dependencies**

### **Minimal Dependencies (10 total)**
```toml
# Core functionality
clap = "4.0"        # CLI parsing
serde = "1.0"       # JSON serialization
chrono = "0.4"      # Date/time handling
uuid = "1.0"        # Unique identifiers
anyhow = "1.0"      # Error handling
thiserror = "1.0"   # Custom errors
colored = "2.0"     # Terminal colors
csv = "1.3"         # CSV export

# GUI (optional)
eframe = "0.24"     # GUI framework
egui = "0.24"       # GUI toolkit
```

### **System Dependencies**
- **Linux**: `build-essential` (gcc, make, etc.)
- **macOS**: Xcode Command Line Tools
- **Windows**: Visual Studio Build Tools

## 🔧 **Build Scripts**

### **macOS Build Script**
```bash
# Run the provided script
./build-macos.sh
```

### **Cross-Platform Build**
```bash
# Build for all platforms
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add x86_64-pc-windows-gnu

cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-pc-windows-gnu
```

## 📁 **Binary Locations**

After building, binaries are located at:
```
target/
├── release/
│   └── timecard          # Current platform
└── x86_64-apple-darwin/release/
    └── timecard          # macOS Intel
└── aarch64-apple-darwin/release/
    └── timecard          # macOS Apple Silicon
└── x86_64-pc-windows-gnu/release/
    └── timecard.exe      # Windows
```

## 🎨 **GUI Features**

### **Desktop Application**
- **No web server** - runs locally
- **No port required** - native window
- **Cross-platform** - same code on all OS
- **Lightweight** - ~10MB total size

### **GUI Interface**
- 📊 **Status Display**: Current clock status
- 🕐 **Clock In/Out**: Simple buttons
- 📈 **Quick Stats**: Today's hours and entries
- 📊 **Reports**: Period-based reporting
- ➕ **Manual Entry**: Add time entries

## 🔍 **Troubleshooting**

### **Common Issues**

#### **Linux**
```bash
# Missing build tools
sudo apt install build-essential

# Missing OpenGL libraries (for GUI)
sudo apt install libgl1-mesa-dev
```

#### **macOS**
```bash
# Missing Xcode tools
xcode-select --install

# GUI not working
# Make sure you're running on a Mac with display
```

#### **Windows**
```bash
# Missing Visual Studio tools
# Install Visual Studio Build Tools 2019 or later

# GUI not working
# Make sure you have a display and graphics drivers
```

### **Build Errors**
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Update Rust
rustup update

# Check target
rustup target list --installed
```

## 📊 **Performance**

### **Binary Sizes**
- **CLI Only**: ~3-5MB
- **With GUI**: ~8-12MB
- **All Platforms**: ~25-30MB total

### **Startup Time**
- **CLI**: <100ms
- **GUI**: <500ms
- **Memory**: <50MB

## 🚀 **Deployment**

### **Distribution**
1. **Copy binary** to target machine
2. **Make executable**: `chmod +x timecard`
3. **Run**: `./timecard gui`

### **macOS App Bundle** (Optional)
```bash
# Create .app bundle
mkdir -p TimeCard.app/Contents/MacOS
cp target/release/timecard TimeCard.app/Contents/MacOS/
# Add Info.plist and other bundle files
```

### **Windows Installer** (Optional)
```bash
# Use tools like NSIS or Inno Setup
# Package timecard.exe with dependencies
```

## 🎯 **Summary**

- ✅ **No web server** - pure desktop app
- ✅ **No port needed** - runs locally
- ✅ **Cross-platform** - Linux, macOS, Windows
- ✅ **Minimal dependencies** - only 10 crates
- ✅ **Fast performance** - Rust native code
- ✅ **Easy deployment** - single binary

**TimeCard works as a traditional desktop application, just like Notepad or Calculator!** 🖥️✨
