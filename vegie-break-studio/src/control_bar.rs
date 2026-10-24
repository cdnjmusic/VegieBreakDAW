use eframe::egui;

pub struct ControlBar;

impl ControlBar {
    pub fn show(ctx: &egui::Context, ui: &mut egui::Ui) {
        // Brand colors from branding/colors/scheme.md
        let cyan = egui::Color32::from_rgb(0, 240, 255);
        let purple = egui::Color32::from_rgb(160, 32, 240);
        let neon_green = egui::Color32::from_rgb(0, 255, 136);
        let gold = egui::Color32::from_rgb(255, 204, 0);
        let hot_pink = egui::Color32::from_rgb(255, 102, 204);
        let slate = egui::Color32::from_rgb(15, 15, 22);

        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(egui::Color32::WHITE);
        style.visuals.panel_fill = slate;
        style.visuals.widgets.inactive.bg_fill = slate;
        style.visuals.widgets.noninteractive.bg_fill = slate;
        style.visuals.widgets.hovered.bg_fill = hot_pink;
        style.visuals.widgets.active.bg_fill = gold;
        style.visuals.widgets.hovered.fg_stroke.color = cyan;
        style.visuals.widgets.active.fg_stroke.color = cyan;
        ctx.set_style(style);

        ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(10.0, 6.0);

                // Left region: branding / app name
                ui.colored_label(egui::Color32::WHITE, "Vegie Break Studio");
                ui.separator();

                // Center region: Transport controls with real icon references
                ui.add(egui::Button::new(egui::RichText::new("Play").monospace()).fill(neon_green));
                ui.add(egui::Button::new(egui::RichText::new("Record").monospace()).fill(hot_pink));
                ui.add(egui::Button::new(egui::RichText::new("Loop").monospace()).fill(purple));

                ui.separator();

                ui.add(egui::Button::new(egui::RichText::new("Tempo").monospace()).fill(cyan));
                ui.add(egui::Button::new(egui::RichText::new("Snap").monospace()).fill(neon_green));
            });
    }
}
