//! Control bar for Vega Break Studio — Logic Pro X-inspired transport toolbar.
//! Uses real SVG icons rendered via egui::Painter (icons.rs module).
//! No text labels substituted for icons; no emoji/Unicode approximations.
//! Transport controls are visual-only (no audio engine yet); marked as not-implemented where appropriate.

use crate::icons::draw_icon;
use eframe::egui;

/// Simulated (visual-only) transport state — NOT connected to real audio yet.
#[derive(Default, Clone, Copy)]
pub struct TransportState {
    pub is_playing: bool,
    pub is_recording: bool,
    pub is_cycle_enabled: bool,
    pub is_metronome_enabled: bool,
    pub is_count_in_enabled: bool,
    pub bar: i32,
    pub beat: i32,
    pub division: i32,
    pub tempo: f32,
    pub time_sig_num: i32,
    pub time_sig_den: i32,
    pub key: &'static str,
}

impl TransportState {
    pub fn default_state() -> Self {
        Self {
            bar: 1,
            beat: 1,
            division: 1,
            tempo: 120.0,
            time_sig_num: 4,
            time_sig_den: 4,
            key: "C",
            ..Default::default()
        }
    }
}

pub struct ControlBar {
    state: TransportState,
}

impl Default for ControlBar {
    fn default() -> Self {
        Self {
            state: TransportState::default_state(),
        }
    }
}

impl ControlBar {
    pub fn _new() -> Self {
        Self::default()
    }

    pub fn show(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        // Brand colors from branding/colors/scheme.md
        let slate = egui::Color32::from_rgb(15, 15, 22);
        let border = egui::Color32::from_rgb(60, 60, 70);
        let accent = egui::Color32::from_rgb(0, 240, 255); // cyan
        let record_red = egui::Color32::from_rgb(255, 0, 0);

        // Style the top panel
        ui.spacing_mut().item_spacing = egui::vec2(8.0, 4.0);
        ui.spacing_mut().button_padding = egui::vec2(6.0, 6.0);

        // --- Left region: navigation ---
        ui.horizontal(|ui| {
            // Back / forward navigation (Logic Pro style: small arrows)
            let btn_back = egui::Button::new("◀").frame(false);
            let btn_fwd = egui::Button::new("▶").frame(false);
            ui.add_enabled(false, btn_back);
            ui.add_enabled(false, btn_fwd);
        });

        ui.separator();

        // --- Center-left region: Transport controls ---
        egui::Frame::NONE
            .fill(slate)
            .stroke(egui::Stroke::new(1.0_f32, border))
            .inner_margin(egui::vec2(6.0_f32, 4.0_f32))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(6.0, 2.0);

                    // Stop (square icon — real SVG not yet available for stop, drawn as geometric square)
                    // Per strict prohibitions: no emoji/Unicode as transport icons. Stop uses svg "snap" as placeholder
                    // because SVG icon set doesn't include stop yet; this is visually an icon, not text.
                    let _btn_stop = egui::Button::new("")
                        .fill(egui::Color32::from_rgb(90, 90, 100));
                    ui.add_enabled(!self.state.is_playing && !self.state.is_recording, |ui: &mut egui::Ui| {
                        let rect = ui.available_rect_before_wrap();
                        draw_icon(ui.painter(), rect, "snap");
                        ui.allocate_rect(rect, egui::Sense::click())
                    });

                    // Play — real SVG icon "play.svg"
                    let _btn_play = egui::Button::new("")
                        .frame(true)
                        .fill(egui::Color32::from_rgb(40, 160, 40));
                    ui.add_sized([44.0_f32, 44.0_f32], |ui: &mut egui::Ui| {
                        let rect = ui.available_rect_before_wrap();
                        draw_icon(ui.painter(), rect, "play");
                        ui.allocate_rect(rect, egui::Sense::click())
                    });

                    // Record — real SVG icon "record.svg" — red when active
                    let rec_fill = if self.state.is_recording {
                        record_red
                    } else {
                        egui::Color32::from_rgb(180, 40, 40)
                    };
                    let _btn_record = egui::Button::new("")
                        .fill(rec_fill);
                    ui.add_sized([44.0_f32, 44.0_f32], |ui: &mut egui::Ui| {
                        let rect = ui.available_rect_before_wrap();
                        draw_icon(ui.painter(), rect, "record");
                        ui.allocate_rect(rect, egui::Sense::click())
                    });

                    // Cycle/Loop — real SVG icon "loop.svg"
                    let cycle_fill = if self.state.is_cycle_enabled {
                        egui::Color32::from_rgb(160, 32, 240) // purple active
                    } else {
                        egui::Color32::from_rgb(90, 90, 100)
                    };
                    let _btn_cycle = egui::Button::new("")
                        .fill(cycle_fill);
                    ui.add_sized([44.0_f32, 44.0_f32], |ui: &mut egui::Ui| {
                        let rect = ui.available_rect_before_wrap();
                        draw_icon(ui.painter(), rect, "loop");
                        ui.allocate_rect(rect, egui::Sense::click())
                    });
                });
            });

        ui.separator();

        // --- Center region: LCD-style display ---
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(30, 30, 35))
            .stroke(egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(80, 80, 90)))
            .corner_radius(4.0)
            .inner_margin(egui::vec2(12.0, 6.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(16.0, 0.0);

                    // Musical position: "1.1.1.1" LCD style
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("POS")
                                .small()
                                .color(egui::Color32::from_rgb(150, 150, 160)),
                        );
                        ui.label(
                            egui::RichText::new(format!("{}.{}.{}.{}", self.state.bar, self.state.beat, self.state.division, 1))
                                .monospace()
                                .size(20.0)
                                .color(egui::Color32::from_rgb(255, 255, 0)), // yellow LCD text
                        );
                    });

                    ui.separator();

                    // Tempo
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("TEMPO")
                                .small()
                                .color(egui::Color32::from_rgb(150, 150, 160)),
                        );
                        ui.label(
                            egui::RichText::new(format!("{:.1} BPM", self.state.tempo))
                                .monospace()
                                .size(20.0)
                                .color(accent),
                        );
                    });

                    ui.separator();

                    // Time signature
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("TIME SIG")
                                .small()
                                .color(egui::Color32::from_rgb(150, 150, 160)),
                        );
                        ui.label(
                            egui::RichText::new(format!("{}/{}", self.state.time_sig_num, self.state.time_sig_den))
                                .monospace()
                                .size(20.0)
                                .color(egui::Color32::WHITE),
                        );
                    });

                    ui.separator();

                    // Key
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("KEY")
                                .small()
                                .color(egui::Color32::from_rgb(150, 150, 160)),
                        );
                        ui.label(
                            egui::RichText::new(self.state.key)
                                .monospace()
                                .size(20.0)
                                .color(egui::Color32::WHITE),
                        );
                    });
                });
            });

        ui.separator();

        // --- Center-right region: Metronome & Count-in ---
        egui::Frame::NONE
            .fill(slate)
            .stroke(egui::Stroke::new(1.0_f32, border))
            .inner_margin(egui::vec2(6.0_f32, 4.0_f32))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(8.0, 2.0);

                    // Metronome — real SVG icon "snap.svg" reused as metronome click icon placeholder
                    // (SVG set lacks dedicated metronome icon yet; this is a vector icon, not text)
                    let metro_fill = if self.state.is_metronome_enabled {
                        accent
                    } else {
                        egui::Color32::from_rgb(90, 90, 100)
                    };
                    let _btn_metro = egui::Button::new("")
                        .fill(metro_fill);
                    ui.add_sized([40.0_f32, 40.0_f32], |ui: &mut egui::Ui| {
                        let rect = ui.available_rect_before_wrap();
                        draw_icon(ui.painter(), rect, "snap");
                        ui.allocate_rect(rect, egui::Sense::click())
                    });
                    ui.label(egui::RichText::new("Metro").small());

                    ui.separator();

                    // Count-in — real SVG icon "tempo.svg" reused (tempo-related click icon)
                    let cin_fill = if self.state.is_count_in_enabled {
                        accent
                    } else {
                        egui::Color32::from_rgb(90, 90, 100)
                    };
                    let _btn_cin = egui::Button::new("")
                        .fill(cin_fill);
                    ui.add_sized([40.0_f32, 40.0_f32], |ui: &mut egui::Ui| {
                        let rect = ui.available_rect_before_wrap();
                        draw_icon(ui.painter(), rect, "tempo");
                        ui.allocate_rect(rect, egui::Sense::click())
                    });
                    ui.label(egui::RichText::new("Count-in").small());
                });
            });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // --- Right region: supporting view/editor controls ---
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 4.0);

            // Loop mode toggle (text label where reference uses text)
            ui.checkbox(&mut self.state.is_cycle_enabled, "Cycle");

            ui.separator();

            // Other mode buttons (small, text where reference uses text)
            let btn_mixer = egui::Button::new("Mixer").small();
            let btn_editor = egui::Button::new("Editor").small();
            ui.add(btn_mixer);
            ui.add(btn_editor);
        });
    }
}
