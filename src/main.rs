mod cli;
mod processor;
mod render;
mod player;
mod ascii_to_image;
mod export;
mod render_colored;

use cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("🎬 Input: {}", args.input);
    println!("📼 Format: {}", args.format);
    println!("📐 Size: {}x{}", args.width, args.height);
    println!("🎞️ FPS: {}", args.fps);

    processor::process(&args);

    render::render_all_frames(args.color);
    // player::play_ascii_frames(args.fps);

    render_colored::render_all_colored_frames(12.0);
    export::generate_video_from_dir("ascii_frames_colored", "gif");
}

