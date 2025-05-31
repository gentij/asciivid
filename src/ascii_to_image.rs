use std::path::Path;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use regex::Regex;

use ab_glyph::{FontArc, Font, ScaleFont, PxScaleFont, PxScale};


pub fn ascii_txt_to_image(input_txt: &str, output_path: &Path, scale_value: f32) {
    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let cleaned_txt = ansi_regex.replace_all(input_txt, "");
    let lines: Vec<&str> = cleaned_txt.lines().collect();

    let width = lines.first().map(|l| l.len()).unwrap_or(0);
    let height = lines.len();

    let font = FontArc::try_from_slice(include_bytes!("../fonts/JetBrainsMono-Regular.ttf"))
    .expect("Failed to load font");

    let scale = PxScale::from(scale_value);

    let char_width = scale.x.ceil() as u32;
    let char_height = scale.y.ceil() as u32;

    let mut image = RgbImage::new(width as u32 * char_width, height as u32 * char_height);
    let white = Rgb([255, 255, 255]);

    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, ch) in line.chars().enumerate() {
            let x = (char_idx as u32) * char_width;
            let y = (line_idx as u32) * char_height;

            draw_text_mut(
                &mut image,
                white,
                x as i32,
                y as i32,
                scale,
                &font,
                &ch.to_string(),
            );
        }
    }

    image.save(output_path).expect("Failed to save image");
}
