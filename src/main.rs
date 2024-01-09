use notify_rust::Notification;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    show_notification("Work Time! Focus!", "Hi! let's do our best today!");
    pomodoro(25, 5).await;
}

async fn pomodoro(work_minutes: u64, break_minutes: u64) {
    loop {
        start_timer(work_minutes, "Work Time! Focus!");
        show_notification("Take a Break!", "Work time is over, start your break.");

        start_timer(break_minutes, "Break Time! Relax!");
        show_notification("Back to Work!", "Break time is over, back to work.");
    }
}

fn start_timer(minutes: u64, message: &str) {
    println!("{}", message);
    let sleep_duration = Duration::from_secs(minutes * 60);
    thread::sleep(sleep_duration);
}

fn show_notification(summary: &str, body: &str) {
    Notification::new()
        .summary(summary)
        .body(body)
        .show()
        .unwrap();

    // Plays a sound only if the runtime environment is macOS
    #[cfg(target_os = "macos")]
    tokio::spawn(async {
        play_sound_at_mac("src/sound.mp3").await;
    });
}

async fn play_sound_at_mac(file_path: &str) {
    let status = tokio::process::Command::new("afplay")
        .arg(file_path)
        .status()
        .await
        .expect("Failed to play sound");

    if status.success() {
        println!("Sound played successfully");
    } else {
        println!("Failed to play sound");
    }
}
