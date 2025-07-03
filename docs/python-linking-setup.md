# Python Linking Setup for Rust + PyO3

This document explains how the Python linking issue was resolved for this Rust application.

## Problem
The application was failing to build with the error:
```
ld: library not found for -lpython3.9
clang: error: linker command failed with exit code 1
```

## Root Cause
The PyO3 build system was trying to link against the Xcode Python framework:
```
/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/3.9
```

But this framework didn't exist on the system, causing the linker to fail.

## Solution
We configured the build to use Homebrew Python instead, which has proper library files:

1. **Python Location**: `/usr/local/opt/python@3.9/bin/python3.9`
2. **Library Path**: `/usr/local/opt/python@3.9/Frameworks/Python.framework/Versions/3.9/lib`

## Files Created/Modified

### `.env`
Contains environment variables for the build:
```bash
export PYTHON_SYS_EXECUTABLE=/usr/local/opt/python@3.9/bin/python3.9
export RUSTFLAGS="-L /usr/local/opt/python@3.9/Frameworks/Python.framework/Versions/3.9/lib"
```

### `scripts/setup_python_env.sh`
Automatically detects and configures the Python environment. Features:
- Detects virtual environments
- Falls back to Homebrew or system Python
- Installs maturin if needed
- Creates/updates `.env` file

### `scripts/build.sh`
Build script that ensures proper Python linking:
- Sources environment variables
- Verifies Python executable
- Builds with correct configuration

## Usage

### Quick Build
```bash
source .env && cargo build --release
```

### Using Build Script
```bash
./scripts/build.sh
```

### Setting Up From Scratch
```bash
./scripts/setup_python_env.sh
./scripts/build.sh
```

## Verification
You can verify the Python configuration with:
```bash
/usr/local/opt/python@3.9/bin/python3.9 -c "import sysconfig; print('LIBDIR:', sysconfig.get_config_var('LIBDIR'))"
```

Should output:
```
LIBDIR: /usr/local/opt/python@3.9/Frameworks/Python.framework/Versions/3.9/lib
```

## Troubleshooting

### If Homebrew Python is not available
Install it with:
```bash
brew install python@3.9
```

### If you get permission errors
Make sure the scripts are executable:
```bash
chmod +x scripts/*.sh
```

### If the build still fails
1. Clean previous builds: `cargo clean`
2. Run setup script: `./scripts/setup_python_env.sh`
3. Try building again: `./scripts/build.sh`

## Environment Requirements
- macOS with Homebrew
- Python 3.9 installed via Homebrew
- Rust with PyO3 dependencies
