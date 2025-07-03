#!/bin/bash

# Setup script for Rust + FasterWhisper Medium Model Application
# Optimized for Apple Silicon with Metal acceleration

set -e

echo "🚀 Setting up Rust + FasterWhisper Medium Model Application"
echo "============================================================"

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
    print_warning "Not on macOS - Metal acceleration not available, using CPU/CUDA"
    METAL_AVAILABLE=false
fi

# Check for Apple Silicon
if [[ $(uname -m) == "arm64" ]]; then
    print_status "Detected Apple Silicon - optimal performance expected"
    APPLE_SILICON=true
else
    print_warning "Not on Apple Silicon - performance may be limited"
    APPLE_SILICON=false
fi

# Check Rust installation
print_status "Checking Rust installation..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    print_success "Rust found: $RUST_VERSION"
else
    print_error "Rust not found. Please install Rust first:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check Python installation and prioritize Homebrew Python
print_status "Checking Python installation..."
PYTHON_CMD=""

# Prioritize Homebrew Python for proper linking
if [[ -f "/opt/homebrew/bin/python3.11" ]]; then
    PYTHON_CMD="/opt/homebrew/bin/python3.11"
    print_success "Using Homebrew Python 3.11 (Apple Silicon)"
elif [[ -f "/usr/local/bin/python3.11" ]]; then
    PYTHON_CMD="/usr/local/bin/python3.11"
    print_success "Using Homebrew Python 3.11 (Intel)"
elif [[ -f "/opt/homebrew/bin/python3.10" ]]; then
    PYTHON_CMD="/opt/homebrew/bin/python3.10"
    print_success "Using Homebrew Python 3.10 (Apple Silicon)"
elif [[ -f "/usr/local/bin/python3.10" ]]; then
    PYTHON_CMD="/usr/local/bin/python3.10"
    print_success "Using Homebrew Python 3.10 (Intel)"
else
    print_error "No proper Homebrew Python found. Installing Python via Homebrew..."
    if command -v brew &> /dev/null; then
        brew install python@3.11
        PYTHON_CMD="/opt/homebrew/bin/python3.11"
    else
        print_error "Homebrew not found. Please install Homebrew first:"
        echo '/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"'
        exit 1
    fi
fi

PYTHON_VERSION=$($PYTHON_CMD --version 2>&1)
print_success "Python found: $PYTHON_VERSION at $PYTHON_CMD"

# Set PyO3 environment variables
export PYTHON_SYS_EXECUTABLE=$PYTHON_CMD
export PYO3_PYTHON=$PYTHON_CMD
print_status "Setting PYTHON_SYS_EXECUTABLE=$PYTHON_CMD"
print_status "Setting PYO3_PYTHON=$PYTHON_CMD"

# Install Python dependencies
print_status "Installing Python dependencies..."
$PYTHON_CMD -m pip install --upgrade pip

# Install specific packages for faster-whisper with proper audio support
print_status "Installing faster-whisper and dependencies..."
$PYTHON_CMD -m pip install torch torchvision torchaudio
$PYTHON_CMD -m pip install faster-whisper
$PYTHON_CMD -m pip install av  # Required for audio file handling

if [[ $METAL_AVAILABLE == true ]]; then
    # Verify Metal support
    print_status "Verifying Metal acceleration..."
    $PYTHON_CMD -c "
import torch
if hasattr(torch.backends, 'mps') and torch.backends.mps.is_available():
    print('✅ Metal Performance Shaders (MPS) is available')
else:
    print('⚠️  MPS not available, falling back to CPU')
" || print_warning "Could not verify Metal support"
fi

# Verify faster-whisper installation
print_status "Verifying faster-whisper installation..."
$PYTHON_CMD -c "
try:
    import faster_whisper
    print('✅ faster-whisper imported successfully')
    print(f'faster-whisper version: {faster_whisper.__version__}')
except ImportError as e:
    print(f'❌ faster-whisper import failed: {e}')
    exit(1)
"

# Build the Rust application
print_status "Building Rust application..."

# Clean previous builds
cargo clean

# Build with proper Python environment
PYTHON_SYS_EXECUTABLE=$PYTHON_CMD PYO3_PYTHON=$PYTHON_CMD cargo build --release

if [[ $? -eq 0 ]]; then
    print_success "Build completed successfully!"
else
    print_error "Build failed!"
    exit 1
fi

# Test the installation
print_status "Testing installation..."
if [[ -f "OSR_uk_000_0020_8k.wav" ]]; then
    print_status "Testing with sample audio file..."
    
    echo "Testing medium model (default):"
    ./target/release/rust-whisper-app -i OSR_uk_000_0020_8k.wav > /dev/null 2>&1
    
    if [[ $? -eq 0 ]]; then
        print_success "Medium model test passed!"
        
        # Run performance comparison if on Metal
        if [[ $METAL_AVAILABLE == true && $APPLE_SILICON == true ]]; then
            print_status "Running base vs medium model performance comparison..."
            ./target/release/rust-whisper-app -i OSR_uk_000_0020_8k.wav --medium-bench
        fi
    else
        print_warning "Test failed - but application is built. Try manually with: ./target/release/rust-whisper-app -i your_audio.wav"
    fi
else
    print_warning "No sample audio file found. Test manually with: ./target/release/rust-whisper-app -i your_audio.wav"
fi

echo ""
print_success "Setup complete!"
echo ""
echo "🎯 Application is ready with medium model as default"
echo "⚡ Metal acceleration: $(if [[ $METAL_AVAILABLE == true ]]; then echo 'ENABLED'; else echo 'NOT AVAILABLE'; fi)"
echo "🏗️ Apple Silicon optimization: $(if [[ $APPLE_SILICON == true ]]; then echo 'ENABLED'; else echo 'NOT AVAILABLE'; fi)"
echo ""
echo "Quick start:"
echo "  ./target/release/rust-whisper-app -i your_audio.wav"
echo "  ./target/release/rust-whisper-app -i your_audio.wav --medium-bench"
echo ""
echo "For help:"
echo "  ./target/release/rust-whisper-app --help"
