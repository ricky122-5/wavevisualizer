mod audio_capture;
mod metadata;
mod spectrum;
mod ui;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

fn main() {
    println!("Hello, world!");
    let audio_data = Arc::new(Mutex::new(Vec::new()));
    let _a = audio_capture::start_audio_capture(audio_data.clone());
    let b = metadata::query_apple_music();
    let song_info = Arc::new(Mutex::new(None));
    let song_info_clone = song_info.clone();
    std::thread::spawn(move || {
        loop {
            let info = metadata::query_apple_music();
            let mut lock = song_info_clone.lock().unwrap();
            *lock = info;
            drop(lock);
            std::thread::sleep(Duration::from_secs(1));
        }
    });
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

    if let Err(e) = ui::start_ui(audio_data, song_info) {
        eprintln!("UI Error: {}", e);
    }
}
