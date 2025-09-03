#!/bin/bash

# Simple Docker Cross-Compilation for macOS
set -e

echo "🐳 Simple Docker Cross-Compilation for macOS"

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Docker not found. Please install Docker first."
    exit 1
fi

echo "🔧 Setting up simple cross-compilation..."

# Create a simpler Dockerfile that uses a pre-built image
cat > Dockerfile.simple << 'EOF'
FROM rust:1.75-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Add macOS targets
RUN rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Set up build environment
WORKDIR /app
COPY . .

# Create build script
RUN echo '#!/bin/bash\n\
set -e\n\
echo "🔨 Attempting cross-compilation..."\n\
\n\
# Try to build for Intel Mac\n\
echo "Building for Intel Mac..."\n\
if cargo build --release --target x86_64-apple-darwin; then\n\
    echo "✅ Intel Mac build successful!"\n\
    mkdir -p releases\n\
    cp target/x86_64-apple-darwin/release/timecard releases/timecard-macos-x86_64 || true\n\
else\n\
    echo "❌ Intel Mac build failed (missing SDK)"\n\
fi\n\
\n\
# Try to build for Apple Silicon\n\
echo "Building for Apple Silicon..."\n\
if cargo build --release --target aarch64-apple-darwin; then\n\
    echo "✅ Apple Silicon build successful!"\n\
    mkdir -p releases\n\
    cp target/aarch64-apple-darwin/release/timecard releases/timecard-macos-aarch64 || true\n\
else\n\
    echo "❌ Apple Silicon build failed (missing SDK)"\n\
fi\n\
\n\
echo "📊 Build results:"\n\
ls -la releases/ 2>/dev/null || echo "No builds completed"\n\
echo ""\n\
echo "💡 Note: Cross-compilation requires macOS SDK."\n\
echo "   For successful builds, use GitHub Actions instead."\n\
' > /opt/build.sh && chmod +x /opt/build.sh

CMD ["/opt/build.sh"]
EOF

echo "📦 Created Dockerfile.simple"

# Create a test script
cat > test-docker-build.sh << 'EOF'
#!/bin/bash

# Test Docker Cross-Compilation
set -e

echo "🧪 Testing Docker Cross-Compilation"

# Build the Docker image
echo "🐳 Building Docker image..."
docker build -f Dockerfile.simple -t timecard-cross-test .

# Run the build
echo "🔨 Running cross-compilation test..."
docker run --rm \
    -v "$(pwd):/app" \
    timecard-cross-test

echo ""
echo "📋 Results:"
echo "==========="
echo "If builds failed, this is expected without the macOS SDK."
echo "The GitHub Actions approach is recommended for production builds."
EOF

chmod +x test-docker-build.sh

echo "📦 Created test-docker-build.sh"

# Create a GitHub Actions alternative
echo ""
echo "🚀 Recommended Alternative: GitHub Actions"
echo "=========================================="
echo ""
echo "Since Docker cross-compilation requires the macOS SDK,"
echo "the most reliable approach is GitHub Actions:"
echo ""
echo "1. Push your code to GitHub:"
echo "   git remote add origin <your-github-repo>"
echo "   git push -u origin main"
echo ""
echo "2. Create a release:"
echo "   git tag v1.0.0"
echo "   git push origin v1.0.0"
echo ""
echo "3. GitHub Actions will automatically:"
echo "   - Build for Linux (x86_64)"
echo "   - Build for macOS Intel (x86_64)"
echo "   - Build for macOS Apple Silicon (aarch64)"
echo "   - Build for Windows (x86_64)"
echo ""
echo "4. Download the Apple Silicon binary from the release"
echo ""

echo "🐳 Docker setup complete!"
echo "📋 Options:"
echo "1. Test Docker build: ./test-docker-build.sh"
echo "2. Use GitHub Actions (recommended): Push to GitHub and create release"
echo "3. Build natively on Apple Silicon Mac"
