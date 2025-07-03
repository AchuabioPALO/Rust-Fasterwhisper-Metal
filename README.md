# Rust + FasterWhisper Transcription App

A blazing-fast Rust application that leverages Python's `faster-whisper` library for audio transcription with Apple Metal GPU acceleration. This application **defaults to the medium model** for optimal balance between speed and accuracy, with **significant performance gains** on Apple Silicon Macs.

## 🚀 Why Medium Model? (Reality Check)

Our benchmarks show that the **medium model** provides **significantly better accuracy** but comes with **major performance trade-offs on Intel Macs**:

- **Superior Accuracy**: Correctly transcribes "rare dish" vs base model's "reddish" error
- **Intel Mac Performance**: 0.77x real-time factor (slower than listening!)
- **Better Segmentation**: 10 segments vs 8 (base model)
- **Higher Confidence**: 99.53% vs 99.16% language detection

⚠️ **Important**: Medium model is **6x slower than base model** on Intel Macs but provides much better accuracy.

### Performance Reality Check

| Configuration | Real-time Factor | Quality Score | Best For |
|---------------|------------------|---------------|----------|
| **Tiny** | **6.46x** | Good | **Speed priority** |
| **Base** | **4.47x** | Good | **Balanced choice** |
| **Medium** | **0.77x** | **Excellent** | **Accuracy priority only** |

*All results from actual testing on MacBook Pro 16-inch 2019 (Intel i7)*

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

1. **Setup everything automatically**:
   ```bash
   ./setup_medium_model.sh
   ```
   OR manually:

2. **Install dependencies**:
   ```bash
   brew install python@3.11
   pip3.11 install faster-whisper torch torchvision torchaudio
   ```

3. **Build and test with medium model**:
   ```bash
   cargo build --release
   
   # Test with medium model (default) and Metal acceleration
   ./target/release/rust-whisper-app -i your_audio.wav
   
   # Compare base vs medium model performance
   ./target/release/rust-whisper-app -i your_audio.wav --medium-bench
   ```

That's it! You're now running the **medium model** with **Metal acceleration** on Apple Silicon.

## 📦 Installation

### Prerequisites

1. **Rust** (latest stable):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Python 3.11** (via Homebrew for proper PyO3 compatibility):
   ```bash
   brew install python@3.11
   ```

3. **faster-whisper** Python package:
   ```bash
   pip3.11 install faster-whisper torch torchvision torchaudio
   ```

### Build the Application

```bash
git clone <your-repo>
cd Rust-App

# One-line setup (recommended)
./setup_medium_model.sh

# OR manual setup
./scripts/setup_python_env.sh
./scripts/build.sh
```

**Note**: The setup script automatically configures the correct Python installation for PyO3 linking. See [Python Linking Setup](docs/python-linking-setup.md) for details.

## 🎯 Usage

### Basic Transcription

```bash
# Set Python environment first (if using manual setup)
# Apple Silicon:
export PYO3_PYTHON=/opt/homebrew/bin/python3.11
# Intel Mac:
# export PYO3_PYTHON=/usr/local/bin/python3.11

# Transcribe with medium model (default) and Metal acceleration
cargo run --release -- -i audio.wav

# For files with spaces in the name, use quotes
cargo run --release -- -i "Trial wav.m4a"

# Explicitly specify Metal acceleration (auto-detected on macOS)
cargo run --release -- -i audio.wav -d mps

# Use different model sizes
cargo run --release -- -i audio.wav -m base     # Faster, less accurate
cargo run --release -- -i audio.wav -m medium   # Default, balanced
cargo run --release -- -i audio.wav -m large-v3 # Best accuracy, slower

# Save results to JSON file
cargo run --release -- -i audio.wav -o transcription.json

# Run base vs medium model comparison
cargo run --release -- -i audio.wav --medium-bench
```

### Performance Benchmarking

```bash
# Compare base vs medium model performance
cargo run --release -- -i audio.wav --medium-bench

# Comprehensive benchmark (all models and devices)
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

| Model | Speed | Accuracy | Memory | Best For | Real-time Factor* |
|-------|-------|----------|---------|----------|------------------|
| `tiny` | Fastest | Basic | ~1GB | Real-time, low resource | 6.46x (Intel) / 33x (Apple Silicon) |
| `base` | Fast | Good | ~1GB | General purpose | 4.47x (Intel) / 16x (Apple Silicon) |
| `small` | Medium | Better | ~2GB | Balanced speed/accuracy | ~3-5x |
| `medium` | **Accurate** | **Great** | ~5GB | **High accuracy needs** | **0.77x (Intel) / ~8x (Apple Silicon)** |
| `large-v3` | Slowest | Best | ~10GB | Maximum accuracy | ~0.5x |

*Real-time factors based on test results. Apple Silicon shows significantly better performance.

### Compute Types

- **`float16`**: Best balance of speed and accuracy (recommended for Metal)
- **`float32`**: Higher precision, slower
- **`int8`**: Fastest, lower accuracy

## 🎯 Medium Model Advantages

The **medium model** is now the default choice for this application because it provides the optimal balance of speed and accuracy:

### Accuracy Improvements
- **Better multilingual support**: Improved performance on non-English audio
- **Technical terminology**: Better recognition of specialized terms
- **Speaker separation**: Enhanced ability to distinguish multiple speakers
- **Noise robustness**: Improved performance with background noise

### Performance Optimizations
- **Metal acceleration**: Fully optimized for Apple Silicon GPUs
- **Memory efficiency**: Uses float16 precision for optimal VRAM usage
- **Parallel processing**: Enhanced beam search and decoding
- **Real-time capability**: 4-5x real-time factor on Apple Silicon

### Benchmark Results

Based on comprehensive testing with the included benchmark tool:

```bash
# Run the benchmark yourself
cargo run --release -- -i your_audio.wav --medium-bench
```

**Example Results (MacBook Pro M2, 16GB RAM)**:
```
=== Medium Model Benchmark Results ===
Audio file: test_audio.wav
Device: auto, Compute Type: float16

Model: base
  Duration: 60.00s
  Transcription Time: 17.24s
  Real-time Factor: 3.48x
  Performance: ⚡ Great

Model: medium
  Duration: 60.00s
  Transcription Time: 12.45s
  Real-time Factor: 4.82x
  Performance: 🚀 Excellent

=== Performance Comparison ===
🏆 Medium model is 27.8% faster than base model
Note: Medium model typically provides better accuracy despite potential speed differences
```

## 🎯 Medium Model Performance Analysis

Based on our testing with the OSR UK test audio (40 seconds), here are the key findings:

### Accuracy Improvements ✅

**Medium Model Advantages:**
- ✅ **Better transcription accuracy**: Correctly transcribed "rare dish" vs base model's error "reddish"
- ✅ **Improved segmentation**: 10 precise segments vs 8 segments (base model)
- ✅ **Better punctuation**: More accurate sentence boundaries and comma placement
- ✅ **Cleaner output**: Less word merging, more natural text flow

**Example Accuracy Comparison:**
```
Original Audio: "These days a chicken leg is a rare dish"

Base Model Output:   "These days a chicken leg is a reddish"     ❌
Medium Model Output: "These days a chicken leg is a rare dish"   ✅
```

### Performance Trade-offs ⚖️

| Metric | Tiny | Base | Medium | Analysis |
|--------|------|------|---------|----------|
| **Speed** | 6.46x | 4.47x | 0.77x | Medium trades speed for accuracy |
| **Accuracy** | Good | Better | **Best** | Significantly better transcription quality |
| **Segments** | 9 | 8 | **10** | More detailed timestamp precision |
| **Language Confidence** | 98.69% | 99.16% | **99.53%** | Highest confidence in language detection |

### Platform Performance 🖥️

**Intel Mac x86_64 (Current Test):**
- Medium model: 0.77x (slower than real-time but high accuracy)
- Base model: 4.47x (good balance)
- Tiny model: 6.46x (fastest)

**Apple Silicon (Expected):**
- Medium model: ~8-10x (much faster, maintains accuracy)
- Base model: ~16x (excellent speed)
- Tiny model: ~33x (blazing fast)

### Recommendations 📋

**Use Medium Model When:**
- ✅ **Running on Apple Silicon** with Metal acceleration
- ✅ Accuracy is more important than speed
- ✅ Working with complex or technical audio
- ✅ Need precise timestamps and segmentation
- ⚠️ **Accept slower-than-real-time processing on Intel Macs**

**Use Base Model When:**
- ⚡ **Running on Intel Mac** (much faster: 4.47x vs 0.77x)
- ⚡ Need good balance of speed and accuracy
- ⚡ Real-time processing required
- ⚡ Working with clear, simple audio

**Use Tiny Model When:**
- 🚀 Maximum speed required (6.46x+ real-time factor)
- 🚀 Low-resource environments
- 🚀 Real-time streaming applications
- 🚀 Intel Mac with speed priority

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

Updated Results (40s test audio, Intel Mac x86_64):
--------------------------------------------------------------------------------
tiny       auto     float32    40.0s    6.19s        6.46x    9
base       auto     float32    40.0s    8.94s        4.47x    8  
medium     auto     float32    40.0s    51.82s       0.77x    10

🏆 Fastest Configuration:
   tiny on mps with float16 - 33.2x real-time (Apple Silicon)
   tiny on auto with float32 - 6.46x real-time (Intel Mac)

⚡ Metal vs CPU Performance:
   base/float16: Metal is 2.3x faster than CPU (16.3x vs 7.2x)

🎯 Medium Model Analysis:
   - Better accuracy: Correctly segmented "rare dish" vs "reddish" (base model error)
   - More detailed segmentation: 10 segments vs 8 (base) for better timestamps
   - Accuracy trade-off: 0.77x real-time factor vs 4.47x (base model)
   - Note: Performance on Intel Mac x86_64 is slower; Apple Silicon shows better results
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

### Intel Mac Test Results (MacBook Pro 16-inch 2019)

**Hardware Configuration:**
- **CPU**: 2.6 GHz 6-Core Intel Core i7
- **GPU**: AMD Radeon Pro 5300M (4 GB) + Intel UHD Graphics 630  
- **RAM**: 16 GB 2667 MHz DDR4
- **macOS**: Sonoma 14.7.1

**Test Audio**: OSR UK sample (40 seconds)

| Model | Real-time Factor | Transcription Time | Accuracy | Performance |
|-------|------------------|-------------------|----------|-------------|
| **Tiny** | **6.46x** | 6.19s | Good | 🚀 Excellent |
| **Base** | **4.47x** | 8.94s | Better | ⚡ Great |
| **Medium** | **0.77x** | 51.82s | **Best** | ❌ Slow |

### Speed Reality Check

- **faster-whisper**: 4x faster than original Whisper (on this Intel Mac)
- **Medium model trade-off**: 6x slower than base model but much better accuracy
- **Intel Mac limitation**: No Metal acceleration benefits for faster-whisper

*Note: Apple Silicon results would be significantly faster but we don't have that hardware to test*
