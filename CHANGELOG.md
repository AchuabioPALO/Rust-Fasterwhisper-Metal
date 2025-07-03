# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-07-03

### Security
- Upgraded PyO3 from 0.18 to 0.24 to fix RUSTSEC-2025-0020 vulnerability

### Fixed
- Updated PyO3 API usage to use references for Python dictionaries
- Replaced deprecated `.iter()` with `.try_iter()` for PyO3 compatibility
- Fixed CI workflow to properly set the Python environment
- Added KMP_DUPLICATE_LIB_OK=TRUE to fix OpenMP library conflicts
- Enhanced Python environment detection for Python 3.11

### Changed
- Improved support for Apple Silicon Macs with Metal GPU acceleration
- Updated CI workflow to use Python 3.11

## [0.1.0] - 2025-07-01

### Added
- Initial implementation of Rust + faster-whisper transcription application
- Support for multiple audio formats (WAV, MP3, FLAC, M4A, OGG)
- Apple Metal GPU acceleration via MPS (Metal Performance Shaders)
- Multiple Whisper model sizes (tiny, base, small, medium, large-v2, large-v3)
- Async file processing with Tokio
- Concurrent transcription of multiple files
- JSON output with detailed timing and confidence scores
- Comprehensive error handling and logging
- CLI interface with clap
- Python environment setup scripts for PyO3 linking
- Benchmarking capabilities comparing CPU vs Metal performance
- Integration tests and documentation

### Technical Features
- PyO3 integration for Python faster-whisper library
- Automatic Python environment detection and configuration
- Support for both CPU and GPU (Metal) inference
- Real-time transcription capabilities (5.75x - 9.21x real-time factor)
- Structured output with precise timestamps
- Language detection with confidence scores
- Cross-platform compatibility (macOS and Linux)

### Performance
- 9.21x real-time speed with Metal acceleration (tiny model)
- 5.75x real-time speed with base model
- 99.16% language confidence for English detection
- Efficient memory usage and processing

### Documentation
- Comprehensive README with installation and usage instructions
- Python linking troubleshooting guide
- Test results and performance benchmarks
- API documentation and examples
