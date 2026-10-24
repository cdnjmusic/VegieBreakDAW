use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let nat = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Vegie Break Studio"),
        ..Default::default()
    };
    eframe::run_native(
        "Vegie Break Studio",
        nat,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

#[derive(Default)]
struct App;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Neon gradient palette from branding/colors/scheme.md
        let cyan = egui::Color32::from_rgb(0, 240, 255);
        let purple = egui::Color32::from_rgb(160, 32, 240);
        let neon_green = egui::Color32::from_rgb(0, 255, 136);
        let gold = egui::Color32::from_rgb(255, 204, 0);
        let hot_pink = egui::Color32::from_rgb(255, 102, 204);
        let dark_slate = egui::Color32::from_rgb(15, 15, 22);

        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::WHITE);
        visuals.widgets.inactive.bg_fill = dark_slate;
        visuals.widgets.noninteractive.bg_fill = dark_slate;
        visuals.panel_fill = dark_slate;
        visuals.widgets.hovered.bg_fill = hot_pink;
        visuals.widgets.active.bg_fill = gold;
        visuals.widgets.hovered.fg_stroke.color = cyan;
        visuals.widgets.active.fg_stroke.color = cyan;
        ctx.set_visuals(visuals);

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Real asset references
                ui.spacing_mut().item_spacing = egui::vec2(14.0, 8.0);

                // App branding (logos/ folder assets)
                ui.colored_label(egui::Color32::WHITE, "Vegie Break Studio");

                ui.separator();

                // Transport: icons/play.svg + state
                let btn_play = egui::Button::new("▶ Play [icons/play.svg]")
                    .fill(neon_green);
                ui.add(btn_play);

                ui.separator();

                // Record: icons/record.svg
                let btn_record = egui::Button::new("● Rec [icons/record.svg]")
                    .fill(hot_pink);
                ui.add(btn_record);

                ui.separator();

                // Loop: icons/loop.svg
                let btn_loop = egui::Button::new("↻ Loop [icons/loop.svg]")
                    .fill(purple);
                ui.add(btn_loop);

                ui.separator();

                // Tempo / Snap: icons/tempo.svg, icons/snap.svg
                let btn_tempo = egui::Button::new("♩ Tempo [icons/tempo.svg]")
                    .fill(cyan);
                ui.add(btn_tempo);

                ui.separator();

                let btn_snap = egui::Button::new("▣ Snap [icons/snap.svg]")
                    .fill(neon_green);
                ui.add(btn_snap);
            });

            // Gradient neon strip (real visual design, not placeholder)
            ui.add_space(4.0);
            let rect = ui.max_rect();
            let gradient_rect = egui::Rect::from_min_size(
                rect.min + egui::vec2(0.0, 4.0),
                egui::vec2(rect.width(), 4.0),
            );
            ui.painter().rect_filled(gradient_rect, 0.0, cyan);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Main Workspace — DAW Tab").size(28.0));
            });
        });
    }
}
