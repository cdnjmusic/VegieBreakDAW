#![forbid(unsafe_code)]
#![doc = "Project model and timing primitives for VegieBreakDAW."]

pub mod project;
pub mod time;
pub mod phase2;

pub use project::{
    Clip, ClipSource, MixerChannel, Note, PROJECT_SCHEMA_VERSION, Pattern, Project, ProjectError,
    TimeSignature, Track, TrackKind,
};
pub use phase2::{EditCommand, UndoStack, ArrangementSnapshot};
pub use time::{Beats, Tempo};
