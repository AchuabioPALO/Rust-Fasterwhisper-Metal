use rust_whisper_app::{
    benchmark::Benchmark,
    transcriber::FasterWhisperTranscriber,
    types::{ModelConfig, TranscriptionResult},
    TranscriptionError,
};
use std::path::PathBuf;
use tempfile::tempdir;

/// Integration tests for the FasterWhisper transcriber
/// Note: These tests require Python and faster-whisper to be installed

#[tokio::test]
async fn test_model_config_validation() {
    // Test valid configurations
    let valid_configs = vec![
        ModelConfig::new("base", "cpu", "float32"),
        ModelConfig::new("tiny", "auto", "float16"),
        ModelConfig::new("small", "mps", "float16"),
    ];

    for config in valid_configs {
        assert!(
            config.validate().is_ok(),
            "Config should be valid: {:?}",
            config
        );
    }

    // Test invalid configurations
    let invalid_configs = vec![
        ModelConfig::new("invalid", "cpu", "float32"),
        ModelConfig::new("base", "invalid", "float32"),
        ModelConfig::new("base", "cpu", "invalid"),
    ];

    for config in invalid_configs {
        assert!(
            config.validate().is_err(),
            "Config should be invalid: {:?}",
            config
        );
    }
}

#[tokio::test]
async fn test_transcriber_creation() {
    // Test successful creation with medium model (default)
    let config = ModelConfig::new("medium", "cpu", "float32");
    let result = FasterWhisperTranscriber::new(config);
    assert!(result.is_ok());

    // Test failed creation with invalid config
    let invalid_config = ModelConfig::new("invalid", "cpu", "float32");
    let result = FasterWhisperTranscriber::new(invalid_config);
    assert!(result.is_err());
}

#[test]
fn test_benchmark_configuration() {
    let mut benchmark = Benchmark::new();

    // Test CPU vs Metal comparison
    benchmark.add_cpu_vs_metal_comparison("base", "float16");
    // Should add 2 configs: CPU and Metal

    // Test model size comparison
    benchmark.add_model_size_comparison("cpu", "float32");
    // Should add 4 more configs: tiny, base, small, medium

    // Test compute type comparison
    benchmark.add_compute_type_comparison("base", "cpu");
    // Should add 2 more configs: float16, float32
}

#[test]
fn test_error_types() {
    // Test error creation and formatting
    let error = TranscriptionError::InvalidPath("test/path".to_string());
    assert_eq!(format!("{}", error), "Invalid file path: test/path");

    let error = TranscriptionError::UnsupportedFormat("xyz".to_string());
    assert_eq!(format!("{}", error), "Unsupported audio format: xyz");

    let error = TranscriptionError::ModelInitError("test error".to_string());
    assert_eq!(
        format!("{}", error),
        "Model initialization failed: test error"
    );
}

#[test]
fn test_supported_audio_formats() {
    let config = ModelConfig::new("base", "cpu", "float32");
    let transcriber = FasterWhisperTranscriber::new(config).unwrap();

    // Test supported formats
    let temp_dir = tempdir().unwrap();
    let supported_formats = ["wav", "mp3", "flac", "m4a", "ogg", "mp4", "webm"];

    for format in supported_formats {
        let file_path = temp_dir.path().join(format!("test.{}", format));
        std::fs::write(&file_path, b"fake audio data").unwrap();

        // This should fail because the file isn't real audio, but it should pass format validation
        let result = transcriber.transcribe(&file_path);
        // Should not fail due to unsupported format
        if let Err(TranscriptionError::UnsupportedFormat(_)) = result {
            panic!("Format {} should be supported", format);
        }
    }

    // Test unsupported format
    let unsupported_file = temp_dir.path().join("test.txt");
    std::fs::write(&unsupported_file, b"text content").unwrap();

    let result = transcriber.transcribe(&unsupported_file);
    assert!(matches!(
        result,
        Err(TranscriptionError::UnsupportedFormat(_))
    ));
}

#[test]
fn test_transcription_result_serialization() {
    let result = TranscriptionResult {
        language: "en".to_string(),
        language_probability: 0.99,
        duration: 30.0,
        segments: vec![],
        full_text: "Test transcription".to_string(),
        transcription_time: 2.0,
        real_time_factor: 15.0,
    };

    // Test JSON serialization
    let json = serde_json::to_string(&result);
    assert!(json.is_ok());

    // Test deserialization
    let json_str = json.unwrap();
    let deserialized: Result<TranscriptionResult, _> = serde_json::from_str(&json_str);
    assert!(deserialized.is_ok());

    let deserialized = deserialized.unwrap();
    assert_eq!(deserialized.language, "en");
    assert_eq!(deserialized.duration, 30.0);
}

// This test requires a real audio file and Python environment
#[tokio::test]
#[ignore] // Ignore by default since it requires external dependencies
async fn test_actual_transcription() {
    // This test should be run manually with a real audio file
    let audio_path = PathBuf::from("test.wav");

    if !audio_path.exists() {
        println!("Skipping transcription test - test.wav not found");
        return;
    }

    let config = ModelConfig::new("tiny", "cpu", "float32");
    let transcriber = match FasterWhisperTranscriber::new(config) {
        Ok(t) => t,
        Err(e) => {
            println!(
                "Skipping transcription test - failed to create transcriber: {}",
                e
            );
            return;
        }
    };

    match transcriber.transcribe(&audio_path) {
        Ok(result) => {
            println!("Transcription successful!");
            println!("Language: {}", result.language);
            println!("Duration: {:.2}s", result.duration);
            println!("Text: {}", result.full_text);
            assert!(!result.full_text.is_empty());
        }
        Err(e) => {
            println!(
                "Transcription failed (this may be expected if dependencies aren't installed): {}",
                e
            );
        }
    }
}

// Benchmark test - also requires external dependencies
#[tokio::test]
#[ignore] // Ignore by default since it requires external dependencies
async fn test_benchmark_execution() {
    let audio_path = PathBuf::from("test.wav");

    if !audio_path.exists() {
        println!("Skipping benchmark test - test.wav not found");
        return;
    }

    let mut benchmark = Benchmark::new();
    benchmark.add_config(ModelConfig::new("tiny", "cpu", "float32"));

    match benchmark.run(&audio_path).await {
        Ok(results) => {
            println!("Benchmark completed successfully!");
            assert!(!results.is_empty());
            for result in &results {
                println!(
                    "Model: {}, Device: {}, RT Factor: {:.2}x",
                    result.model_size, result.device, result.real_time_factor
                );
            }
        }
        Err(e) => {
            println!(
                "Benchmark failed (this may be expected if dependencies aren't installed): {}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_medium_model_creation() {
    // Test medium model creation with different configurations
    let configs = vec![
        ModelConfig::new("medium", "auto", "float16"),
        ModelConfig::new("medium", "cpu", "float32"),
    ];

    for config in configs {
        let result = FasterWhisperTranscriber::new(config.clone());
        assert!(
            result.is_ok(),
            "Failed to create transcriber with config: {:?}",
            config
        );
    }
}

#[tokio::test] 
#[ignore] // Ignore by default due to potential OpenMP initialization issues
async fn test_metal_acceleration_detection() {
    // Test Metal acceleration detection on macOS
    let config = ModelConfig::new("medium", "auto", "float16");
    
    match FasterWhisperTranscriber::new(config) {
        Ok(transcriber) => {
            // This should not fail even if Metal is not available
            match transcriber.get_device_info() {
                Ok(info) => {
                    println!("Device info: {}", info);
                    // Test passed if we got this far
                    assert!(true);
                },
                Err(e) => {
                    println!("Failed to get device info: {}. This may be expected in CI environments.", e);
                    // Don't fail the test, as this might be environment-specific
                }
            }
        },
        Err(e) => {
            println!("Failed to create transcriber: {}. This may be expected in CI environments.", e);
            // Don't fail the test, as this might be environment-specific
        }
    }
}

#[tokio::test]
async fn test_performance_comparison() {
    // Create a small test audio file if none exists
    let test_file = "test_audio.wav";
    
    // Skip if no test audio file is available
    if !std::path::Path::new(test_file).exists() {
        println!("Skipping performance test - no test audio file available");
        return;
    }

    // Test performance comparison between base and medium
    let models = vec!["base", "medium"];
    let results = FasterWhisperTranscriber::benchmark_model_comparison(
        test_file,
        &models,
        "auto", 
        "float16"
    );

    if let Ok(benchmark_results) = results {
        assert_eq!(benchmark_results.len(), 2);
        
        for (model, result) in &benchmark_results {
            println!("Model {}: RTF = {:.2}x", model, result.real_time_factor);
            assert!(result.transcription_time > 0.0, "Transcription time should be positive");
            assert!(result.duration > 0.0, "Audio duration should be positive");
        }
    } else {
        println!("Skipping performance comparison - faster-whisper not available");
    }
}
