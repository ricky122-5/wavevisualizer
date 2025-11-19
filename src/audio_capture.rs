use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub fn start_audio_capture(buffer: Arc<Mutex<Vec<f32>>>) -> anyhow::Result<()> {
    let host = cpal::default_host();
    let mut device = host.input_devices()?;
    let our_device = device
        .find(|d| {
            d.name()
                .unwrap_or("unknown".to_string())
                .contains("BlackHole")
        })
        .unwrap();
    println!("{:?}", our_device.default_input_config().unwrap());
    Ok(())
}
