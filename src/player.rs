use std::{fs, thread, time, sync::atomic::{AtomicBool, Ordering}, sync::Arc};
use std::path::Path;
use std::io::{self, Write};


/// Reads and plays ASCII frames from ./ascii/
pub fn play_ascii_frames(fps: u32) {
    let should_run = Arc::new(AtomicBool::new(true));
    let ctrlc_flag = should_run.clone();

    // Setup Ctrl+C handler
    ctrlc::set_handler(move || {
        ctrlc_flag.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    let frame_delay = time::Duration::from_millis(1000 / fps as u64);
    let ascii_dir = Path::new("ascii");

    let mut frames: Vec<_> = fs::read_dir(ascii_dir)
        .expect("Failed to read ascii/ directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().map(|ext| ext == "txt").unwrap_or(false) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    frames.sort();

    println!("▶️ Playing {} ASCII frames at {} FPS", frames.len(), fps);

    for frame_path in frames {
        if !should_run.load(Ordering::SeqCst) {
            println!("\n⛔ Interrupted by user");
            break;
        }

        if let Ok(frame_text) = fs::read_to_string(&frame_path) {
            print!("\x1B[2J\x1B[H"); // Clear terminal
            print!("{}", frame_text);
            io::stdout().flush().unwrap();
            thread::sleep(frame_delay);
        }
    }

    println!("\n✅ Playback ended");
}

