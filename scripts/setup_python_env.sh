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

# Check if we're in a virtual environment
if [[ "$VIRTUAL_ENV" != "" ]]; then
    echo "✅ Virtual environment detected: $VIRTUAL_ENV"
    # Use the virtual environment's Python
    export PYTHON_SYS_EXECUTABLE="$PYTHON_EXEC"
else
    echo "ℹ️  No virtual environment detected"
    # Try to find the best system Python
    if [[ -f "/opt/homebrew/bin/python3" ]]; then
        echo "Found Homebrew Python"
        export PYTHON_SYS_EXECUTABLE="/opt/homebrew/bin/python3"
    elif [[ -f "/usr/local/bin/python3" ]]; then
        echo "Found /usr/local/bin/python3"
        export PYTHON_SYS_EXECUTABLE="/usr/local/bin/python3"
    else
        echo "Using system Python"
        export PYTHON_SYS_EXECUTABLE="$PYTHON_EXEC"
    fi
fi

echo "🔧 Setting PYTHON_SYS_EXECUTABLE to: $PYTHON_SYS_EXECUTABLE"

# Verify Python has the required modules
echo "🧪 Checking Python configuration..."
python3 -c "import sysconfig; print(f'Python lib dir: {sysconfig.get_config_var(\"LIBDIR\")}')"
python3 -c "import sysconfig; print(f'Python include dir: {sysconfig.get_config_var(\"INCLUDEPY\")}')"

# Check if maturin is installed
if ! python3 -c "import maturin" 2>/dev/null; then
    echo "📦 Installing maturin..."
    python3 -m pip install maturin
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
