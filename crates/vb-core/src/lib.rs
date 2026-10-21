#![forbid(unsafe_code)]
#![doc = "Project model and timing primitives for VegieBreakDAW."]

pub mod project;
pub mod time;
pub mod phase2;
pub mod phase5;
pub mod phase6;

pub use project::{
    Clip, ClipSource, MixerChannel, Note, PROJECT_SCHEMA_VERSION, Pattern, Project, ProjectError,
    TimeSignature, Track, TrackKind,
};
pub use phase2::{EditCommand, UndoStack, ArrangementSnapshot};
pub use phase5::{SynthDevice, SynthEngine, SamplerDevice, DrumMachineDevice, DrumKind};
pub use phase6::{ProfilePoint, PERFORMANCE_TARGETS};
pub use time::{Beats, Tempo};
