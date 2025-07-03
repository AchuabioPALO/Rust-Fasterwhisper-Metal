#!/bin/bash

# Build script for Rust + PyO3 application
# Ensures correct Python linking

set -e

echo "🚀 Building Rust Whisper App with proper Python linking..."

# Source environment if .env exists
if [[ -f ".env" ]]; then
    echo "📂 Loading environment from .env"
    source .env
else
    echo "⚠️  No .env file found, running setup..."
    ./scripts/setup_python_env.sh
    source .env
fi

echo "🔧 Using Python: $PYTHON_SYS_EXECUTABLE"

# Verify Python is working
if ! command -v "$PYTHON_SYS_EXECUTABLE" &> /dev/null; then
    echo "❌ Error: Python executable not found at $PYTHON_SYS_EXECUTABLE"
    exit 1
fi

echo "✅ Python verification passed"

# Build the application
echo "🔨 Building with cargo..."
PYTHON_SYS_EXECUTABLE="$PYTHON_SYS_EXECUTABLE" cargo build --release

echo "✅ Build completed successfully!"
echo ""
echo "🎯 You can now run your application with:"
echo "  ./target/release/rust-whisper-app"
