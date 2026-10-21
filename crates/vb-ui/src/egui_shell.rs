//! FL Studio-inspired full layout replication for VegieBreak Studio.
//! Uses original VB branding (`VB-front.png` colors); cleanroom removed per instruction.
//! Layout derived from FL Studio binary observations (`C:\Program Files\Image-Line`).

use egui::{Color32, Context};

pub struct EguiShell {
    state: crate::ShellState,
    selected_browser_category: String,
}

impl EguiShell {
    pub fn new(state: crate::ShellState) -> Self {
        Self {
            state,
            selected_browser_category: "Current project".to_string(),
        }
    }

    pub fn render_frame(&self, ctx: &Context) {
        let theme = &self.state.theme;
        let bg = Color32::from_rgb(
            theme.background[0], theme.background[1], theme.background[2],
        );
        let panel_bg = Color32::from_rgb(
            theme.panel[0], theme.panel[1], theme.panel[2],
        );
        let accent = Color32::from_rgb(
            theme.accent[0], theme.accent[1], theme.accent[2],
        );
        let muted = Color32::from_rgb(
            theme.muted_text[0], theme.muted_text[1], theme.muted_text[2],
        );
        let text = Color32::from_rgb(
            theme.text[0], theme.text[1], theme.text[2],
        );
        let playhead = Color32::from_rgb(
            theme.playhead[0], theme.playhead[1], theme.playhead[2],
        );

        // Full FL Studio-style menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("FILE", |ui| {
                    ui.label("New  (Ctrl+N)");
                    ui.label("Open  (Ctrl+O)");
                    ui.label("Save  (Ctrl+S)");
                    ui.separator();
                    ui.label("Export WAV / FLAC");
                    ui.separator();
                    ui.label("Exit");
                });
                ui.menu_button("EDIT", |ui| {
                    ui.label("Undo / Redo");
                    ui.label("Copy / Paste");
                });
                ui.menu_button("ADD", |ui| {
                    ui.label("Pattern");
                    ui.label("Audio Clip");
                    ui.label("Automation");
                });
                ui.menu_button("PATTERNS", |ui| {
                    ui.label("Pattern 1");
                    ui.label("Pattern 2");
                    ui.label("Pattern 3");
                    ui.label("Pattern 4");
                });
                ui.menu_button("VIEW", |ui| {
                    ui.label("Playlist / Piano Roll / Channel Rack");
                });
                ui.menu_button("OPTIONS", |ui| {
                    ui.label("Audio Settings");
                    ui.label("MIDI Settings");
                });
                ui.menu_button("TOOLS", |ui| {
                    ui.label("Macro");
                    ui.label("Project Info");
                });
                ui.menu_button("HELP", |ui| {
                    ui.label("About VegieBreak Studio");
                });
            });
        });

        // Transport bar (FL Studio full transport)
        egui::TopBottomPanel::top("transport").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(4.0);
                ui.colored_label(accent, "VegieBreak Studio  0.1.0");
                ui.separator();
                ui.add_space(8.0);
                ui.small_button(if self.state.transport_playing { "⏹ Stop" } else { "▶ Play" });
                ui.small_button("⏺ Record");
                ui.small_button("◀ Loop");
                ui.colored_label(muted, format!("Tempo: {:.0} BPM", 120.0));
                ui.colored_label(playhead, format!("Position: {:.2}  Beat", self.state.position_beats));
                ui.colored_label(
                    if self.state.transport_playing { accent } else { muted },
                    format!("{} | {} ch | {} Hz",
                        if self.state.transport_playing { "▶ PLAYING" } else { "⏹ STOPPED" },
                        2,
                        48000,
                    ),
                );
            });
        });

        // Main 3-column FL Studio-style workspace: Browser (left), Channel Rack (mid-left), Playlist (center), Browser categories expanded (right sidebar style in FL is left)
        // Using 3 split: Browser (left), Channel Rack (left-center), Playlist (center-right), Mixer/Stem (bottom full)
        egui::SidePanel::left("browser_sidebar").resizable(true).show(ctx, |ui| {
            ui.set_min_width(240.0);
            ui.heading("Browser");
            ui.separator();
            ui.collapsing("Current project", |ui| {
                ui.small_button("Project .vbproj");
            });
            ui.collapsing("Recent files", |ui| {
                ui.small_button("demo.wav");
                ui.small_button("loop.flac");
            });
            ui.collapsing("Plugin database", |ui| {
                ui.small_button("Generator Plugins");
                ui.small_button("Effect Plugins");
            });
            ui.collapsing("Plugin presets", |ui| {
                ui.small_button("Bass / Synth / Drums");
            });
            ui.collapsing("Channel presets", |ui| {
                ui.small_button("Master / Send");
            });
            ui.collapsing("Mixer presets", |ui| {
                ui.small_button("Default / DJ");
            });
            ui.collapsing("Scores", |ui| {
                ui.small_button("Default Score");
            });
            ui.collapsing("Audio", |ui| {
                ui.small_button("Recorded / Imported");
            });
            ui.collapsing("Backup", |ui| {
                ui.small_button("Auto-save files");
            });
            ui.collapsing("Channel envelopes", |ui| {
                ui.small_button("Volume / Pan");
            });
            ui.collapsing("Clipboard files", |ui| {
                ui.small_button("Copied clips");
            });
            ui.collapsing("Demo projects", |ui| {
                ui.small_button("Factory demo");
            });
            ui.collapsing("Envelopes", |ui| {
                ui.small_button("Templates");
            });
            ui.collapsing("Misc", |ui| {
                ui.small_button("Scratch files");
            });
            ui.collapsing("My projects", |ui| {
                ui.small_button("VB Studio Projects");
            });
            ui.collapsing("Packs", |ui| {
                ui.collapsing("Drums", |ui| { ui.small_button("Drum Samples"); });
                ui.collapsing("Instruments", |ui| { ui.small_button("Synth Presets"); });
                ui.collapsing("Bass", |ui| { ui.small_button("Bass Patches"); });
                ui.collapsing("Guitar", |ui| { ui.small_button("Guitar Riffs"); });
            });
            ui.collapsing("Impulses", |ui| {
                ui.small_button("Convolution IRs");
            });
        });

        // Channel rack (left-center in FL Studio layout, between browser and playlist)
        egui::SidePanel::left("channel_rack").resizable(true).show(ctx, |ui| {
            ui.set_min_width(200.0);
            ui.heading("Channel Rack");
            ui.separator();
            for i in 1..=19 {
                ui.horizontal(|ui| {
                    ui.colored_label(
                        if i == 1 { accent } else { muted },
                        format!("{}", i),
                    );
                    ui.colored_label(muted, format!("Track {}", i));
                    ui.add_space(4.0);
                    ui.small_button("●");
                    ui.small_button("●");
                    ui.small_button("●");
                });
            }
        });

        // Center playlist (main working area, like FL Studio)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter().rect_filled(ui.min_rect(), 0.0, bg);
            ui.heading("Playlist — Arrangement / Pattern 1");
            ui.separator();
            ui.horizontal(|ui| {
                ui.colored_label(text, format!("Project: {}  |  Panel: {}", self.state.project_name, match self.state.panel {
                    crate::Panel::Playlist => "Playlist",
                    crate::Panel::StemLab => "Stem Lab",
                    _ => "Transport",
                }));
                ui.colored_label(
                    if self.state.transport_playing { playhead } else { muted },
                    format!("▶ Play | Beat {:.2} | Tempo 120 BPM", self.state.position_beats),
                );
            });

            ui.separator();
            // Timeline ruler (independent original representation, FL Studio-style grid)
            ui.horizontal(|ui| {
                for tick in 1..=13 {
                    ui.colored_label(muted, format!("{}", tick));
                    ui.add_space(36.0);
                }
            });
            ui.separator();

            // Pattern clips as dark boxes (FL Studio-style pattern blocks in timeline)
            for row in 1..=4 {
                ui.horizontal(|ui| {
                    ui.colored_label(muted, format!("Pattern {}", row));
                    ui.add_space(4.0);
                    ui.painter().rect_filled(
                        egui::Rect::from_min_size(ui.cursor().min, egui::vec2(60.0, 28.0)),
                        2.0,
                        panel_bg,
                    );
                    ui.colored_label(text, "▮ 16 beats  ▮  Mute  ▮  Solo");
                    ui.colored_label(muted, "Loop: off  |  Snap: 1/4");
                });
            }

            ui.separator();
            ui.label("Note editor / piano roll (Phase 2): note slide, velocity, scale helpers planned.");
            ui.label("Loop region, snap, clip resize: arrives in Phase 2.");
            ui.label("Transport advances deterministically through the audio engine.");
        });

        // Bottom bar: mixer / stem export / transport controls (like FL Studio)
        egui::TopBottomPanel::bottom("bottom_mixer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Mixer / Stem Export / Transport");
                ui.colored_label(accent, "Master  Gain: 0 dB  Pan: 0.0");
                ui.small_button("Mute");
                ui.small_button("Solo");
                ui.separator();
                ui.colored_label(playhead, "Stem Export");
                ui.small_button("WAV Float");
                ui.small_button("FLAC");
                ui.colored_label(muted, "|  Transport: ");
                ui.small_button("Play");
                ui.small_button("Stop");
                ui.small_button("Record");
                ui.colored_label(muted, format!("Beat: {:.2}  |  Tempo: 120 BPM", self.state.position_beats));
            });
        });
    }
}

impl crate::Shell for EguiShell {
    type Error = String;
    fn run(&mut self, state: crate::ShellState) -> Result<(), Self::Error> {
        self.state = state;
        Ok(())
    }
}
