use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub fn start_audio_capture(buffer: Arc<Mutex<Vec<f32>>>) -> anyhow::Result<cpal::Stream> {
    let host = cpal::default_host();
    let mut device = host.input_devices()?;
    let our_device = device
        .find(|d| {
            d.name()
                .unwrap_or("unknown".to_string())
                .contains("BlackHole")
        })
        .unwrap();
    let config = our_device.default_input_config().unwrap().into();
    let buffer_clone = buffer.clone();
    let stream = our_device.build_input_stream(
        &config,
        move |data: &[f32], _: &_| {
            let mut lock = buffer_clone.lock().unwrap();
            lock.extend_from_slice(data);
            if lock.len() > 4096 {
                let excess = lock.len() - 4096;
                lock.drain(..excess);
            }
        },
        move |err| {
            eprintln!("{:?}", err);
        },
        None,
    )?;
    stream.play()?;
    Ok(stream)
}
