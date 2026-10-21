//! Musical time conversion.
//!
//! The project stores positions in beats. Seconds are derived from the current
//! tempo at the point of use, so tempo changes do not rewrite clip data.

use serde::{Deserialize, Serialize};

/// A position or duration measured in quarter-note beats.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Beats(pub f64);

impl Beats {
    pub const ZERO: Self = Self(0.0);

    #[must_use]
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn as_f64(self) -> f64 {
        self.0
    }

    /// Convert seconds to beats at a fixed tempo.
    #[must_use]
    pub fn from_seconds(seconds: f64, tempo: Tempo) -> Self {
        Self(seconds * tempo.bpm() / 60.0)
    }

    /// Convert beats to seconds at a fixed tempo.
    #[must_use]
    pub fn to_seconds(self, tempo: Tempo) -> f64 {
        self.0 * 60.0 / tempo.bpm()
    }

    #[must_use]
    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }
}

/// A validated tempo in beats per minute.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tempo {
    bpm: f64,
}

impl Tempo {
    pub const MIN_BPM: f64 = 1.0;
    pub const MAX_BPM: f64 = 999.0;

    /// Create a tempo, rejecting non-finite or out-of-range values.
    pub fn new(bpm: f64) -> Result<Self, TempoError> {
        if !bpm.is_finite() || !(Self::MIN_BPM..=Self::MAX_BPM).contains(&bpm) {
            return Err(TempoError::OutOfRange(bpm));
        }
        Ok(Self { bpm })
    }

    #[must_use]
    pub fn bpm(self) -> f64 {
        self.bpm
    }

    /// Length of one quarter note in seconds.
    #[must_use]
    pub fn seconds_per_beat(self) -> f64 {
        60.0 / self.bpm
    }
}

impl Default for Tempo {
    fn default() -> Self {
        Self { bpm: 120.0 }
    }
}

impl TryFrom<f64> for Tempo {
    type Error = TempoError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum TempoError {
    #[error("tempo must be finite and between {min} and {max} BPM, got {0}", min = Tempo::MIN_BPM, max = Tempo::MAX_BPM)]
    OutOfRange(f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_beats_and_seconds() {
        let tempo = Tempo::new(120.0).unwrap();
        assert!((Beats::from_seconds(2.0, tempo).as_f64() - 4.0).abs() < 1e-12);
        assert!((Beats(8.0).to_seconds(tempo) - 4.0).abs() < 1e-12);
    }

    #[test]
    fn rejects_invalid_tempo() {
        assert!(Tempo::new(0.0).is_err());
        assert!(Tempo::new(f64::NAN).is_err());
        assert!(Tempo::new(1000.0).is_err());
    }
}
