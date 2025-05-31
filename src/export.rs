use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::ascii_to_image::ascii_txt_to_image;

pub fn export_ascii_to_images(scale: f32) {
    let input_dir = Path::new("ascii");
    let output_dir = Path::new("ascii_frames");

    if !output_dir.exists() {
        fs::create_dir(output_dir).expect("Failed to create ascii_frames/");
    }

    let entries: Vec<PathBuf> = fs::read_dir(input_dir)
        .expect("Cannot read ascii/")
        .filter_map(|e| {
            let e = e.ok()?;
            let p = e.path();
            if p.extension().map(|ext| ext == "txt").unwrap_or(false) {
                Some(p)
            } else {
                None
            }
        })
        .collect();

    for entry in entries {
        let content = fs::read_to_string(&entry).expect("Failed to read ASCII .txt");
        let name = entry.file_stem().unwrap().to_string_lossy();
        let out_path = output_dir.join(format!("{name}.png"));
        ascii_txt_to_image(&content, &out_path, scale);
    }

    println!("✅ Exported PNG frames to ./ascii_frames/");
}

pub fn generate_video(output_format: &str) {
    let output_file = match output_format {
        "gif" => "output.gif",
        "mp4" => "output.mp4",
        _ => {
            eprintln!("Unsupported format: {}", output_format);
            return;
        }
    };

    let status = Command::new("ffmpeg")
        .args([
            "-framerate", "15",
            "-i", "ascii_frames/frame_%04d.png",
            "-vf", "pad=ceil(iw/2)*2:ceil(ih/2)*2", // ensures even dimensions
            "-pix_fmt", "yuv420p",
            "-y",
            output_file,
        ])
        .status()
        .expect("Failed to run ffmpeg");

    if status.success() {
        println!("🎬 Exported to {}", output_file);
    } else {
        eprintln!("❌ ffmpeg failed");
    }
}


pub fn generate_video_from_dir(dir: &str, format: &str) {
    let input_pattern = format!("{dir}/frame_%04d.png");
    let output = format!("output.{format}");

    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-framerate", "15",
            "-i", &input_pattern,
            &output,
        ])
        .status()
        .expect("Failed to run ffmpeg");

    if status.success() {
        println!("🎬 Exported to {output}");
    } else {
        eprintln!("❌ ffmpeg failed");
    }
}