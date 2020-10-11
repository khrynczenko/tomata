/// Be aware that most of this module is either copied or based on
/// the `beep` example from the `cpal` crate. For more details
/// go there.
use std::error::Error;
use std::f32::consts::PI;
use std::fmt;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SupportedStreamConfig};
use once_cell::sync::OnceCell;

pub static BEEPER: OnceCell<SoundSystem> = OnceCell::new();

pub struct SoundSystem {
    device: Device,
    config: SupportedStreamConfig,
}

impl fmt::Debug for SoundSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SoundSystem").finish()
    }
}

impl Default for SoundSystem {
    fn default() -> SoundSystem {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("Failed to find a default sound output device.");
        let config = device
            .default_output_config()
            .expect("Could not initialize default sound configuration.");

        SoundSystem { device, config }
    }
}

impl SoundSystem {
    pub fn beep(&self) -> Result<(), Box<dyn Error>> {
        match self.config.sample_format() {
            cpal::SampleFormat::F32 => {
                make_beep_sound::<f32>(&self.device, &self.config.clone().into())?
            }
            cpal::SampleFormat::I16 => {
                make_beep_sound::<i16>(&self.device, &self.config.clone().into())?
            }
            cpal::SampleFormat::U16 => {
                make_beep_sound::<u16>(&self.device, &self.config.clone().into())?
            }
        }
        Ok(())
    }
}

fn make_beep_sound<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> Result<(), Box<dyn Error>>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * PI / sample_rate).sin() * 0.1
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let value: T = cpal::Sample::from::<f32>(&next_value());
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
    )?;
    stream.play()?;
    std::thread::sleep(Duration::from_millis(500));

    Ok(())
}
