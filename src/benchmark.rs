use crate::error::Result;
use crate::transcriber::FasterWhisperTranscriber;
use crate::types::{ModelConfig, TranscriptionResult};
use log::info;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkResult {
    pub model_size: String,
    pub device: String,
    pub compute_type: String,
    pub audio_duration: f64,
    pub transcription_time: f64,
    pub real_time_factor: f64,
    pub memory_usage_mb: Option<f64>,
    pub accuracy_score: Option<f64>,
    pub segments_count: usize,
}

impl BenchmarkResult {
    pub fn from_transcription(config: &ModelConfig, result: &TranscriptionResult) -> Self {
        Self {
            model_size: config.model_size.clone(),
            device: config.device.clone(),
            compute_type: config.compute_type.clone(),
            audio_duration: result.duration,
            transcription_time: result.transcription_time,
            real_time_factor: result.real_time_factor,
            memory_usage_mb: None, // TODO: Implement memory monitoring
            accuracy_score: None,  // TODO: Implement accuracy calculation if reference available
            segments_count: result.segments.len(),
        }
    }
}

pub struct Benchmark {
    configs: Vec<ModelConfig>,
}

impl Benchmark {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
        }
    }

    pub fn add_config(&mut self, config: ModelConfig) {
        self.configs.push(config);
    }

    pub fn add_cpu_vs_metal_comparison(&mut self, model_size: &str, compute_type: &str) {
        // Add CPU configuration
        self.configs
            .push(ModelConfig::new(model_size, "cpu", compute_type));
        // Add Metal/MPS configuration for macOS
        self.configs
            .push(ModelConfig::new(model_size, "mps", compute_type));
    }

    pub fn add_model_size_comparison(&mut self, device: &str, compute_type: &str) {
        let models = ["tiny", "base", "small", "medium"];
        for model in models {
            self.configs
                .push(ModelConfig::new(model, device, compute_type));
        }
    }

    pub fn add_compute_type_comparison(&mut self, model_size: &str, device: &str) {
        let compute_types = ["float16", "float32"];
        for compute_type in compute_types {
            self.configs
                .push(ModelConfig::new(model_size, device, compute_type));
        }
    }

    pub async fn run<P: AsRef<Path>>(&self, audio_path: P) -> Result<Vec<BenchmarkResult>> {
        let audio_path = audio_path.as_ref();
        let mut results = Vec::new();

        info!(
            "Starting benchmark with {} configurations",
            self.configs.len()
        );
        info!("Audio file: {}", audio_path.display());

        for (i, config) in self.configs.iter().enumerate() {
            info!(
                "Running benchmark {}/{}: {} on {} with {}",
                i + 1,
                self.configs.len(),
                config.model_size,
                config.device,
                config.compute_type
            );

            match self.run_single_benchmark(config, audio_path).await {
                Ok(result) => {
                    info!(
                        "✓ Completed: {:.2}s ({}x real-time)",
                        result.transcription_time, result.real_time_factor
                    );
                    results.push(result);
                }
                Err(e) => {
                    eprintln!("✗ Failed {}/{}: {}", config.model_size, config.device, e);
                }
            }
        }

        Ok(results)
    }

    async fn run_single_benchmark<P: AsRef<Path>>(
        &self,
        config: &ModelConfig,
        audio_path: P,
    ) -> Result<BenchmarkResult> {
        let transcriber = FasterWhisperTranscriber::new(config.clone())?;

        // Warm up - not counted in benchmark
        if let Err(e) = transcriber.test_initialization() {
            return Err(e);
        }

        let result = transcriber.transcribe(audio_path)?;
        Ok(BenchmarkResult::from_transcription(config, &result))
    }

    pub fn print_comparison(&self, results: &[BenchmarkResult]) {
        println!("\n📊 Benchmark Results Comparison");
        println!(
            "{:<10} {:<8} {:<10} {:<8} {:<12} {:<8} {:<8}",
            "Model", "Device", "Compute", "Audio", "Transcr.", "RT Factor", "Segments"
        );
        println!("{}", "-".repeat(80));

        for result in results {
            println!(
                "{:<10} {:<8} {:<10} {:<8.1}s {:<12.2}s {:<8.1}x {:<8}",
                result.model_size,
                result.device,
                result.compute_type,
                result.audio_duration,
                result.transcription_time,
                result.real_time_factor,
                result.segments_count
            );
        }

        // Find best performance
        if let Some(fastest) = results.iter().max_by(|a, b| {
            a.real_time_factor
                .partial_cmp(&b.real_time_factor)
                .unwrap_or(std::cmp::Ordering::Equal)
        }) {
            println!("\n🏆 Fastest Configuration:");
            println!(
                "   {} on {} with {} - {:.1}x real-time",
                fastest.model_size, fastest.device, fastest.compute_type, fastest.real_time_factor
            );
        }

        // Compare CPU vs Metal if both available
        let cpu_results: Vec<_> = results.iter().filter(|r| r.device == "cpu").collect();
        let metal_results: Vec<_> = results.iter().filter(|r| r.device == "mps").collect();

        if !cpu_results.is_empty() && !metal_results.is_empty() {
            println!("\n⚡ Metal vs CPU Performance:");
            for cpu in &cpu_results {
                if let Some(metal) = metal_results
                    .iter()
                    .find(|m| m.model_size == cpu.model_size && m.compute_type == cpu.compute_type)
                {
                    let speedup = metal.real_time_factor / cpu.real_time_factor;
                    println!(
                        "   {}/{}: Metal is {:.1}x faster than CPU ({:.1}x vs {:.1}x)",
                        cpu.model_size,
                        cpu.compute_type,
                        speedup,
                        metal.real_time_factor,
                        cpu.real_time_factor
                    );
                }
            }
        }
    }

    pub fn save_results_json<P: AsRef<Path>>(
        &self,
        results: &[BenchmarkResult],
        path: P,
    ) -> Result<()> {
        let json = serde_json::to_string_pretty(results)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

impl Default for Benchmark {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_creation() {
        let mut benchmark = Benchmark::new();
        assert_eq!(benchmark.configs.len(), 0);

        benchmark.add_config(ModelConfig::new("base", "cpu", "float32"));
        assert_eq!(benchmark.configs.len(), 1);
    }

    #[test]
    fn test_cpu_vs_metal_comparison() {
        let mut benchmark = Benchmark::new();
        benchmark.add_cpu_vs_metal_comparison("base", "float16");

        assert_eq!(benchmark.configs.len(), 2);
        assert_eq!(benchmark.configs[0].device, "cpu");
        assert_eq!(benchmark.configs[1].device, "mps");
    }

    #[test]
    fn test_model_size_comparison() {
        let mut benchmark = Benchmark::new();
        benchmark.add_model_size_comparison("auto", "float16");

        assert_eq!(benchmark.configs.len(), 4); // tiny, base, small, medium
    }

    #[test]
    fn test_benchmark_result_creation() {
        let config = ModelConfig::new("base", "mps", "float16");
        let transcription_result = TranscriptionResult {
            language: "en".to_string(),
            language_probability: 0.99,
            duration: 30.0,
            segments: vec![],
            full_text: "Test".to_string(),
            transcription_time: 2.0,
            real_time_factor: 15.0,
        };

        let benchmark_result = BenchmarkResult::from_transcription(&config, &transcription_result);

        assert_eq!(benchmark_result.model_size, "base");
        assert_eq!(benchmark_result.device, "mps");
        assert_eq!(benchmark_result.compute_type, "float16");
        assert_eq!(benchmark_result.real_time_factor, 15.0);
    }
}
