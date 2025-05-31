mod cli;
mod processor;
mod render;
mod player;

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
    player::play_ascii_frames(args.fps);
}
