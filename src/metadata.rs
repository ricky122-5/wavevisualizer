use std::process::Command;

pub fn query_apple_music() -> Option<TrackInfo> {
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
            set artworkData to raw data of artwork 1 of current track
            set artworkFile to open for access POSIX file "/tmp/cover.jpg" with write permission
            write artworkData to artworkFile
            close access artworkFile
            
            set AppleScript's text item delimiters to "|||"
            return {t, a, b, d, p, "/tmp/cover.jpg"} as string
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
    if track_info.len() < 6 {
        println!("Failed to parse track info");
        return None;
    }

    Some(TrackInfo {
        name: track_info[0].to_string(),
        artist: track_info[1].to_string(),
        album: track_info[2].to_string(),
        duration: track_info[3].to_string(),
        position: track_info[4].to_string(),
        cover: track_info[5].to_string(),
    })
}

#[derive(Clone, Debug)]
pub struct TrackInfo {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub duration: String,
    pub position: String,
    pub cover: String,
}
