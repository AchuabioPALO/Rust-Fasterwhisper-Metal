---
mode: agent
---
Expected output and any relevant constraints for this task.

Expected output and any relevant constraints for this task.

1. **Task**: Implement a Rust application that uses the `faster-whisper` library to transcribe audio files. not the normal old whisper
2. **Dependencies**: Use the following dependencies in your `Cargo.toml`:
so the idea is that running a rust + fasterwhisper app on metal would lead to crazy speed gains
3. use faster whisper, not the old whisper version from openai
4. **Audio Formats**: The application should support WAV, MP3, FLAC, M4A, and OGG audio formats.
5. **Performance**: The application should be efficient and optimized for performance, especially on macOS with Metal acceleration.
6. **Concurrency**: The application should handle multiple audio files concurrently.
7. **Error Handling**: Provide detailed error messages in case of failures.
8. **Documentation**: The application should be well-documented and easy to use.
9. **Best Practices**: Follow Rust's best practices and idioms.     
10.**Testing**: Include unit tests for critical components. We want to see the speed gains on macOS with Metal acceleration. and document these differences in the README.
11.**size of models**: Use the `base` model for testing, but allow users to specify larger models if they have the hardware. We want to try a medium model on a mac with metal too
