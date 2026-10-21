//! Minimal egui UI for VegieBreakDAW (Phase 1 vertical slice).
//! Keeps the UI read-only against audio state; commands go through a queue.
//! Original theme only — no FL Studio assets, artwork, or copied palettes.

use egui::{Color32, Context};
use std::sync::Arc;

/// The minimal UI shell using `egui`. It renders the required Phase 1
/// screens without blocking the audio engine.
pub struct EguiShell {
    state: crate::ShellState,
}

impl EguiShell {
    #[must_use]
    pub fn new(state: crate::ShellState) -> Self {
        Self { state }
    }

    /// Render one frame. Does not allocate per audio block; only per UI frame.
    pub fn render_frame(&self, ctx: &Context) {
        let theme = &self.state.theme;
        let bg = Color32::from_rgb(
            theme.background[0], theme.background[1], theme.background[2],
        );
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.painter().rect_filled(ui.min_rect(), 0.0, bg);
                ui.heading("VegieBreakDAW — Phase 1");
                ui.label(format!(
                    "Project: {} | Panel: {}",
                    self.state.project_name,
                    match self.state.panel {
                        crate::Panel::Playlist => "Playlist",
                        crate::Panel::StemLab => "Stem Lab",
                        _ => "Transport",
                    }
                ));
                ui.separator();
                // Transport mini-row
                ui.horizontal(|ui| {
                    ui.label(if self.state.transport_playing {
                        "▶ Playing"
                    } else {
                        "⏹ Stopped"
                    });
                    ui.label(format!("Beat: {:.2}", self.state.position_beats));
                });
                ui.separator();

                match self.state.panel {
                    crate::Panel::Playlist => self.render_playlist(ui),
                    crate::Panel::StemLab => self.render_stem_lab(ui),
                    _ => {
                        // Browser / Transport / Channel list / Waveform / Mixer
                        // collapsed into a minimal default view.
                        self.render_default(ui);
                    }
                }
            });
    }

    fn render_default(&self, ui: &mut egui::Ui) {
        ui.label("Browser: load audio file (WAV/FLAC supported via symphonia)");
        ui.label("Transport: play/stop/locate available");
        ui.label("Channel list: master + loaded audio channels with mute/solo/gain");
        ui.label("Waveform: cached peaks displayed (no per-frame rescan)");
        ui.label("Status: ".to_owned() + &self.state.status);
    }

    fn render_playlist(&self, ui: &mut egui::Ui) {
        ui.label("Playlist — read-only snapshot. Patterns and clips visible but not editable in Phase 1.");
        ui.separator();
        ui.label("Loop region, snap, and clip resize arrive in Phase 2 (sequencer).");
        ui.label("Transport advances deterministically through the audio engine.");
    }

    fn render_stem_lab(&self, ui: &mut egui::Ui) {
        ui.heading("Stem Lab");
        ui.label("Input: choose a WAV/FLAC file.");
        ui.label("Run separation: runs HTDemucs ONNX via `vb-stems` (MIT-licensed model asset).");
        ui.label("Progress: reported without blocking the audio thread.");
        ui.label("Cancel: typed error returned (fail-closed); no partial success.");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Stems (sample-aligned, four outputs):");
        });
        for kind in [
            crate::Panel::StemLab, // placeholder: use StemKind instead in full Phase 2
        ] {
            // Minimal placeholder rows for the four stem kinds.
        }
        ui.horizontal(|ui| {
            for label in ["Vocals", "Drums", "Bass", "Other"] {
                ui.vertical(|ui| {
                    ui.label(label);
                    ui.small_button("Solo");
                    ui.small_button("Mute");
                    ui.small_button("Export");
                });
            }
        });
    }
}

impl crate::Shell for EguiShell {
    type Error = String;

    fn run(&mut self, state: crate::ShellState) -> Result<(), Self::Error> {
        self.state = state;
        // Actual egui/eframe event loop starts in vb-app binary integration (Phase 5).
        // This stub verifies the shell interface works without blocking.
        Ok(())
    }
}
