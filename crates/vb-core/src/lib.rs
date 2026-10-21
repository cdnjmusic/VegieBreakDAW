#![forbid(unsafe_code)]
#![doc = "Project model and timing primitives for VegieBreakDAW."]

pub mod project;
pub mod time;

pub use project::{
    Clip, ClipSource, MixerChannel, Note, PROJECT_SCHEMA_VERSION, Pattern, Project, ProjectError,
    TimeSignature, Track, TrackKind,
};
pub use time::{Beats, Tempo};
