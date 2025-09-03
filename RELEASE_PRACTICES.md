# 📦 Release Folder Practices in Rust

## 🎯 **Common Release Folder Patterns**

### **1. Rust Standard (Default)**
```
project/
├── src/
├── Cargo.toml
└── target/
    ├── debug/          # Development builds
    └── release/        # Optimized builds
        └── timecard    # Your executable
```

**Pros:**
- ✅ Rust standard convention
- ✅ Automatic cleanup with `cargo clean`
- ✅ Integrated with Cargo workflow
- ✅ No additional setup needed

**Cons:**
- ❌ Not versioned
- ❌ No checksums
- ❌ No multi-platform organization

### **2. Custom Releases Folder (Recommended)**
```
project/
├── src/
├── Cargo.toml
├── target/             # Rust build artifacts
└── releases/           # Distribution binaries
    ├── timecard-v1.0.0-linux-x86_64
    ├── timecard-v1.0.0-linux-x86_64.sha256
    ├── timecard-v1.0.0-macos-x86_64
    ├── timecard-v1.0.0-macos-aarch64
    ├── timecard-v1.0.0-windows-x86_64.exe
    └── checksums.sha256
```

**Pros:**
- ✅ Versioned releases
- ✅ Checksums for integrity
- ✅ Multi-platform support
- ✅ Easy distribution
- ✅ Clear separation from build artifacts

**Cons:**
- ❌ Manual management
- ❌ Need custom scripts

### **3. GitHub Releases Style**
```
project/
├── src/
├── Cargo.toml
└── dist/               # Distribution folder
    ├── v1.0.0/
    │   ├── timecard-linux-x86_64
    │   ├── timecard-macos-x86_64
    │   ├── timecard-macos-aarch64
    │   └── timecard-windows-x86_64.exe
    └── latest/
        └── timecard-{platform}
```

## 🚀 **Industry Best Practices**

### **Versioning Convention**
```bash
# Semantic versioning
timecard-v1.2.3-linux-x86_64
timecard-v1.2.3-macos-aarch64
timecard-v1.2.3-windows-x86_64.exe

# With build info
timecard-v1.2.3+20230903-linux-x86_64
timecard-v1.2.3-beta.1-linux-x86_64
```

### **File Organization**
```bash
releases/
├── v1.0.0/                    # Version folder
│   ├── binaries/              # Executables
│   ├── checksums/             # Integrity files
│   └── installers/            # Platform installers
├── latest/                    # Latest version symlinks
└── archive/                   # Old versions
```

### **Checksums & Integrity**
```bash
# Always include checksums
sha256sum timecard-v1.0.0-linux-x86_64 > timecard-v1.0.0-linux-x86_64.sha256

# Verify downloads
sha256sum -c timecard-v1.0.0-linux-x86_64.sha256
```

## 🔧 **Our TimeCard Release Setup**

### **Current Structure**
```
timecard/
├── src/
├── Cargo.toml
├── target/release/timecard    # Rust build
├── releases/                  # Distribution
│   ├── timecard-v1.0.0-linux-x86_64
│   └── timecard-v1.0.0-linux-x86_64.sha256
├── build-release.sh          # Release script
└── build-macos.sh            # macOS specific
```

### **Release Script Features**
- ✅ Automatic versioning
- ✅ Platform detection
- ✅ Checksum generation
- ✅ Cross-compilation support
- ✅ Clean organization

## 📊 **Comparison: Release vs Target**

| Aspect | `target/release/` | `releases/` |
|--------|------------------|-------------|
| **Purpose** | Build artifacts | Distribution |
| **Versioning** | No | Yes |
| **Checksums** | No | Yes |
| **Multi-platform** | No | Yes |
| **Cleanup** | `cargo clean` | Manual |
| **Distribution** | Not recommended | Recommended |

## 🎯 **When to Use Each**

### **Use `target/release/` for:**
- ✅ Development testing
- ✅ Local builds
- ✅ CI/CD pipelines
- ✅ Quick testing

### **Use `releases/` for:**
- ✅ Distribution to users
- ✅ Version management
- ✅ Multi-platform builds
- ✅ Release archives
- ✅ GitHub releases

## 🚀 **Recommended Workflow**

### **1. Development**
```bash
# Build for testing
cargo build --release
./target/release/timecard gui
```

### **2. Release**
```bash
# Create distribution release
./build-release.sh

# Test the release
./releases/timecard-v1.0.0-linux-x86_64 gui
```

### **3. Distribution**
```bash
# Upload to GitHub releases
# Share with users
# Include checksums for verification
```

## 📋 **Best Practices Summary**

### **✅ Do:**
- Use semantic versioning
- Include checksums
- Organize by platform
- Keep build artifacts separate
- Document release process
- Test releases before distribution

### **❌ Don't:**
- Distribute from `target/`
- Skip versioning
- Forget checksums
- Mix build and release files
- Ignore platform differences

## 🎯 **Conclusion**

**For TimeCard:**
- ✅ **Development**: Use `target/release/`
- ✅ **Distribution**: Use `releases/` folder
- ✅ **Versioning**: Semantic versioning
- ✅ **Integrity**: Always include checksums
- ✅ **Organization**: Clear platform separation

**This gives us the best of both worlds:**
- Fast development workflow with Rust's standard `target/`
- Professional distribution with organized `releases/`

The `releases/` folder is definitely common practice for production applications! 🚀
