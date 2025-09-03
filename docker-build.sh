#!/bin/bash

# Docker-based macOS Cross-Compilation Script
set -e

echo "🐳 Setting up Docker cross-compilation for macOS..."

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Docker not found. Please install Docker first."
    exit 1
fi

# Create a more comprehensive Dockerfile
cat > Dockerfile.macos-cross << 'EOF'
FROM ubuntu:22.04

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    cmake \
    libssl-dev \
    pkg-config \
    clang \
    llvm \
    libxml2-dev \
    libssl-dev \
    libbz2-dev \
    libreadline-dev \
    libsqlite3-dev \
    wget \
    ca-certificates \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add macOS targets
RUN rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Install osxcross (we'll mount the SDK later)
RUN git clone https://github.com/tpoechtrager/osxcross.git /opt/osxcross
WORKDIR /opt/osxcross

# Set up environment for cross-compilation
ENV OSXCROSS_SDK=/opt/osxcross/SDK
ENV OSXCROSS_TARGET=x86_64-apple-darwin
ENV PATH="/opt/osxcross/target/bin:${PATH}"

# Create a build script
RUN echo '#!/bin/bash\n\
set -e\n\
echo "🔨 Building for macOS..."\n\
cd /app\n\
\n\
# Build for Intel Mac\n\
echo "Building for Intel Mac (x86_64)..."\n\
cargo build --release --target x86_64-apple-darwin\n\
\n\
# Build for Apple Silicon\n\
echo "Building for Apple Silicon (aarch64)..."\n\
cargo build --release --target aarch64-apple-darwin\n\
\n\
# Create releases directory\n\
mkdir -p releases\n\
\n\
# Copy binaries\n\
cp target/x86_64-apple-darwin/release/timecard releases/timecard-macos-x86_64\n\
cp target/aarch64-apple-darwin/release/timecard releases/timecard-macos-aarch64\n\
\n\
# Make executable\n\
chmod +x releases/timecard-macos-*\n\
\n\
echo "✅ Build complete!"\n\
ls -la releases/\n\
' > /opt/build.sh && chmod +x /opt/build.sh

WORKDIR /app
CMD ["/opt/build.sh"]
EOF

echo "📦 Created Dockerfile.macos-cross"

# Create a build script that handles the SDK
cat > build-macos-docker.sh << 'EOF'
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
EOF

chmod +x build-macos-docker.sh

echo "📦 Created build-macos-docker.sh"

# Create a simpler alternative using GitHub Actions
echo ""
echo "🔄 Alternative: GitHub Actions (Recommended)"
echo "============================================="
echo ""
echo "Since Docker cross-compilation requires the macOS SDK,"
echo "the easiest approach is to use GitHub Actions:"
echo ""
echo "1. Push your code to GitHub"
echo "2. Create a release tag:"
echo "   git tag v1.0.0 && git push origin v1.0.0"
echo "3. GitHub Actions will automatically build for Apple Silicon"
echo "4. Download the binary from the release"
echo ""
echo "The GitHub Actions workflow is already set up in .github/workflows/release.yml"
echo ""

echo "🐳 Docker setup complete!"
echo "📋 Next steps:"
echo "1. Get macOS SDK (see instructions above)"
echo "2. Run: ./build-macos-docker.sh"
echo "3. Or use GitHub Actions for automatic builds"
