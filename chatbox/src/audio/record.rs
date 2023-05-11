use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use log::{debug, warn};
use anyhow::{Context, Result};
use std::time::Duration;
use chrono::Utc;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_RECORDING: AtomicBool = AtomicBool::new(false);

pub fn set_recording(is_recording: bool) {
    IS_RECORDING.store(is_recording, Ordering::SeqCst);
}

pub fn input_devices_name() -> Vec<String> {
    let mut names = vec![];
    let host = cpal::default_host();
    match host.input_devices() {
        Ok(devices) => {
            for device in devices {
                if let Ok(name) = device.name() {
                    names.push(name);
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }

    names
}

pub fn record(device_name: &str, path: &str, max_record_time: i64) -> Result<(), anyhow::Error> {
    let host = cpal::default_host();

    let device = if device_name == "default" {
        host.default_input_device()
    } else {
        host.input_devices()?
            .find(|x| x.name().map(|y| y == device_name).unwrap_or(false))
    }.with_context(|| "failed to find input device")?;

    let config = device
        .default_input_config()
        .with_context(|| "Failed to get default input config")?;

    debug!("Default input config: {:?}", config);

    let spec = wav_spec_from_config(&config);
    let writer = hound::WavWriter::create(path, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));

    debug!("Begin recording...");

    let writer_2 = writer.clone();

    let err_fn = move |err| {
        warn!("An error occurred on stream: {}", err);
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
            err_fn,
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )))
        }
    };

    set_recording(true);

    stream.play()?;
    let start_time = Utc::now().timestamp();
    loop {
        if !IS_RECORDING.load(Ordering::SeqCst) {
            break;
        }

        if Utc::now().timestamp() - start_time > max_record_time  {
            break;
        }

        std::thread::sleep(Duration::from_secs(1));
    }
    set_recording(false);

    drop(stream);
    writer.lock().unwrap().take().unwrap().finalize()?;
    debug!("Recording {} complete!", path);
    Ok(())
}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}
