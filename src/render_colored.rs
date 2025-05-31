use image::{RgbImage, Pixel};
use imageproc::drawing::draw_text_mut;
use ab_glyph::{FontArc, Font, PxScale};
use std::fs;
use std::path::{Path, PathBuf};

/// Dark-to-light character ramp
const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";

/// Converts a single image frame to colored ASCII PNG
fn render_colored_frame(
    frame_path: &Path,
    font: &FontArc,
    scale: PxScale,
    out_dir: &Path,
){
    let img = image::open(frame_path).expect("Failed to open frame").to_rgb8();
    let (width, height) = img.dimensions();

    let char_width = scale.x.ceil() as u32;
    let char_height = scale.y.ceil() as u32;

    let mut canvas = RgbImage::new(width * char_width, height * char_height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness = pixel.to_luma()[0] as f32 / 255.0;
            let index = (brightness * (ASCII_CHARS.len() - 1) as f32).round() as usize;
            let ascii_char = ASCII_CHARS[index] as char;

            let draw_x = x * char_width;
            let draw_y = y * char_height;

            draw_text_mut(
                &mut canvas,
                *pixel, // original pixel color
                draw_x as i32,
                draw_y as i32,
                scale,
                font,
                &ascii_char.to_string(),
            );
        }
    }

    let stem = frame_path.file_stem().unwrap().to_string_lossy();
    let out_path = out_dir.join(format!("{stem}.png"));
    canvas.save(out_path).expect("Failed to save image");
}

/// Converts all frames in ./frames to ./ascii_frames_colored
pub fn render_all_colored_frames(scale_factor: f32) {
    let font = FontArc::try_from_slice(include_bytes!("../fonts/JetBrainsMono-Regular.ttf"))
        .expect("Failed to load font");
    let scale = PxScale::from(scale_factor);

    let input_dir = Path::new("frames");
    let output_dir = Path::new("ascii_frames_colored");

    fs::create_dir_all(output_dir).expect("Failed to create output dir");

    let entries: Vec<PathBuf> = fs::read_dir(input_dir)
        .expect("Failed to read frames/")
        .filter_map(|e| {
            let path = e.ok()?.path();
            if path.extension().map(|ext| ext == "png").unwrap_or(false) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    for frame_path in entries {
        render_colored_frame(&frame_path, &font, scale, output_dir);
    }

    println!("✅ Colored ASCII PNGs written to ./ascii_frames_colored/");
}
