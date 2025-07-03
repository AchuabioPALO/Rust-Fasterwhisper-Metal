#!/bin/bash

# Setup script for Rust + FasterWhisper Application
# This script sets up the environment and runs comprehensive tests

set -e

echo "🚀 Setting up Rust + FasterWhisper Environment"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're on macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    print_status "Detected macOS - Metal acceleration will be available"
    METAL_AVAILABLE=true
else
    print_warning "Not running on macOS - Metal acceleration not available"
    METAL_AVAILABLE=false
fi

# Check Python installation
print_status "Checking Python installation..."
if command -v python3.11 &> /dev/null; then
    PYTHON_CMD="python3.11"
    print_success "Found Python 3.11"
elif command -v python3 &> /dev/null; then
    PYTHON_CMD="python3"
    print_warning "Using system Python 3 (3.11 recommended for best PyO3 compatibility)"
else
    print_error "Python 3 not found. Please install Python 3.11+"
    print_status "On macOS: brew install python@3.11"
    exit 1
fi

# Set PyO3 Python path
export PYO3_PYTHON=$(which $PYTHON_CMD)
print_status "Using Python: $PYO3_PYTHON"

# Check if faster-whisper is installed
print_status "Checking faster-whisper installation..."
if $PYTHON_CMD -c "import faster_whisper" 2>/dev/null; then
    print_success "faster-whisper is installed"
else
    print_warning "faster-whisper not found. Installing..."
    $PYTHON_CMD -m pip install faster-whisper
    if $PYTHON_CMD -c "import faster_whisper" 2>/dev/null; then
        print_success "faster-whisper installed successfully"
    else
        print_error "Failed to install faster-whisper"
        exit 1
    fi
fi

# Check PyTorch MPS availability on macOS
if [[ "$METAL_AVAILABLE" == true ]]; then
    print_status "Checking Metal Performance Shaders (MPS) availability..."
    if $PYTHON_CMD -c "import torch; print('MPS available:', torch.backends.mps.is_available())" 2>/dev/null | grep -q "True"; then
        print_success "Metal acceleration available via PyTorch MPS"
    else
        print_warning "Metal acceleration not available - will use CPU"
    fi
fi

# Build the Rust application
print_status "Building Rust application..."
if cargo build --release; then
    print_success "Build completed successfully"
else
    print_error "Build failed"
    exit 1
fi

# Run unit tests
print_status "Running unit tests..."
if cargo test; then
    print_success "Unit tests passed"
else
    print_warning "Some unit tests failed"
fi

# Check if test audio file exists
if [[ -f "test.wav" ]]; then
    print_success "Found test audio file: test.wav"
    
    # Run basic transcription test
    print_status "Running basic transcription test..."
    if ./target/release/rust-whisper-app -i test.wav -d cpu -m tiny; then
        print_success "Basic transcription test passed"
    else
        print_warning "Basic transcription test failed"
    fi
    
    # Run Metal vs CPU benchmark if Metal is available
    if [[ "$METAL_AVAILABLE" == true ]]; then
        print_status "Running Metal vs CPU benchmark..."
        if ./target/release/rust-whisper-app -i test.wav --benchmark -o benchmark_results.json; then
            print_success "Benchmark completed successfully"
            print_status "Results saved to benchmark_results.json"
        else
            print_warning "Benchmark failed"
        fi
    fi
    
    # Run integration tests
    print_status "Running integration tests..."
    if cargo test --test integration_tests -- --ignored; then
        print_success "Integration tests passed"
    else
        print_warning "Some integration tests failed (this is expected if dependencies are missing)"
    fi
else
    print_warning "No test audio file found (test.wav)"
    print_status "You can test the application with your own audio files:"
    print_status "  ./target/release/rust-whisper-app -i your_audio.wav"
fi

# Performance recommendations
echo ""
print_status "🏎️  Performance Recommendations:"
echo "  • Use 'mps' device on Apple Silicon Macs for best performance"
echo "  • Use 'float16' compute type for optimal speed/accuracy balance"
echo "  • Start with 'base' model for general use, upgrade to 'medium' for better accuracy"
echo "  • Use 'tiny' model for real-time applications"
echo ""

# Usage examples
print_status "📝 Usage Examples:"
echo "  # Basic transcription with auto device detection:"
echo "  ./target/release/rust-whisper-app -i audio.wav"
echo ""
echo "  # Force Metal acceleration on macOS:"
echo "  ./target/release/rust-whisper-app -i audio.wav -d mps"
echo ""
echo "  # Use medium model for better accuracy:"
echo "  ./target/release/rust-whisper-app -i audio.wav -m medium"
echo ""
echo "  # Run comprehensive benchmark:"
echo "  ./target/release/rust-whisper-app -i audio.wav --benchmark"
echo ""
echo "  # Process multiple files:"
echo "  ./target/release/rust-whisper-app -i /path/to/audio/files/"
echo ""

print_success "Setup completed! 🎉"
print_status "Your Rust + FasterWhisper application is ready to use."

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed. Please install Python 3.8+ first."
    exit 1
fi

echo "✅ Python 3 found: $(python3 --version)"

# Check if pip is available
if ! command -v pip3 &> /dev/null; then
    echo "❌ pip3 is required but not installed. Please install pip first."
    exit 1
fi

echo "✅ pip3 found"

# Install faster-whisper
echo "📦 Installing faster-whisper..."
pip3 install faster-whisper

# Verify installation
echo "🔍 Verifying faster-whisper installation..."
python3 -c "import faster_whisper; print('✅ faster-whisper installed successfully')" || {
    echo "❌ Failed to import faster-whisper. Please check your Python environment."
    exit 1
}

# Check for Metal/MPS support on macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "🍎 Detected macOS - checking for Metal Performance Shaders support..."
    python3 -c "
import torch
if torch.backends.mps.is_available():
    print('✅ Metal Performance Shaders (MPS) available - GPU acceleration ready!')
    print('   Use --device mps for optimal performance')
else:
    print('⚠️  MPS not available - will use CPU inference')
    print('   Consider updating to macOS 12.3+ and PyTorch with MPS support')
" 2>/dev/null || echo "⚠️  Could not check MPS availability"
fi

# Build the Rust application
echo "🔨 Building Rust application..."
cargo build --release

echo ""
echo "🎉 Setup complete!"
echo ""
echo "🚀 Quick Start:"
echo "  # Transcribe an audio file with Metal acceleration:"
echo "  cargo run --release -- -i your_audio.wav -d mps"
echo ""
echo "  # Save results to JSON:"
echo "  cargo run --release -- -i your_audio.wav -o results.json"
echo ""
echo "  # Process a directory of audio files:"
echo "  cargo run --release -- -i /path/to/audio/files/ -o /path/to/outputs/"
echo ""
echo "📖 See README.md for detailed usage instructions and performance tips."
