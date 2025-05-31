use std::process::Command;
use std::path::Path;
use crate::cli::Args;

/// Main processing pipeline
pub fn process(args: &Args) {
    println!("🔧 Processing video...");

    // Step 1: Prepare frame extraction
    if !Path::new("frames").exists() {
        std::fs::create_dir("frames").expect("Failed to create frames directory");
    }

    let output_pattern = "frames/frame_%04d.png";

    // ffmpeg command
    let status = Command::new("ffmpeg")
        .args([
            "-i", &args.input,
            "-vf", &format!("fps={},scale={}x{}", args.fps, args.width, args.height),
            output_pattern,
            "-hide_banner", "-loglevel", "error"
        ])
        .status()
        .expect("Failed to execute ffmpeg");

    if status.success() {
        println!("✅ Frames extracted to ./frames/");
    } else {
        eprintln!("❌ Failed to extract frames");
    }
}
