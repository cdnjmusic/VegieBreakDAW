//! The versioned project model.
//!
//! The model is deliberately conservative: positions are in beats, IDs are
//! explicit, and every project is validated before it is saved or loaded into
//! the audio engine.

use std::collections::HashSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::time::Tempo;

pub const PROJECT_SCHEMA_VERSION: u32 = 1;

/// A stable track identifier. IDs are scoped to one project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrackId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClipId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PatternId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChannelId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackKind {
    Audio,
    Instrument,
    Automation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClipSource {
    Audio { asset: String },
    Midi { pattern: PatternId },
    Automation { parameter: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clip {
    pub id: ClipId,
    pub name: String,
    /// Start position in quarter-note beats.
    pub start_beats: f64,
    /// Length in quarter-note beats. Must be positive.
    pub length_beats: f64,
    /// Offset into the source in beats.
    pub source_offset_beats: f64,
    pub gain_db: f64,
    pub source: ClipSource,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub kind: TrackKind,
    pub mute: bool,
    pub solo: bool,
    pub volume_db: f64,
    pub pan: f64,
    pub clips: Vec<Clip>,
    /// Mixer channel this track feeds.
    pub channel: ChannelId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    /// Start in beats relative to the pattern.
    pub start_beats: f64,
    /// Length in beats.
    pub length_beats: f64,
    /// MIDI note number, 0..=127.
    pub key: u8,
    /// Velocity, 0.0..=1.0.
    pub velocity: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pattern {
    pub id: PatternId,
    pub name: String,
    /// Pattern length in beats.
    pub length_beats: f64,
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MixerChannel {
    pub id: ChannelId,
    pub name: String,
    pub volume_db: f64,
    pub pan: f64,
    pub mute: bool,
    pub solo: bool,
    /// Channel IDs this channel sends to.
    pub sends: Vec<ChannelId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeSignature {
    pub numerator: u8,
    pub denominator: u8,
}

impl TimeSignature {
    pub const COMMON: Self = Self {
        numerator: 4,
        denominator: 4,
    };

    #[must_use]
    pub fn is_valid(self) -> bool {
        self.numerator > 0
            && self.numerator <= 32
            && self.denominator.is_power_of_two()
            && (1..=32).contains(&self.denominator)
    }
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self::COMMON
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub schema_version: u32,
    pub name: String,
    pub sample_rate: u32,
    pub tempo_bpm: f64,
    pub time_signature: TimeSignature,
    pub tracks: Vec<Track>,
    pub patterns: Vec<Pattern>,
    pub mixer: Vec<MixerChannel>,
    pub created_with: String,
}

impl Project {
    /// Create an empty, valid project at 48 kHz and 120 BPM.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            schema_version: PROJECT_SCHEMA_VERSION,
            name: name.into(),
            sample_rate: 48_000,
            tempo_bpm: 120.0,
            time_signature: TimeSignature::COMMON,
            tracks: Vec::new(),
            patterns: Vec::new(),
            mixer: vec![MixerChannel {
                id: ChannelId(0),
                name: "Master".to_owned(),
                volume_db: 0.0,
                pan: 0.0,
                mute: false,
                solo: false,
                sends: Vec::new(),
            }],
            created_with: format!("VegieBreakDAW {}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// Validate structural invariants. This does not load external assets.
    pub fn validate(&self) -> Result<(), ProjectError> {
        if self.schema_version != PROJECT_SCHEMA_VERSION {
            return Err(ProjectError::UnsupportedSchema(self.schema_version));
        }
        if self.name.trim().is_empty() {
            return Err(ProjectError::InvalidName);
        }
        if !(8_000..=384_000).contains(&self.sample_rate) {
            return Err(ProjectError::InvalidSampleRate(self.sample_rate));
        }
        Tempo::new(self.tempo_bpm).map_err(|_| ProjectError::InvalidTempo(self.tempo_bpm))?;
        if !self.time_signature.is_valid() {
            return Err(ProjectError::InvalidTimeSignature(self.time_signature));
        }
        if self.mixer.is_empty() {
            return Err(ProjectError::MissingMaster);
        }

        let mut channel_ids = HashSet::new();
        for channel in &self.mixer {
            if !channel_ids.insert(channel.id) {
                return Err(ProjectError::DuplicateChannel(channel.id.0));
            }
            if !channel.volume_db.is_finite() || !channel.pan.is_finite() {
                return Err(ProjectError::InvalidChannelValue(channel.id.0));
            }
            if !(-1.0..=1.0).contains(&channel.pan) {
                return Err(ProjectError::InvalidPan(channel.pan));
            }
        }
        for channel in &self.mixer {
            for send in &channel.sends {
                if !channel_ids.contains(send) {
                    return Err(ProjectError::UnknownChannel(send.0));
                }
            }
        }

        let mut track_ids = HashSet::new();
        for track in &self.tracks {
            if !track_ids.insert(track.id) {
                return Err(ProjectError::DuplicateTrack(track.id.0));
            }
            if track.name.trim().is_empty() {
                return Err(ProjectError::InvalidName);
            }
            if !channel_ids.contains(&track.channel) {
                return Err(ProjectError::UnknownChannel(track.channel.0));
            }
            if !(-1.0..=1.0).contains(&track.pan) {
                return Err(ProjectError::InvalidPan(track.pan));
            }
            let mut clip_ids = HashSet::new();
            for clip in &track.clips {
                if !clip_ids.insert(clip.id) {
                    return Err(ProjectError::DuplicateClip(clip.id.0));
                }
                if !clip.start_beats.is_finite()
                    || !clip.length_beats.is_finite()
                    || !clip.source_offset_beats.is_finite()
                {
                    return Err(ProjectError::InvalidClip(clip.id.0));
                }
                if clip.length_beats <= 0.0 {
                    return Err(ProjectError::InvalidClip(clip.id.0));
                }
            }
        }

        let mut pattern_ids = HashSet::new();
        for pattern in &self.patterns {
            if !pattern_ids.insert(pattern.id) {
                return Err(ProjectError::DuplicatePattern(pattern.id.0));
            }
            if !pattern.length_beats.is_finite() || pattern.length_beats <= 0.0 {
                return Err(ProjectError::InvalidPattern(pattern.id.0));
            }
            for note in &pattern.notes {
                if !note.start_beats.is_finite()
                    || !note.length_beats.is_finite()
                    || note.length_beats <= 0.0
                    || note.key > 127
                    || !(0.0..=1.0).contains(&note.velocity)
                {
                    return Err(ProjectError::InvalidNote(pattern.id.0));
                }
            }
        }

        Ok(())
    }

    /// Serialize the project to pretty JSON.
    pub fn to_json(&self) -> Result<String, ProjectError> {
        self.validate()?;
        serde_json::to_string_pretty(self).map_err(ProjectError::Serialize)
    }

    /// Deserialize and validate a project from JSON.
    pub fn from_json(json: &str) -> Result<Self, ProjectError> {
        let project: Self = serde_json::from_str(json).map_err(ProjectError::Deserialize)?;
        project.validate()?;
        Ok(project)
    }

    /// Save the project as JSON, creating parent directories if needed.
    pub fn save_to_path(&self, path: impl AsRef<Path>) -> Result<(), ProjectError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(ProjectError::Io)?;
        }
        let json = self.to_json()?;
        std::fs::write(path, json).map_err(ProjectError::Io)
    }

    /// Load a project from JSON.
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ProjectError> {
        let json = std::fs::read_to_string(path).map_err(ProjectError::Io)?;
        Self::from_json(&json)
    }

    /// Return the total arrangement length in beats.
    #[must_use]
    pub fn arrangement_length_beats(&self) -> f64 {
        self.tracks
            .iter()
            .flat_map(|track| track.clips.iter())
            .map(|clip| clip.start_beats + clip.length_beats)
            .fold(0.0, f64::max)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("unsupported project schema version {0}")]
    UnsupportedSchema(u32),
    #[error("project name must not be empty")]
    InvalidName,
    #[error("sample rate {0} Hz is outside the supported range")]
    InvalidSampleRate(u32),
    #[error("tempo {0} BPM is invalid")]
    InvalidTempo(f64),
    #[error("time signature {0:?} is invalid")]
    InvalidTimeSignature(TimeSignature),
    #[error("project has no master mixer channel")]
    MissingMaster,
    #[error("duplicate track id {0}")]
    DuplicateTrack(u64),
    #[error("duplicate clip id {0}")]
    DuplicateClip(u64),
    #[error("duplicate pattern id {0}")]
    DuplicatePattern(u64),
    #[error("duplicate mixer channel id {0}")]
    DuplicateChannel(u64),
    #[error("track {0} references an unknown mixer channel")]
    UnknownChannel(u64),
    #[error("pan value {0} is outside -1.0..=1.0")]
    InvalidPan(f64),
    #[error("mixer channel {0} has invalid values")]
    InvalidChannelValue(u64),
    #[error("clip {0} is invalid")]
    InvalidClip(u64),
    #[error("pattern {0} is invalid")]
    InvalidPattern(u64),
    #[error("pattern {0} contains an invalid note")]
    InvalidNote(u64),
    #[error("project serialization failed: {0}")]
    Serialize(#[source] serde_json::Error),
    #[error("project deserialization failed: {0}")]
    Deserialize(#[source] serde_json::Error),
    #[error("project I/O failed: {0}")]
    Io(#[source] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_project() -> Project {
        let mut project = Project::new("Demo");
        project.tracks.push(Track {
            id: TrackId(1),
            name: "Lead".to_owned(),
            kind: TrackKind::Instrument,
            mute: false,
            solo: false,
            volume_db: -3.0,
            pan: 0.0,
            channel: ChannelId(0),
            clips: vec![Clip {
                id: ClipId(1),
                name: "Verse".to_owned(),
                start_beats: 0.0,
                length_beats: 16.0,
                source_offset_beats: 0.0,
                gain_db: 0.0,
                source: ClipSource::Midi {
                    pattern: PatternId(1),
                },
            }],
        });
        project.patterns.push(Pattern {
            id: PatternId(1),
            name: "Verse".to_owned(),
            length_beats: 16.0,
            notes: vec![Note {
                start_beats: 0.0,
                length_beats: 1.0,
                key: 60,
                velocity: 0.8,
            }],
        });
        project
    }

    #[test]
    fn default_project_is_valid() {
        Project::new("Test").validate().unwrap();
    }

    #[test]
    fn round_trips_through_json() {
        let project = sample_project();
        let json = project.to_json().unwrap();
        let restored = Project::from_json(&json).unwrap();
        assert_eq!(project, restored);
        assert!((restored.arrangement_length_beats() - 16.0).abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_bad_tempo() {
        let mut project = Project::new("Test");
        project.tempo_bpm = 0.0;
        assert!(matches!(
            project.validate(),
            Err(ProjectError::InvalidTempo(0.0))
        ));
    }

    #[test]
    fn rejects_duplicate_tracks() {
        let mut project = sample_project();
        let mut duplicate = project.tracks[0].clone();
        duplicate.clips.clear();
        project.tracks.push(duplicate);
        assert!(matches!(
            project.validate(),
            Err(ProjectError::DuplicateTrack(1))
        ));
    }

    #[test]
    fn rejects_unknown_send() {
        let mut project = Project::new("Test");
        project.mixer[0].sends.push(ChannelId(99));
        assert!(matches!(
            project.validate(),
            Err(ProjectError::UnknownChannel(99))
        ));
    }

    #[test]
    fn saves_and_loads_from_disk() {
        let project = sample_project();
        let dir = std::env::temp_dir().join(format!(
            "vegiebreak-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let path = dir.join("demo.vbproj");
        project.save_to_path(&path).unwrap();
        let restored = Project::load_from_path(&path).unwrap();
        assert_eq!(project, restored);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
