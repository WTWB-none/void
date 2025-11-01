use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use hound::WavWriter;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

type Utils = Option<(Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>, cpal::Stream)>;
pub struct Recorder {
    utils: Utils,
    destination: PathBuf,
}

impl Recorder {
    pub fn new(destination: PathBuf) -> Self {
        Recorder {
            utils: None,
            destination,
        }
    }
    pub fn start_recording(&mut self) -> Result<(), anyhow::Error> {
        if self.utils.is_some() {
            error!("already recording");
            return Err(anyhow::Error::msg(
                "Attempted to start recording when already recording!",
            ));
        }

        let host = cpal::default_host();

        let device = host
            .default_input_device()
            .expect("failed to find input device");

        let config = device
            .default_input_config()
            .expect("Failed to get default input config");

        let spec = wav_spec_from_config(&config);
        let writer = hound::WavWriter::create(&self.destination, spec)?;
        let writer = Arc::new(Mutex::new(Some(writer)));

        let writer_2 = writer.clone();

        let err_fn = move |_| {
            error!("error in streams");
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
                error!("unsupported sample");
                return Err(anyhow::Error::msg(format!(
                    "Unsupported sample format '{sample_format}'"
                )));
            }
        };

        stream.play().unwrap();
        self.utils = Some((writer, stream));
        Ok(())
    }
    pub fn stop_recording(&mut self) -> Result<(), anyhow::Error> {
        match self.utils.take() {
            Some((writer, stream)) => {
                stream.pause().unwrap();
                writer.lock().unwrap().take().unwrap().finalize().unwrap();
                Ok(())
            }
            None => {
                error!("stopping recording while not recording");
                Err(anyhow::Error::msg(
                    "Attempted to stop recording when not recording!",
                ))
            }
        }
    }
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
    if let Ok(mut guard) = writer.try_lock()
        && let Some(writer) = guard.as_mut()
    {
        for &sample in input.iter() {
            let sample: U = U::from_sample(sample);
            writer.write_sample(sample).ok();
        }
    }
}
