use image::{GenericImageView, Pixel};
use std::fs;
use std::path::Path;
use rayon::prelude::*;

/// Maps brightness to ASCII characters (from dark to light)
const ASCII_CHARS: &[u8] = b"@MW#8&$%B*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

pub fn render_frame_to_ascii_colored(frame_path: &Path, color_enabled: bool) -> String {
    let img = image::open(frame_path).expect("Failed to open frame");

    let (width, height) = img.dimensions();
    let mut ascii_output = String::with_capacity((width * height * 20) as usize); // allocate generously

    for y in 0..height {
        for x in 0..width {
            let rgb = img.get_pixel(x, y).to_rgb();
            let r = rgb[0];
            let g = rgb[1];
            let b = rgb[2];

            // Use brightness for ASCII char
            let brightness = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
            let brightness_norm = brightness / 255.0;
            let index = (brightness_norm * (ASCII_CHARS.len() - 1) as f32).round() as usize;
            let ch = ASCII_CHARS[index] as char;

            if color_enabled {
                // ANSI truecolor escape code
                ascii_output.push_str(&format!("\x1b[38;2;{r};{g};{b}m{}", ch));
            } else {
                ascii_output.push(ch);
            }
        }
        ascii_output.push('\n');
    }

    // After rendering, normalize line lengths
    let lines: Vec<_> = ascii_output.lines().collect();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padded = lines
    .into_iter()
    .map(|line| format!("{:<width$}", line, width = max_len))
    .collect::<Vec<_>>()
    .join("\n");

    padded;

    if color_enabled {
        ascii_output.push_str("\x1b[0m"); // Reset color
    }

    ascii_output
}


pub fn render_all_frames(color_enabled: bool) {
    let frame_dir = Path::new("frames");
    let output_dir = Path::new("ascii");

    if !output_dir.exists() {
        fs::create_dir(output_dir).expect("Failed to create ascii/ dir");
    }

    // Step 1: Collect all .png frame paths
    let frames: Vec<_> = fs::read_dir(frame_dir)
        .expect("Failed to read frames/")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().map(|s| s == "png").unwrap_or(false) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Step 2: Process frames in parallel
    frames.par_iter().for_each(|path| {
        let ascii = render_frame_to_ascii_colored(path, color_enabled);
        let filename = path.file_stem().unwrap().to_string_lossy();
        let out_path = output_dir.join(format!("{filename}.txt"));
        fs::write(out_path, ascii).expect("Failed to write ASCII frame");
    });

    println!("✅ ASCII frames written to ./ascii/ in parallel");
}

