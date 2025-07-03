use crate::error::{Result, TranscriptionError};
use crate::types::{ModelConfig, TranscriptionResult, TranscriptionSegment};
use log::info;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::path::Path;
use std::time::Instant;

pub struct FasterWhisperTranscriber {
    config: ModelConfig,
}

impl FasterWhisperTranscriber {
    pub fn new(config: ModelConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| TranscriptionError::ModelInitError(e))?;

        Ok(Self { config })
    }

    pub fn from_params(model_size: &str, device: &str, compute_type: &str) -> Result<Self> {
        let config = ModelConfig::new(model_size, device, compute_type);
        Self::new(config)
    }

    pub fn config(&self) -> &ModelConfig {
        &self.config
    }

    pub fn transcribe<P: AsRef<Path>>(&self, audio_path: P) -> Result<TranscriptionResult> {
        let audio_path = audio_path.as_ref();

        // Validate file exists
        if !audio_path.exists() {
            return Err(TranscriptionError::InvalidPath(format!(
                "File does not exist: {}",
                audio_path.display()
            )));
        }

        // Validate audio format
        if let Some(ext) = audio_path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            if !matches!(
                ext.as_str(),
                "wav" | "mp3" | "flac" | "m4a" | "ogg" | "mp4" | "webm"
            ) {
                return Err(TranscriptionError::UnsupportedFormat(ext));
            }
        } else {
            return Err(TranscriptionError::UnsupportedFormat(
                "no extension".to_string(),
            ));
        }

        let audio_path_str = audio_path.to_str().ok_or_else(|| {
            TranscriptionError::InvalidPath("Invalid Unicode in path".to_string())
        })?;

        info!("Starting transcription for: {}", audio_path_str);
        let start_time = Instant::now();

        let result = Python::with_gil(|py| -> Result<TranscriptionResult> {
            // Import faster_whisper
            let faster_whisper = py.import("faster_whisper")
                .map_err(|e| TranscriptionError::ModelInitError(
                    format!("Failed to import faster_whisper. Install with: pip install faster-whisper. Error: {}", e)
                ))?;

            // Create WhisperModel with Metal/GPU acceleration
            let model_kwargs = PyDict::new(py);

            // Map device names for compatibility with faster-whisper
            let device = match self.config.device.as_str() {
                "mps" => "auto",  // faster-whisper auto-detects Metal acceleration
                "cuda" => "auto", // faster-whisper auto-detects CUDA
                "cpu" => "cpu",
                "auto" => "auto",
                _ => "auto",
            };

            model_kwargs.set_item("device", device)?;
            model_kwargs.set_item("compute_type", &self.config.compute_type)?;

            info!(
                "Initializing FasterWhisper model: {} on {} with compute_type: {}",
                self.config.model_size, self.config.device, self.config.compute_type
            );

            let model = faster_whisper
                .getattr("WhisperModel")?
                .call((&self.config.model_size,), Some(model_kwargs))
                .map_err(|e| {
                    TranscriptionError::ModelInitError(format!("Failed to initialize model: {}", e))
                })?;

            // Transcribe with optimized settings for speed and accuracy
            let transcribe_kwargs = PyDict::new(py);
            transcribe_kwargs.set_item("beam_size", 5)?;
            transcribe_kwargs.set_item("word_timestamps", true)?;
            transcribe_kwargs.set_item("vad_filter", true)?;
            transcribe_kwargs.set_item("vad_parameters", PyDict::new(py))?;

            info!("Starting transcription...");
            let result = model
                .call_method("transcribe", (audio_path_str,), Some(transcribe_kwargs))
                .map_err(|e| {
                    TranscriptionError::TranscriptionFailed(format!("Transcription failed: {}", e))
                })?;

            // Extract segments and info
            let segments_iter = result.get_item(0)?;
            let info = result.get_item(1)?;

            // Get language info
            let language = info.getattr("language")?.extract::<String>()?;
            let language_probability = info.getattr("language_probability")?.extract::<f64>()?;
            let duration = info.getattr("duration")?.extract::<f64>()?;

            // Process segments
            let mut segments = Vec::new();
            let mut full_text = String::new();

            for segment in segments_iter.iter()? {
                let segment = segment?;
                let start = segment.getattr("start")?.extract::<f64>()?;
                let end = segment.getattr("end")?.extract::<f64>()?;
                let text = segment.getattr("text")?.extract::<String>()?;
                let no_speech_prob = segment.getattr("no_speech_prob")?.extract::<f64>()?;

                if !full_text.is_empty() {
                    full_text.push(' ');
                }
                full_text.push_str(&text.trim());

                segments.push(TranscriptionSegment {
                    start,
                    end,
                    text: text.trim().to_string(),
                    no_speech_prob,
                });
            }

            let elapsed = start_time.elapsed();
            let transcription_time = elapsed.as_secs_f64();
            let real_time_factor = if transcription_time > 0.0 {
                duration / transcription_time
            } else {
                0.0
            };

            info!("Transcription completed in {:.2}s", transcription_time);
            info!(
                "Audio duration: {:.2}s, Real-time factor: {:.2}x",
                duration, real_time_factor
            );

            Ok(TranscriptionResult {
                language,
                language_probability,
                duration,
                segments,
                full_text,
                transcription_time,
                real_time_factor,
            })
        })?;

        Ok(result)
    }

    /// Test if the model can be initialized successfully
    pub fn test_initialization(&self) -> Result<()> {
        info!("Testing model initialization...");

        Python::with_gil(|py| -> Result<()> {
            let faster_whisper = py.import("faster_whisper").map_err(|e| {
                TranscriptionError::ModelInitError(format!(
                    "Failed to import faster_whisper: {}",
                    e
                ))
            })?;

            let model_kwargs = PyDict::new(py);
            let device = match self.config.device.as_str() {
                "mps" => "auto",
                "cuda" => "auto",
                "cpu" => "cpu",
                "auto" => "auto",
                _ => "auto",
            };

            model_kwargs.set_item("device", device)?;
            model_kwargs.set_item("compute_type", &self.config.compute_type)?;

            let _model = faster_whisper
                .getattr("WhisperModel")?
                .call((&self.config.model_size,), Some(model_kwargs))
                .map_err(|e| {
                    TranscriptionError::ModelInitError(format!("Failed to initialize model: {}", e))
                })?;

            info!("✓ Model initialization successful");
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_model_config_validation() {
        let valid_config = ModelConfig::new("base", "auto", "float16");
        assert!(valid_config.validate().is_ok());

        let invalid_model = ModelConfig::new("invalid", "auto", "float16");
        assert!(invalid_model.validate().is_err());

        let invalid_device = ModelConfig::new("base", "invalid", "float16");
        assert!(invalid_device.validate().is_err());

        let invalid_compute = ModelConfig::new("base", "auto", "invalid");
        assert!(invalid_compute.validate().is_err());
    }

    #[test]
    fn test_transcriber_creation() {
        let config = ModelConfig::new("base", "cpu", "float32");
        let transcriber = FasterWhisperTranscriber::new(config);
        assert!(transcriber.is_ok());

        let invalid_config = ModelConfig::new("invalid", "auto", "float16");
        let invalid_transcriber = FasterWhisperTranscriber::new(invalid_config);
        assert!(invalid_transcriber.is_err());
    }

    #[test]
    fn test_from_params() {
        let transcriber = FasterWhisperTranscriber::from_params("base", "cpu", "float32");
        assert!(transcriber.is_ok());

        let transcriber = transcriber.unwrap();
        let config = transcriber.config();
        assert_eq!(config.model_size, "base");
        assert_eq!(config.device, "cpu");
        assert_eq!(config.compute_type, "float32");
    }

    #[test]
    fn test_file_validation() {
        let config = ModelConfig::new("base", "cpu", "float32");
        let transcriber = FasterWhisperTranscriber::new(config).unwrap();

        // Test non-existent file
        let result = transcriber.transcribe("/nonexistent/file.wav");
        assert!(matches!(result, Err(TranscriptionError::InvalidPath(_))));

        // Test unsupported format
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "test content").unwrap();

        let result = transcriber.transcribe(&file_path);
        assert!(matches!(
            result,
            Err(TranscriptionError::UnsupportedFormat(_))
        ));
    }
}
