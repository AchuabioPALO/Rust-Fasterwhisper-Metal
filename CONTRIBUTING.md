# Contributing to Rust Whisper App

Thank you for your interest in contributing! This document provides guidelines and information for contributors.

## 🚀 Quick Start

1. **Fork the repository**
2. **Clone your fork**:
   ```bash
   git clone https://github.com/yourusername/rust-whisper-app.git
   cd rust-whisper-app
   ```
3. **Set up the development environment**:
   ```bash
   ./scripts/setup_python_env.sh
   ./scripts/build.sh
   ```

## 🛠️ Development Setup

### Prerequisites
- Rust (latest stable)
- Python 3.9+ with faster-whisper
- macOS (for Metal acceleration testing)

### Environment Setup
```bash
# Install dependencies
brew install python@3.9
pip3.9 install faster-whisper

# Setup project
./scripts/setup_python_env.sh
cargo build --release
```

## 📝 Coding Standards

### Rust Code Style
- Follow standard Rust formatting with `cargo fmt`
- Use `cargo clippy` for linting
- Maintain comprehensive error handling with `anyhow` and `thiserror`
- Write tests for new functionality

### Python Integration
- Ensure PyO3 compatibility
- Test with both CPU and Metal acceleration
- Validate Python environment setup scripts

### Performance Considerations
- Benchmark new features
- Maintain async/await patterns for I/O operations
- Consider memory usage for large audio files
- Document performance implications

## 🧪 Testing

### Running Tests
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# With sample audio
./target/release/rust-whisper-app --input test_audio.wav --benchmark
```

### Adding Tests
- Write unit tests for new modules
- Add integration tests for major features
- Include performance benchmarks for optimizations
- Test with various audio formats and sizes

## 📚 Documentation

### Code Documentation
- Document all public APIs with rustdoc
- Include examples in documentation
- Keep README.md updated with new features

### Commit Messages
Use conventional commits:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `perf:` for performance improvements
- `test:` for test additions/modifications

Example:
```
feat: add support for MP3 audio format

- Implement MP3 decoding via ffmpeg
- Add tests for MP3 transcription
- Update documentation with MP3 examples
```

## 🐛 Bug Reports

When reporting bugs, please include:
- Rust version (`rustc --version`)
- Python version and faster-whisper version
- Operating system and version
- Audio file format and characteristics
- Steps to reproduce
- Expected vs actual behavior
- Error messages and logs

## 💡 Feature Requests

For new features, please:
- Check existing issues and discussions
- Describe the use case and benefits
- Consider implementation approaches
- Discuss performance implications

## 🔍 Code Review Process

1. All changes require a pull request
2. Ensure CI passes (tests, formatting, linting)
3. Include performance benchmarks for significant changes
4. Update documentation as needed
5. Respond to review feedback promptly

## 🏆 Recognition

Contributors will be recognized in:
- CHANGELOG.md for significant contributions
- README.md contributors section
- Release notes

## ❓ Questions?

- Open an issue for technical questions
- Check existing documentation first
- Join discussions in pull requests

Thank you for contributing to making Rust + faster-whisper transcription even better! 🎉
