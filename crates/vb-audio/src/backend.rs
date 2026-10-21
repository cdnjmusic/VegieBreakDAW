//! Device abstraction.
//!
//! The first production backend will be WASAPI shared/exclusive via `cpal`,
//! with optional ASIO support when the user supplies Steinberg's SDK. The null
//! backend exists so transport, graph, and offline-render tests are deterministic.

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
