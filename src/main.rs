use anyhow::Result;
use clap::{Arg, Command};
use futures::future;
use log::{error, info, warn};
use rust_whisper_app::{
    benchmark::Benchmark, transcriber::FasterWhisperTranscriber, types::ModelConfig,
};
use std::path::PathBuf;
use tokio::fs;

async fn transcribe_file(
    transcriber: &FasterWhisperTranscriber,
    input_path: PathBuf,
    output_path: Option<PathBuf>,
) -> Result<()> {
    info!("Processing: {}", input_path.display());

    let result = transcriber
        .transcribe(&input_path)
        .map_err(|e| anyhow::anyhow!("Transcription failed: {}", e))?;
    // Output results
    if let Some(output_path) = output_path {
        let json_output = serde_json::to_string_pretty(&result)?;
        fs::write(&output_path, json_output).await?;
        info!("Results saved to: {}", output_path.display());
    } else {
        // Print to stdout
        println!("\n=== Transcription Results ===");
        println!(
            "Language: {} (confidence: {:.2}%)",
            result.language,
            result.language_probability * 100.0
        );
        println!("Duration: {:.2}s", result.duration);
        println!("Transcription Time: {:.2}s", result.transcription_time);
        println!("Real-time Factor: {:.2}x", result.real_time_factor);
        println!("\nFull Text:\n{}", result.full_text);

        if !result.segments.is_empty() {
            println!("\n=== Segments ===");
            for (i, segment) in result.segments.iter().enumerate() {
                println!(
                    "[{:03}] [{:.2}s -> {:.2}s] {}",
                    i + 1,
                    segment.start,
                    segment.end,
                    segment.text
                );
            }
        }
    }

    Ok(())
}

async fn transcribe_multiple_files(
    transcriber: &FasterWhisperTranscriber,
    input_paths: Vec<PathBuf>,
    output_dir: Option<PathBuf>,
) -> Result<()> {
    info!("Processing {} files concurrently", input_paths.len());

    let futures: Vec<_> = input_paths
        .into_iter()
        .map(|input_path| {
            let output_path = output_dir.as_ref().map(|dir| {
                let mut output_name = input_path.file_stem().unwrap().to_owned();
                output_name.push("_transcription.json");
                dir.join(output_name)
            });

            async move {
                match transcribe_file(transcriber, input_path.clone(), output_path).await {
                    Ok(_) => info!("✓ Completed: {}", input_path.display()),
                    Err(e) => error!("✗ Failed {}: {}", input_path.display(), e),
                }
            }
        })
        .collect();

    future::join_all(futures).await;
    Ok(())
}

async fn run_benchmark(input_path: PathBuf, output_path: Option<PathBuf>) -> Result<()> {
    info!("🚀 Starting comprehensive benchmark...");

    let mut benchmark = Benchmark::new();

    // Add medium vs base comparison - the primary focus
    info!("Adding medium vs base model comparison...");
    benchmark.add_medium_vs_base_comparison("auto", "float16");

    // Add Metal-specific optimizations
    info!("Adding Metal acceleration benchmarks...");
    benchmark.add_metal_optimized_benchmarks();

    // Add CPU vs Metal comparison for base model (for reference)
    info!("Adding CPU vs Metal comparison tests...");
    benchmark.add_cpu_vs_metal_comparison("base", "float16");

    // Add compute type comparison for medium model
    info!("Adding compute type comparison tests...");
    benchmark.add_compute_type_comparison("medium", "auto");

    let results = benchmark
        .run(&input_path)
        .await
        .map_err(|e| anyhow::anyhow!("Benchmark failed: {}", e))?;

    // Print results
    benchmark.print_comparison(&results);

    // Save to JSON if output path provided
    if let Some(output_path) = output_path {
        benchmark
            .save_results_json(&results, &output_path)
            .map_err(|e| anyhow::anyhow!("Failed to save benchmark results: {}", e))?;
        info!("Benchmark results saved to: {}", output_path.display());
    }

    Ok(())
}

async fn run_medium_model_benchmark(
    input_path: PathBuf,
    device: &str,
    compute_type: &str,
) -> Result<()> {
    info!("🚀 Starting medium model benchmark on Metal acceleration...");

    let models = vec!["base", "medium"];
    let results = FasterWhisperTranscriber::benchmark_model_comparison(
        &input_path,
        &models,
        device,
        compute_type,
    )?;

    println!("\n=== Medium Model Benchmark Results ===");
    println!("Audio file: {}", input_path.display());
    println!("Device: {}, Compute Type: {}", device, compute_type);
    println!();

    for (model, result) in &results {
        println!("Model: {}", model);
        println!("  Duration: {:.2}s", result.duration);
        println!("  Transcription Time: {:.2}s", result.transcription_time);
        println!("  Real-time Factor: {:.2}x", result.real_time_factor);
        println!("  Language: {} ({:.1}% confidence)", result.language, result.language_probability * 100.0);

        // Show performance classification
        let performance_class = if result.real_time_factor > 10.0 {
            "🚀 Excellent"
        } else if result.real_time_factor > 5.0 {
            "⚡ Great"
        } else if result.real_time_factor > 2.0 {
            "✅ Good"
        } else if result.real_time_factor > 1.0 {
            "⚠️ Acceptable"
        } else {
            "❌ Slow"
        };

        println!("  Performance: {}", performance_class);
        println!();
    }

    // Calculate improvement
    if results.len() >= 2 {
        let base_rtf = results[0].1.real_time_factor;
        let medium_rtf = results[1].1.real_time_factor;
        let improvement = if base_rtf > 0.0 {
            (medium_rtf - base_rtf) / base_rtf * 100.0
        } else {
            0.0
        };

        println!("=== Performance Comparison ===");
        if improvement > 0.0 {
            println!("🏆 Medium model is {:.1}% faster than base model", improvement);
        } else {
            println!("🐌 Medium model is {:.1}% slower than base model", -improvement);
        }

        let accuracy_note = if medium_rtf > 0.0 {
            "Note: Medium model typically provides better accuracy despite potential speed differences"
        } else {
            "Note: Performance may vary based on audio content and system configuration"
        };
        println!("{}", accuracy_note);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let matches = Command::new("FasterWhisper Rust Transcriber")
        .version("1.0")
        .author("Your Name")
        .about(
            "High-performance audio transcription using faster-whisper with Metal GPU acceleration",
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE/DIR")
                .help("Input audio file or directory")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE/DIR")
                .help("Output file or directory for JSON results"),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_name("SIZE")
                .help("Model size: tiny, base, small, medium, large-v2, large-v3")
                .default_value("medium"),
        )
        .arg(
            Arg::new("device")
                .short('d')
                .long("device")
                .value_name("DEVICE")
                .help("Device: auto, cpu, cuda, mps (Metal Performance Shaders for macOS)")
                .default_value("auto"),
        )
        .arg(
            Arg::new("compute_type")
                .short('c')
                .long("compute-type")
                .value_name("TYPE")
                .help("Compute type: float16, float32, int8")
                .default_value("float16"),
        )
        .arg(
            Arg::new("benchmark")
                .short('b')
                .long("benchmark")
                .action(clap::ArgAction::SetTrue)
                .help(
                    "Run comprehensive benchmark comparing CPU vs Metal and different model sizes",
                ),
        )
        .arg(
            Arg::new("medium_benchmark")
                .long("medium-bench")
                .action(clap::ArgAction::SetTrue)
                .help(
                    "Run specific benchmark comparing base vs medium model on Metal acceleration",
                ),
        )
        .get_matches();

    let input_path = PathBuf::from(matches.get_one::<String>("input").unwrap());
    let output_path = matches.get_one::<String>("output").map(PathBuf::from);
    let model_size = matches.get_one::<String>("model").unwrap();
    let device = matches.get_one::<String>("device").unwrap();
    let compute_type = matches.get_one::<String>("compute_type").unwrap();
    let run_benchmark_mode = matches.get_flag("benchmark");
    let medium_benchmark = matches.get_flag("medium_benchmark");

    if run_benchmark_mode {
        if input_path.is_file() {
            return run_benchmark(input_path, output_path).await;
        } else {
            error!("Benchmark mode requires a single audio file as input");
            std::process::exit(1);
        }
    }

    if medium_benchmark {
        if input_path.is_file() {
            return run_medium_model_benchmark(input_path, device, compute_type).await;
        } else {
            error!("Medium benchmark mode requires a single audio file as input");
            std::process::exit(1);
        }
    }

    // Initialize the transcriber
    let config = ModelConfig::new(model_size, device, compute_type);
    let transcriber = FasterWhisperTranscriber::new(config)
        .map_err(|e| anyhow::anyhow!("Failed to create transcriber: {}", e))?;

    info!("🚀 FasterWhisper Rust Transcriber starting...");
    info!(
        "Model: {}, Device: {}, Compute Type: {}",
        model_size, device, compute_type
    );

    if input_path.is_file() {
        // Single file
        transcribe_file(&transcriber, input_path, output_path).await?;
    } else if input_path.is_dir() {
        // Directory - find all audio files
        let mut audio_files = Vec::new();
        let mut entries = fs::read_dir(&input_path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if matches!(
                    ext.as_str(),
                    "wav" | "mp3" | "flac" | "m4a" | "ogg" | "mp4" | "webm"
                ) {
                    audio_files.push(path);
                }
            }
        }

        if audio_files.is_empty() {
            warn!(
                "No audio files found in directory: {}",
                input_path.display()
            );
            return Ok(());
        }

        info!("Found {} audio files", audio_files.len());
        transcribe_multiple_files(&transcriber, audio_files, output_path).await?;
    } else {
        error!("Input path does not exist: {}", input_path.display());
        std::process::exit(1);
    }

    info!("🎉 All transcriptions completed successfully!");
    Ok(())
}
