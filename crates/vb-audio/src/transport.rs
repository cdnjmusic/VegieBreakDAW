//! Musical transport.
//!
//! The transport is owned by the audio engine. UI reads a snapshot and sends
//! commands through a lock-free queue; it never advances the position directly.

use serde::{Deserialize, Serialize};
use vb_core::{Beats, Tempo};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transport {
    pub playing: bool,
    pub recording: bool,
    pub loop_enabled: bool,
    pub position_beats: f64,
    pub loop_start_beats: f64,
    pub loop_end_beats: f64,
    pub tempo_bpm: f64,
}

impl Transport {
    #[must_use]
    pub fn new(tempo_bpm: f64) -> Self {
        Self {
            playing: false,
            recording: false,
            loop_enabled: false,
            position_beats: 0.0,
            loop_start_beats: 0.0,
            loop_end_beats: 16.0,
            tempo_bpm,
        }
    }

    /// Advance the playhead by a number of samples.
    ///
    /// Looping wraps the position into the loop range. The method is
    /// allocation-free and suitable for the audio callback.
    pub fn advance(&mut self, frames: u32, sample_rate: u32) {
        if !self.playing || frames == 0 || sample_rate == 0 {
            return;
        }
        let tempo = match Tempo::new(self.tempo_bpm) {
            Ok(tempo) => tempo,
            Err(_) => return,
        };
        let delta_beats =
            Beats::from_seconds(f64::from(frames) / f64::from(sample_rate), tempo).as_f64();
        self.position_beats += delta_beats;

        if self.loop_enabled
            && self.loop_end_beats > self.loop_start_beats
            && self.position_beats >= self.loop_end_beats
        {
            let length = self.loop_end_beats - self.loop_start_beats;
            if length > 0.0 {
                self.position_beats =
                    self.loop_start_beats + (self.position_beats - self.loop_start_beats) % length;
            }
        }
    }

    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
        self.recording = false;
    }

    pub fn locate(&mut self, beats: f64) {
        if beats.is_finite() && beats >= 0.0 {
            self.position_beats = beats;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advances_at_tempo() {
        let mut transport = Transport::new(120.0);
        transport.play();
        transport.advance(48_000, 48_000);
        assert!((transport.position_beats - 2.0).abs() < 1e-9);
    }

    #[test]
    fn loops_within_range() {
        let mut transport = Transport::new(120.0);
        transport.play();
        transport.loop_enabled = true;
        transport.loop_start_beats = 4.0;
        transport.loop_end_beats = 8.0;
        transport.position_beats = 7.9;
        transport.advance(48_000, 48_000);
        assert!((4.0..8.0).contains(&transport.position_beats));
    }

    #[test]
    fn ignores_negative_locate() {
        let mut transport = Transport::new(120.0);
        transport.locate(4.0);
        transport.locate(-1.0);
        assert!((transport.position_beats - 4.0).abs() < f64::EPSILON);
    }
}
