use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptionSegment {
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub no_speech_prob: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptionResult {
    pub language: String,
    pub language_probability: f64,
    pub duration: f64,
    pub segments: Vec<TranscriptionSegment>,
    pub full_text: String,
    pub transcription_time: f64,
    pub real_time_factor: f64,
}

impl TranscriptionResult {
    pub fn calculate_real_time_factor(&mut self, transcription_time: f64) {
        self.transcription_time = transcription_time;
        self.real_time_factor = if transcription_time > 0.0 {
            self.duration / transcription_time
        } else {
            0.0
        };
    }
}

#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub model_size: String,
    pub device: String,
    pub compute_type: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_size: "medium".to_string(),
            device: "auto".to_string(),
            compute_type: "float16".to_string(),
        }
    }
}

impl ModelConfig {
    pub fn new(model_size: &str, device: &str, compute_type: &str) -> Self {
        Self {
            model_size: model_size.to_string(),
            device: device.to_string(),
            compute_type: compute_type.to_string(),
        }
    }

    pub fn is_valid_model_size(&self) -> bool {
        matches!(
            self.model_size.as_str(),
            "tiny" | "base" | "small" | "medium" | "large-v2" | "large-v3"
        )
    }

    pub fn is_valid_device(&self) -> bool {
        matches!(self.device.as_str(), "auto" | "cpu" | "cuda" | "mps")
    }

    pub fn is_valid_compute_type(&self) -> bool {
        matches!(self.compute_type.as_str(), "float16" | "float32" | "int8")
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.is_valid_model_size() {
            return Err(format!("Invalid model size: {}", self.model_size));
        }
        if !self.is_valid_device() {
            return Err(format!("Invalid device: {}", self.device));
        }
        if !self.is_valid_compute_type() {
            return Err(format!("Invalid compute type: {}", self.compute_type));
        }
        Ok(())
    }
}
