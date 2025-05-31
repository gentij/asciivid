use clap::Parser;

/// Asciivid – Convert videos into ASCII animations
#[derive(Parser, Debug)]
#[command(name = "asciivid")]
#[command(author = "Gentrit Jashari")]
#[command(version = "0.1.0")]
#[command(about = "Convert videos into ASCII animations", long_about = None)]
pub struct Args {
    /// Input video file path or URL
    #[arg()]
    pub input: String,

    /// Output format: mp4, gif, txt
    #[arg(short, long, default_value = "mp4")]
    pub format: String,

    /// Output width
    #[arg(short = 'x', long, default_value = "120")]
    pub width: u32,

    /// Output height
    #[arg(short = 'y', long, default_value = "60")]
    pub height: u32,

    /// Frames per second
    #[arg(short = 'r', long, default_value = "15")]
    pub fps: u32,

    #[arg(long, default_value_t = false)]
    pub color: bool,
}
