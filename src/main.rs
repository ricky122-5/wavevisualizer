mod audio_capture;
mod spectrum;
mod ui;

use std::process::Command;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Hello, world!");
    let audio_data = Arc::new(Mutex::new(Vec::new()));
    let _a = audio_capture::start_audio_capture(audio_data.clone());
    let b = query_apple_music();
    println!("{:?}", b);
    // let stream = a.unwrap();
    // loop {
    //     let mut lock = audio_data.lock().unwrap();
    //     if lock.len() < 4096 {
    //         continue;
    //     }
    //     let data = lock.clone();
    //     println!("Samples: {:?}", &data[0..5]);
    //     let res = spectrum::compute_spectrum(&data, 48000);
    //     println!("Max Freq: {:?}", res.unwrap().max());
    //     std::thread::sleep(std::time::Duration::from_millis(50));
    // }

    if let Err(e) = ui::start_ui(audio_data) {
        eprintln!("UI Error: {}", e);
    }
}
fn query_apple_music() -> Option<TrackInfo> {
    let script = r#"
        tell application "Music"
            if not it is running then
                return "STOPPED"
            end if

            if player state is not playing then
                return "STOPPED"
            end if

            set t to name of current track
            set a to artist of current track
            set b to album of current track
            set d to duration of current track
            set p to player position
            
            set AppleScript's text item delimiters to "|||"
            return {t, a, b, d, p} as string
        end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .expect("Failed to execute osascript");

    if !output.status.success() {
        println!("Failed to execute osascript");
        return None;
    }

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if result == "STOPPED" {
        println!("Music is not playing");
        return None;
    }

    let track_info: Vec<&str> = result.split("|||").collect();
    if track_info.len() < 5 {
        println!("Failed to parse track info");
        return None;
    }

    Some(TrackInfo {
        name: track_info[0].to_string(),
        artist: track_info[1].to_string(),
        album: track_info[2].to_string(),
        duration: track_info[3].to_string(),
        position: track_info[4].to_string(),
    })
}

#[derive(Clone, Debug)]
pub struct TrackInfo {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub duration: String,
    pub position: String,
}
