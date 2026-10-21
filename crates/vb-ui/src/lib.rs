#![forbid(unsafe_code)]
#![doc = "UI shell, theme, and panel primitives."]

use serde::{Deserialize, Serialize};
use vb_core::Project;

/// The top-level panels in the workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Panel {
    Browser,
    ChannelRack,
    StepSequencer,
    PianoRoll,
    Playlist,
    Mixer,
    PluginEditor,
    StemLab,
}

impl Panel {
    pub const ALL: [Self; 8] = [
        Self::Browser,
        Self::ChannelRack,
        Self::StepSequencer,
        Self::PianoRoll,
        Self::Playlist,
        Self::Mixer,
        Self::PluginEditor,
        Self::StemLab,
    ];

    #[must_use]
    pub fn title(self) -> &'static str {
        match self {
            Self::Browser => "Browser",
            Self::ChannelRack => "Channel Rack",
            Self::StepSequencer => "Step Sequencer",
            Self::PianoRoll => "Piano Roll",
            Self::Playlist => "Playlist",
            Self::Mixer => "Mixer",
            Self::PluginEditor => "Plugin Editor",
            Self::StemLab => "Stem Lab",
        }
    }
}

/// A serializable color theme. The first theme is original; no FL Studio
/// artwork or palette is copied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: [u8; 3],
    pub panel: [u8; 3],
    pub accent: [u8; 3],
    pub text: [u8; 3],
    pub muted_text: [u8; 3],
    pub waveform: [u8; 3],
    pub playhead: [u8; 3],
}

impl Default for Theme {
    fn default() -> Self {
        // Original VegieBreak Studio dark palette (from images/VegieBreak_Studio_Color_Scheme.md).
        // Takes FL Studio dark workflow inspiration independently.
        Self {
            name: "Vegie Break Studio".to_owned(),
            background: [16, 16, 24],    // #101018 Midnight Charcoal
            panel: [28, 28, 41],        // #1C1C29
            accent: [200, 255, 56],     // #C8FF38 Electric lime
            text: [244, 240, 232],      // #F4F0E8 Warm off-white
            muted_text: [169, 162, 183], // #A9A2B7
            waveform: [53, 223, 255],   // #35DFFF Electric cyan
            playhead: [255, 75, 145],   // #FF4B91 Hot pink
        }
    }
}

/// A read-only snapshot the UI can render without touching audio state.
#[derive(Debug, Clone)]
pub struct ShellState {
    pub project_name: String,
    pub panel: Panel,
    pub theme: Theme,
    pub transport_playing: bool,
    pub position_beats: f64,
    pub status: String,
}

impl ShellState {
    #[must_use]
    pub fn from_project(project: &Project) -> Self {
        Self {
            project_name: project.name.clone(),
            panel: Panel::Playlist,
            theme: Theme::default(),
            transport_playing: false,
            position_beats: 0.0,
            status: "Ready".to_owned(),
        }
    }
}

/// The UI boundary. Implementations own the window and GPU surface.
pub trait Shell {
    type Error;

    fn run(&mut self, state: ShellState) -> Result<(), Self::Error>;
}

/// A headless shell for tests and CLI diagnostics.
#[derive(Debug, Default)]
pub struct NullShell;

impl Shell for NullShell {
    type Error = std::convert::Infallible;

    fn run(&mut self, _state: ShellState) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub mod egui_shell;
pub use egui_shell::EguiShell;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_titles_are_unique() {
        let mut titles = Panel::ALL.map(Panel::title).to_vec();
        titles.sort_unstable();
        titles.dedup();
        assert_eq!(titles.len(), Panel::ALL.len());
    }

    #[test]
    fn theme_is_opaque() {
        let theme = Theme::default();
        assert_eq!(theme.background.len(), 3);
        assert_ne!(theme.background, theme.accent);
    }

    #[test]
    fn shell_state_uses_project_name() {
        let project = Project::new("My Song");
        let state = ShellState::from_project(&project);
        assert_eq!(state.project_name, "My Song");
        assert_eq!(state.panel, Panel::Playlist);
    }
}
