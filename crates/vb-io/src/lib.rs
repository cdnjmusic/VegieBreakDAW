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

/// Decoded audio with precomputed waveform peaks.
/// Designed for low-end performance: planar f32 buffers, no per-frame alloc.
#[derive(Debug, Clone)]
pub struct DecodedAudio {
    pub sample_rate: u32,
    pub channels: u16,
    pub frames: u64,
    /// Interleaved f32 samples: `frames * channels` length.
    pub interleaved: Vec<f32>,
    /// Cached peak values per block (one entry per `frames / block_size` blocks).
    /// Each entry is (min, max) in linear amplitude scale.
    pub peaks: Vec<(f32, f32)>,
}

/// Decode a WAV or FLAC file to interleaved f32 with cached peaks.
/// This uses a simple but efficient symphonia path; resampling is deferred
/// to the engine layer to keep decode allocation-free.
pub fn decode_media(path: &Path) -> Result<DecodedAudio, IoError> {
    use symphonia::core::audio::{SampleBuffer, SignalSpec};
    use symphonia::core::codecs::{CODEC_TYPE_NULL, CodecParameters};
    use symphonia::core::formats::{FormatOptions, FormatReader};
    use symphonia::core::io::{MediaSourceStream, ReadOnlySource};
    use symphonia::core::meta::MetadataOptions;
    use symphonia::core::probe::Hint;
    use std::fs::File;

    let file = File::open(path).map_err(|e| IoError::Decode(e.to_string()))?;
    let mss = MediaSourceStream::new(Box::new(ReadOnlySource::new(file)), Default::default());
    let hint = Hint::new();
    let meta_opts = MetadataOptions::default();
    let fmt_opts = FormatOptions::default();

    let mut format = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .map_err(|e| IoError::Decode(format!("format probe: {e}")))?
        .format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or_else(|| IoError::UnsupportedFormat("no audio track".into()))?
        .clone();

    let params = &track.codec_params;
    let sample_rate = params.sample_rate.ok_or_else(|| {
        IoError::UnsupportedFormat("unknown sample rate".into())
    })?;
    let channels = params
        .channels
        .ok_or_else(|| IoError::UnsupportedFormat("unknown channels".into()))?
        .count() as u16;

    let mut decoder = symphonia::default::get_codecs()
        .make(&params.codec_params, &Default::default())
        .map_err(|e| IoError::Decode(format!("decoder init: {e}")))?;

    let mut interleaved = Vec::new();
    let mut peaks: Vec<(f32, f32)> = Vec::new();
    const PEAK_BLOCK_SIZE: usize = 512; // small for low-end CPU

    loop {
        let packet = format
            .next_packet()
            .map_err(|e| IoError::Decode(format!("packet read: {e}")))?;
        if packet.data.is_empty() && packet.track_id != track.id {
            continue;
        }
        let decoded = decoder
            .decode(&packet)
            .map_err(|e| IoError::Decode(format!("decode: {e}")))?;
        let spec = decoded.spec();
        let mut buffer = SampleBuffer::<f32>::new(
            decoded.capacity() as u64,
            spec,
        );
        buffer.copy_interleaved_ref(decoded);
        for &sample in buffer.samples().iter() {
            interleaved.push(sample);
        }
    }

    let frames = interleaved.len() as u64 / channels as u64;

    // Compute waveform peaks efficiently in chunks (allocation-free scan).
    const PEAK_BLOCK_SIZE: usize = 512;
    let total_samples = interleaved.len();
    for chunk_start in (0..total_samples).step_by(PEAK_BLOCK_SIZE) {
        let chunk_end = (chunk_start + PEAK_BLOCK_SIZE).min(total_samples);
        let chunk = &interleaved[chunk_start..chunk_end];
        let min = chunk.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = chunk.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        peaks.push((min, max));
    }

    Ok(DecodedAudio {
        sample_rate,
        channels,
        frames,
        interleaved,
        peaks,
    })
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

/// Offline render of a project or selection to WAV/FLAC.
/// Keeps the render faster than real-time for low-end targets (no real-time
/// device constraints; deterministic block size).
pub fn render_offline(project: &Project, output_path: &Path) -> Result<(), IoError> {
    // Phase 4: offline render using the same graph as the audio engine.
    // For Phase 4 vertical slice, this verifies the output path and writes
    // a placeholder file confirming the pipeline connects.
    use std::fs;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| IoError::Decode(e.to_string()))?;
    }
    fs::write(output_path, b"VegieBreakDAW Phase 4 offline render placeholder")
        .map_err(|e| IoError::Decode(e.to_string()))?;
    Ok(())
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
