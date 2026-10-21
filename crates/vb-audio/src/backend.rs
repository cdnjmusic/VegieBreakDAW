//! Device abstraction.
//!
//! The first production backend will be WASAPI shared/exclusive via `cpal`,
//! with optional ASIO support when the user supplies Steinberg's SDK. The null
//! backend exists so transport, graph, and offline-render tests are deterministic.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use serde::{Deserialize, Serialize};

/// Audio device configuration requested by the engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub block_size: u32,
}

impl AudioConfig {
    pub const DEFAULT: Self = Self {
        sample_rate: 48_000,
        channels: 2,
        block_size: 128,
    };

    /// Validate that the configuration can be used by the engine.
    pub fn validate(self) -> Result<(), AudioConfigError> {
        if !(8_000..=384_000).contains(&self.sample_rate) {
            return Err(AudioConfigError::SampleRate(self.sample_rate));
        }
        if self.channels == 0 || self.channels > 32 {
            return Err(AudioConfigError::Channels(self.channels));
        }
        if self.block_size == 0 || self.block_size > 8_192 {
            return Err(AudioConfigError::BlockSize(self.block_size));
        }
        Ok(())
    }

    #[must_use]
    pub fn block_duration_seconds(self) -> f64 {
        f64::from(self.block_size) / f64::from(self.sample_rate)
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AudioConfigError {
    #[error("unsupported sample rate {0} Hz")]
    SampleRate(u32),
    #[error("unsupported channel count {0}")]
    Channels(u16),
    #[error("unsupported block size {0}")]
    BlockSize(u32),
}

/// A discovered audio endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub max_output_channels: u16,
    pub supported_sample_rates: Vec<u32>,
}

/// A running audio backend.
///
/// Implementations own the device callback. `start` must not allocate on the
/// audio thread; all resources are prepared before the device is started.
pub trait AudioBackend {
    type Error;

    fn name(&self) -> &'static str;
    fn config(&self) -> AudioConfig;
    fn start(&mut self) -> Result<(), Self::Error>;
    fn stop(&mut self) -> Result<(), Self::Error>;
    fn is_running(&self) -> bool;
}

/// A deterministic backend used by tests and headless rendering.
#[derive(Debug, Default)]
pub struct NullBackend {
    config: AudioConfig,
    running: bool,
    started: u64,
    stopped: u64,
}

impl NullBackend {
    #[must_use]
    pub fn new(config: AudioConfig) -> Self {
        Self {
            config,
            running: false,
            started: 0,
            stopped: 0,
        }
    }

    #[must_use]
    pub fn start_count(&self) -> u64 {
        self.started
    }

    #[must_use]
    pub fn stop_count(&self) -> u64 {
        self.stopped
    }
}

impl AudioBackend for NullBackend {
    type Error = std::convert::Infallible;

    fn name(&self) -> &'static str {
        "null"
    }

    fn config(&self) -> AudioConfig {
        self.config
    }

    fn start(&mut self) -> Result<(), Self::Error> {
        self.running = true;
        self.started += 1;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        self.running = false;
        self.stopped += 1;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

/// Enumerate available output devices using the default host.
#[must_use]
pub fn list_output_devices() -> Vec<AudioDeviceInfo> {
    let mut results = Vec::new();
    let host = cpal::default_host();
    let Ok(devices) = host.output_devices() else {
        return results;
    };
    for device in devices {
        let name = device.name().unwrap_or_else(|_| "Unknown".into());
        let id = name.clone();
        let default = host
            .default_output_device()
            .is_some_and(|d| d.name().map(|dn| dn == name).unwrap_or(false));
        let max_output_channels = 2;
        results.push(AudioDeviceInfo {
            id,
            name,
            is_default: default,
            max_output_channels,
            supported_sample_rates: vec![44100, 48000],
        });
    }
    results
}

/// A real backend using `cpal`. Currently opens the default output device in
/// WASAPI shared mode and writes silence. It verifies device open/close and
/// keeps the callback allocation-free.
pub struct CpalBackend {
    config: AudioConfig,
    running: bool,
    device: Option<cpal::Device>,
    stream: Option<cpal::Stream>,
}

impl std::fmt::Debug for CpalBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpalBackend")
            .field("config", &self.config)
            .field("running", &self.running)
            .field("device", &self.device.as_ref().map(|_| "some"))
            .field("stream", &self.stream.as_ref().map(|_| "some"))
            .finish()
    }
}

impl CpalBackend {
    #[must_use]
    pub fn new(config: AudioConfig) -> Self {
        Self {
            config,
            running: false,
            device: None,
            stream: None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CpalError {
    #[error("cpal device error: {0}")]
    Device(String),
    #[error("cpal stream error: {0}")]
    Stream(String),
}

impl AudioBackend for CpalBackend {
    type Error = CpalError;

    fn name(&self) -> &'static str {
        "cpal"
    }

    fn config(&self) -> AudioConfig {
        self.config
    }

    fn start(&mut self) -> Result<(), Self::Error> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| CpalError::Device("no default output device".into()))?;
        let default_config = device
            .default_output_config()
            .map_err(|e| CpalError::Device(format!("default_output_config: {e}")))?;
        let channels = std::cmp::min(self.config.channels, default_config.channels());
        let sample_rate = default_config.sample_rate();
        let stream_config = cpal::StreamConfig {
            channels,
            sample_rate,
            buffer_size: cpal::BufferSize::Fixed(self.config.block_size),
        };

        let stream = device
            .build_output_stream(
                &stream_config,
                move |_data: &mut [f32], _info| {
                    // Silence renderer for Step 1; real graph arrives later.
                    for s in _data.iter_mut() {
                        *s = 0.0;
                    }
                },
                |_err| {},
                None,
            )
            .map_err(|e| CpalError::Stream(format!("build_output_stream: {e}")))?;

        stream
            .play()
            .map_err(|e| CpalError::Stream(format!("play: {e}")))?;
        self.device = Some(device);
        self.stream = Some(stream);
        self.running = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        if let Some(s) = self.stream.take() {
            // Drop the stream stops it.
            drop(s);
        }
        self.device = None;
        self.running = false;
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_config() {
        AudioConfig::DEFAULT.validate().unwrap();
        assert!(matches!(
            AudioConfig {
                sample_rate: 0,
                ..AudioConfig::DEFAULT
            }
            .validate(),
            Err(AudioConfigError::SampleRate(0))
        ));
    }

    #[test]
    fn null_backend_lifecycle() {
        let mut backend = NullBackend::new(AudioConfig::DEFAULT);
        assert!(!backend.is_running());
        backend.start().unwrap();
        assert!(backend.is_running());
        backend.stop().unwrap();
        assert!(!backend.is_running());
        assert_eq!(backend.start_count(), 1);
        assert_eq!(backend.stop_count(), 1);
    }
}
