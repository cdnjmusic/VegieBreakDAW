#![forbid(unsafe_code)]
#![doc = "Project and media I/O."]

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use vb_core::Project;

/// The kind of media referenced by a project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaKind {
    Wav,
    Flac,
    Aiff,
    Ogg,
    Midi,
}

impl MediaKind {
    /// Infer a media kind from a file extension, case-insensitively.
    #[must_use]
    pub fn from_extension(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?.to_ascii_lowercase();
        match extension.as_str() {
            "wav" | "wave" => Some(Self::Wav),
            "flac" => Some(Self::Flac),
            "aif" | "aiff" => Some(Self::Aiff),
            "ogg" | "oga" => Some(Self::Ogg),
            "mid" | "midi" => Some(Self::Midi),
            _ => None,
        }
    }
}

/// A media file referenced by the project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaAsset {
    pub id: String,
    pub path: PathBuf,
    pub kind: MediaKind,
    pub sample_rate: u32,
    pub channels: u16,
    pub frames: u64,
}

/// Save a project through the canonical project writer.
pub fn save_project(project: &Project, path: &Path) -> Result<(), IoError> {
    project.save_to_path(path).map_err(IoError::Project)
}

/// Load a project through the canonical project reader.
pub fn load_project(path: &Path) -> Result<Project, IoError> {
    Project::load_from_path(path).map_err(IoError::Project)
}

/// A path relative to a project file, if possible. Absolute paths are retained
/// for assets outside the project directory.
#[must_use]
pub fn project_relative(project: &Path, asset: &Path) -> PathBuf {
    let base = project.parent().unwrap_or_else(|| Path::new("."));
    asset
        .strip_prefix(base)
        .map_or_else(|_| asset.to_path_buf(), Path::to_path_buf)
}

#[derive(Debug, thiserror::Error)]
pub enum IoError {
    #[error("project error: {0}")]
    Project(#[from] vb_core::ProjectError),
    #[error("unsupported media format: {0}")]
    UnsupportedFormat(String),
    #[error("media decode failed: {0}")]
    Decode(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_extensions() {
        assert_eq!(
            MediaKind::from_extension(Path::new("song.WAV")),
            Some(MediaKind::Wav)
        );
        assert_eq!(
            MediaKind::from_extension(Path::new("song.flac")),
            Some(MediaKind::Flac)
        );
        assert_eq!(MediaKind::from_extension(Path::new("song.txt")), None);
    }

    #[test]
    fn relative_paths_stay_relative() {
        let path = project_relative(
            Path::new("projects/song.vbproj"),
            Path::new("projects/audio/kick.wav"),
        );
        assert_eq!(path, PathBuf::from("audio/kick.wav"));
    }
}
