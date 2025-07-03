#!/bin/bash

# Script to set up Python environment for Rust + PyO3 builds
# This fixes the common "library not found for -lpython3.9" error

set -e

echo "🔍 Detecting Python environment..."

# Get the current Python executable
PYTHON_EXEC=$(which python3)
echo "Using Python: $PYTHON_EXEC"

# Get Python version
PYTHON_VERSION=$(python3 --version)
echo "Python version: $PYTHON_VERSION"

# Always prioritize Homebrew Python for PyO3 linking
# Check for Homebrew Python installations first
if [[ -f "/usr/local/opt/python@3.11/bin/python3.11" ]]; then
    echo "✅ Found Homebrew Python 3.11"
    export PYTHON_SYS_EXECUTABLE="/usr/local/opt/python@3.11/bin/python3.11"
elif [[ -f "/opt/homebrew/bin/python3.11" ]]; then
    echo "✅ Found Homebrew Python 3.11 (Apple Silicon)"
    export PYTHON_SYS_EXECUTABLE="/opt/homebrew/bin/python3.11"
elif [[ -f "/usr/local/opt/python@3.10/bin/python3.10" ]]; then
    echo "✅ Found Homebrew Python 3.10"
    export PYTHON_SYS_EXECUTABLE="/usr/local/opt/python@3.10/bin/python3.10"
elif [[ -f "/opt/homebrew/bin/python3.10" ]]; then
    echo "✅ Found Homebrew Python 3.10 (Apple Silicon)"
    export PYTHON_SYS_EXECUTABLE="/opt/homebrew/bin/python3.10"
else
    echo "❌ No proper Homebrew Python found. Please install:"
    echo "   brew install python@3.11"
    exit 1
fi

echo "🔧 Setting PYTHON_SYS_EXECUTABLE to: $PYTHON_SYS_EXECUTABLE"

# Verify Python has the required modules
echo "🧪 Checking Python configuration..."
$PYTHON_SYS_EXECUTABLE -c "import sysconfig; print(f'Python lib dir: {sysconfig.get_config_var(\"LIBDIR\")}')"
$PYTHON_SYS_EXECUTABLE -c "import sysconfig; print(f'Python include dir: {sysconfig.get_config_var(\"INCLUDEPY\")}')"

# Check if maturin is installed
if ! $PYTHON_SYS_EXECUTABLE -c "import maturin" 2>/dev/null; then
    echo "📦 Installing maturin..."
    $PYTHON_SYS_EXECUTABLE -m pip install maturin
else
    echo "✅ maturin is already installed"
fi

# Export the environment variable
echo "export PYTHON_SYS_EXECUTABLE=$PYTHON_SYS_EXECUTABLE" > .env
echo "📝 Created .env file with Python configuration"

echo "✅ Python environment setup complete!"
echo ""
echo "Now you can build with:"
echo "  source .env && cargo build --release"
echo ""
echo "Or run the build script:"
echo "  ./scripts/build.sh"
