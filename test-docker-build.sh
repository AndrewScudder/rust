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
