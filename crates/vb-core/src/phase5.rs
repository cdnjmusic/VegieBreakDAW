//! Phase 5 — Built-in original devices (clean-room, MIT-compatible).
//! No FL Studio presets, DSP, or assets copied.

/// A polymorphic synth with subtractive and wavetable engines.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SynthDevice {
    pub engine: SynthEngine,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthEngine {
    Subtractive,
    Wavetable,
    Hybrid,
}

impl Default for SynthEngine {
    fn default() -> Self {
        Self::Subtractive
    }
}

/// Original sampler (not copied from FL Studio sampler).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SamplerDevice {
    pub slices: Vec<SamplerSlice>,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SamplerSlice {
    pub start_sample: usize,
    pub length_samples: usize,
    pub root_key: u8,
}

/// Original drum machine.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DrumMachineDevice {
    pub layers: Vec<DrumLayer>,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrumLayer {
    pub kind: DrumKind,
    pub pitch: f32,
    pub decay: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrumKind {
    Kick,
    Snare,
    HiHat,
    Perc,
}
