//! The audio engine.
//!
//! Phase 0 provides the scheduling shape and a deterministic silence renderer.
//! The real graph, instruments, and plugin slots arrive in later phases.

use crate::backend::{AudioConfig, AudioConfigError};
use crate::transport::Transport;

/// Engine counters. These are cheap snapshots, not a logging API.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AudioEngineStats {
    pub blocks_processed: u64,
    pub frames_processed: u64,
    pub xruns: u64,
}

/// The top-level audio engine.
///
/// `process` is the only method intended for the audio callback. It does not
/// allocate, lock, or perform I/O.
#[derive(Debug)]
pub struct AudioEngine {
    config: AudioConfig,
    transport: Transport,
    stats: AudioEngineStats,
    scratch: Vec<f32>,
}

impl AudioEngine {
    pub fn new(config: AudioConfig, transport: Transport) -> Result<Self, AudioConfigError> {
        config.validate()?;
        let scratch = vec![0.0; config.block_size as usize * usize::from(config.channels)];
        Ok(Self {
            config,
            transport,
            stats: AudioEngineStats::default(),
            scratch,
        })
    }

    #[must_use]
    pub fn config(&self) -> AudioConfig {
        self.config
    }

    #[must_use]
    pub fn transport(&self) -> &Transport {
        &self.transport
    }

    pub fn transport_mut(&mut self) -> &mut Transport {
        &mut self.transport
    }

    #[must_use]
    pub fn stats(&self) -> AudioEngineStats {
        self.stats
    }

    /// Render one block of interleaved audio.
    ///
    /// The output slice must be exactly `block_size * channels` samples. Extra
    /// samples are ignored and missing samples are zero-filled. The current
    /// implementation renders silence while the transport advances, which gives
    /// the rest of the system a deterministic clock to test against.
    pub fn process(&mut self, output: &mut [f32]) {
        let expected = self.scratch.len();
        let written = output.len().min(expected);
        output[..written].fill(0.0);
        if output.len() > written {
            output[written..].fill(0.0);
        }

        self.transport
            .advance(self.config.block_size, self.config.sample_rate);
        self.stats.blocks_processed = self.stats.blocks_processed.saturating_add(1);
        self.stats.frames_processed = self
            .stats
            .frames_processed
            .saturating_add(u64::from(self.config.block_size));
    }

    /// Reset counters without touching the transport.
    pub fn reset_stats(&mut self) {
        self.stats = AudioEngineStats::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_expected_block_size() {
        let mut engine = AudioEngine::new(AudioConfig::DEFAULT, Transport::new(120.0)).unwrap();
        let mut buffer = vec![1.0; 128 * 2];
        engine.process(&mut buffer);
        assert!(buffer.iter().all(|sample| *sample == 0.0));
        assert_eq!(engine.stats().blocks_processed, 1);
        assert_eq!(engine.stats().frames_processed, 128);
    }

    #[test]
    fn silence_renderer_advances_transport() {
        let mut engine = AudioEngine::new(AudioConfig::DEFAULT, Transport::new(120.0)).unwrap();
        engine.transport_mut().play();
        let mut buffer = vec![0.0; 128 * 2];
        for _ in 0..375 {
            engine.process(&mut buffer);
        }
        assert!((engine.transport().position_beats - 2.0).abs() < 1e-9);
    }
}
