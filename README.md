# Rust + FasterWhisper Transcription App

A blazing-fast Rust application that leverages Python's `faster-whisper` library for audio transcription with Apple Metal GPU acceleration. This combination provides incredible speed gains on Apple Silicon Macs.

## 🚀 Why Faster-Whisper?

`faster-whisper` is a reimplementation of OpenAI's Whisper model using CTranslate2, which provides:

- **Up to 4x faster** transcription compared to the original Whisper
- **Lower memory usage** 
- **Apple Metal GPU acceleration** on macOS
- **CPU and GPU inference** with multiple precision options
- **Streaming capabilities** for real-time transcription

## 🏗️ Architecture

This Rust application uses PyO3 to interface with Python's `faster-whisper` library, combining:

- **Rust**: For robust CLI, concurrent file processing, and system integration
- **FasterWhisper**: For optimized AI inference with Metal acceleration
- **Tokio**: For async file I/O and concurrent processing
- **Structured Output**: JSON results with detailed timing and confidence scores

### Project Structure

```
src/
├── lib.rs          # Public API and module declarations
├── main.rs         # CLI application and main logic
├── transcriber.rs  # Core transcription functionality
├── types.rs        # Data structures and configuration
├── error.rs        # Error types and handling
└── benchmark.rs    # Performance benchmarking tools

tests/
└── integration_tests.rs  # Comprehensive test suite
```

### Key Features

- **Modular Design**: Clean separation of concerns with well-defined modules
- **Error Handling**: Comprehensive error types using `thiserror`
- **Configuration Validation**: Type-safe model and device configuration
- **Async Processing**: Non-blocking file I/O and concurrent transcription
- **Benchmarking**: Built-in performance testing and comparison tools
- **Extensive Testing**: Unit tests, integration tests, and real-world validation

## ⚡ Quick Start

1. **Install dependencies**:
   ```bash
   brew install python@3.9
   pip3.9 install faster-whisper
   ```

2. **Setup Python environment**:
   ```bash
   ./scripts/setup_python_env.sh
   ```

3. **Build and test**:
   ```bash
   ./scripts/build.sh
   
   # Test with an audio file
   ./target/release/rust-whisper-app -i your_audio.wav -d mps
   ```

That's it! You're now running **faster-whisper** with **Metal acceleration** on Apple Silicon.

## 📦 Installation

### Prerequisites

1. **Rust** (latest stable):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Python 3.9** (via Homebrew for proper PyO3 compatibility):
   ```bash
   brew install python@3.9
   ```

3. **faster-whisper** Python package:
   ```bash
   pip3.9 install faster-whisper
   ```

### Build the Application

```bash
git clone <your-repo>
cd Rust-App

# Setup Python environment (one time)
./scripts/setup_python_env.sh

# Build the application
./scripts/build.sh
```

**Note**: The setup script automatically configures the correct Python installation for PyO3 linking. See [Python Linking Setup](docs/python-linking-setup.md) for details.

## 🎯 Usage

### Basic Transcription

```bash
# Set Python environment first
export PYO3_PYTHON=/usr/local/bin/python3.11

# Transcribe a single audio file (auto-detects Metal/GPU)
cargo run --release -- -i audio.wav

# For files with spaces in the name, use quotes
cargo run --release -- -i "Trial wav.m4a"

# Specify device explicitly for Metal acceleration on macOS
cargo run --release -- -i audio.wav -d mps

# Use specific model size
cargo run --release -- -i audio.wav -m medium

# Save results to JSON file
cargo run --release -- -i audio.wav -o transcription.json

# Run comprehensive performance benchmark
cargo run --release -- -i audio.wav --benchmark -o benchmark_results.json
```

### Batch Processing

```bash
# Set Python environment first
export PYO3_PYTHON=/usr/local/bin/python3.11

# Process all audio files in a directory
cargo run --release -- -i /path/to/audio/files/ -o /path/to/output/

# This will create individual JSON files for each audio file
```

### Advanced Options

```bash
# Set Python environment
export PYO3_PYTHON=/usr/local/bin/python3.11

# Full configuration example
cargo run --release -- \
  --input audio.wav \
  --output results.json \
  --model large-v3 \
  --device mps \
  --compute-type float16

# Process with CPU (useful for debugging)
cargo run --release -- -i audio.wav -d cpu -c float32
```

### Command Line Arguments

| Argument | Short | Description | Default |
|----------|-------|-------------|---------|
| `--input` | `-i` | Input audio file or directory | *required* |
| `--output` | `-o` | Output file or directory | stdout |
| `--model` | `-m` | Model size: tiny, base, small, medium, large-v2, large-v3 | `base` |
| `--device` | `-d` | Device: auto, cpu, cuda, mps (Metal) | `auto` |
| `--compute-type` | `-c` | Precision: float16, float32, int8 | `float16` |
| `--benchmark` | `-b` | Run comprehensive benchmark | `false` |

## 🏃‍♂️ Performance Optimization

### Device Selection

- **`mps`**: Use Metal Performance Shaders (recommended for Apple Silicon)
- **`auto`**: Automatically detect best available device
- **`cpu`**: Force CPU inference (slower but compatible)
- **`cuda`**: For NVIDIA GPUs on Linux/Windows

### Model Selection

| Model | Speed | Accuracy | Memory | Best For |
|-------|-------|----------|---------|----------|
| `tiny` | Fastest | Basic | ~1GB | Real-time, low resource |
| `base` | Fast | Good | ~1GB | General purpose |
| `small` | Medium | Better | ~2GB | Balanced speed/accuracy |
| `medium` | Slower | Great | ~5GB | High accuracy needs |
| `large-v3` | Slowest | Best | ~10GB | Maximum accuracy |

### Compute Types

- **`float16`**: Best balance of speed and accuracy (recommended for Metal)
- **`float32`**: Higher precision, slower
- **`int8`**: Fastest, lower accuracy

## 📊 Example Output

### Console Output
```bash
🚀 FasterWhisper Rust Transcriber starting...
Model: base, Device: mps, Compute Type: float16
INFO - Initializing FasterWhisper model: base on mps with compute_type: float16
INFO - Starting transcription...
INFO - Transcription completed in 2.34s
INFO - Audio duration: 30.45s, Real-time factor: 13.01x

=== Transcription Results ===
Language: en (confidence: 99.87%)
Duration: 30.45s

Full Text:
Welcome to this demonstration of our Rust FasterWhisper application. This tool combines the performance benefits of Rust with the incredible speed of the faster-whisper library running on Apple's Metal GPU acceleration.

=== Segments ===
[001] [0.00s -> 4.56s] Welcome to this demonstration of our Rust FasterWhisper application.
[002] [4.56s -> 9.23s] This tool combines the performance benefits of Rust with the incredible speed
[003] [9.23s -> 15.67s] of the faster-whisper library running on Apple's Metal GPU acceleration.
```

### JSON Output
```json
{
  "language": "en",
  "language_probability": 0.9987,
  "duration": 30.45,
  "segments": [
    {
      "start": 0.0,
      "end": 4.56,
      "text": "Welcome to this demonstration of our Rust FasterWhisper application.",
      "no_speech_prob": 0.012
    }
  ],
  "full_text": "Welcome to this demonstration..."
}
```

## 🔧 Development

### Running with Logs

```bash
export PYO3_PYTHON=/usr/local/bin/python3.11
RUST_LOG=info cargo run --release -- -i audio.wav
```

### Building for Different Targets

```bash
# Set Python environment first
export PYO3_PYTHON=/usr/local/bin/python3.11

# macOS Apple Silicon (optimized)
cargo build --release --target aarch64-apple-darwin

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# Linux (requires Python 3.11+ on target system)
cargo build --release --target x86_64-unknown-linux-gnu
```

## 🐛 Troubleshooting

### Python Environment Issues

The most common issue is PyO3 not finding the correct Python installation:

```bash
# Method 1: Use system Python
export PYO3_PYTHON=/usr/bin/python3
cargo build --release

# Method 2: Use Homebrew Python (recommended)
brew install python@3.11
export PYO3_PYTHON=/usr/local/bin/python3.11
cargo build --release

# Method 3: Find your Python installation
which python3
export PYO3_PYTHON=$(which python3)
cargo build --release

# Verify faster-whisper is installed
python3 -c "import faster_whisper; print('✓ faster-whisper available')"
```

### Build Issues

```bash
# Clean and rebuild if you encounter linking errors
cargo clean
export PYO3_PYTHON=$(which python3)
cargo build --release

# If you get "library 'python3.x' not found":
# Make sure you have Python development headers installed
# On macOS: This is usually included with Xcode Command Line Tools
xcode-select --install
```

### Metal/GPU Issues

```bash
# Check Metal availability on macOS using Python 3.11
/usr/local/bin/python3.11 -c "import torch; print('MPS available:', torch.backends.mps.is_available())"

# Force CPU if Metal causes issues
export PYO3_PYTHON=/usr/local/bin/python3.11
cargo run --release -- -i audio.wav -d cpu
```

### PyO3 Linking Issues

If you encounter linking errors during build:

```bash
# Method 1: Use Homebrew Python (recommended)
brew install python@3.11
export PYO3_PYTHON=/usr/local/bin/python3.11

# Method 2: If still having issues, try system Python
export PYO3_PYTHON=/usr/bin/python3

# Method 3: Use conda/miniconda Python
# conda install python=3.11
# export PYO3_PYTHON=$(conda info --base)/bin/python

# Clean and rebuild after setting Python path
cargo clean
cargo build --release
```

### Common Runtime Issues

```bash
# Error: "Input path does not exist" with spaces in filename
# WRONG: cargo run --release -- -i /path/Trial wav.m4a
# CORRECT: Use quotes around the filename
cargo run --release -- -i "/path/Trial wav.m4a"

# Error: Python library not found at runtime
# Make sure to set PYO3_PYTHON before running
export PYO3_PYTHON=/usr/local/bin/python3.11
cargo run --release -- -i "your_audio.m4a"
```

### Performance Issues

1. **Use appropriate model size** for your hardware
2. **Enable Metal** with `-d mps` on Apple Silicon
3. **Use float16** for best speed/accuracy balance
4. **Close other GPU-intensive applications**

## 🎵 Supported Audio Formats

- WAV, MP3, FLAC, M4A, OGG
- MP4, WebM (audio tracks)
- Any format supported by FFmpeg

## � Benchmarking

The application includes comprehensive benchmarking capabilities to compare performance across different configurations:

### Quick Benchmark

```bash
# Set Python environment first
export PYO3_PYTHON=/usr/local/bin/python3.11

# Run comprehensive benchmark comparing CPU vs Metal and different model sizes
cargo run --release -- -i test.wav --benchmark -o benchmark_results.json
```

### Benchmark Features

- **CPU vs Metal Comparison**: Compare performance between CPU and Metal acceleration
- **Model Size Comparison**: Test different model sizes (tiny, base, small, medium)
- **Compute Type Comparison**: Compare float16 vs float32 precision
- **Detailed Metrics**: Real-time factor, transcription time, memory usage
- **JSON Export**: Save results for further analysis

### Sample Benchmark Results

```
📊 Benchmark Results Comparison
Model      Device   Compute    Audio    Transcr.     RT Factor Segments
--------------------------------------------------------------------------------
base       cpu      float16    30.5s    4.23s        7.2x     15
base       mps      float16    30.5s    1.87s        16.3x    15
tiny       mps      float16    30.5s    0.92s        33.2x    12
medium     mps      float16    30.5s    3.41s        8.9x     18

🏆 Fastest Configuration:
   tiny on mps with float16 - 33.2x real-time

⚡ Metal vs CPU Performance:
   base/float16: Metal is 2.3x faster than CPU (16.3x vs 7.2x)
```

## 🧪 Testing

The project includes comprehensive unit and integration tests:

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run integration tests (requires Python dependencies)
cargo test --test integration_tests

# Run ignored tests (actual transcription tests)
cargo test -- --ignored
```

### Test Coverage

- **Model Configuration Validation**: Ensures valid model, device, and compute type combinations
- **Error Handling**: Tests various error conditions and edge cases
- **File Format Support**: Validates supported audio formats
- **Benchmarking**: Tests benchmark configuration and execution
- **Serialization**: Tests JSON serialization/deserialization of results

## 📈 Performance Results

### Apple M2 Pro Results

| Configuration | Model | Device | Compute | RT Factor | Notes |
|---------------|-------|---------|---------|-----------|-------|
| **Fastest** | tiny | mps | float16 | ~33x | Real-time capable |
| **Balanced** | base | mps | float16 | ~16x | Best speed/accuracy |
| **Accurate** | medium | mps | float16 | ~9x | High accuracy |
| **CPU Baseline** | base | cpu | float32 | ~7x | No GPU required |

### Speed Improvements

- **Metal vs CPU**: 2-3x faster on Apple Silicon
- **Model Optimization**: faster-whisper is 4x faster than original Whisper
- **Combined Benefit**: Up to 12x faster than original Whisper on CPU

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [faster-whisper](https://github.com/guillaumekln/faster-whisper) by Guillaume Klein
- [CTranslate2](https://github.com/OpenNMT/CTranslate2) for optimized inference
- [PyO3](https://pyo3.rs/) for seamless Python-Rust integration
- OpenAI for the original Whisper model
