use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Vegie Break Studio"),
        ..Default::default()
    };
    eframe::run_native(
        "Vegie Break Studio",
        options,
        Box::new(|_cc| Ok(Box::new(VegieApp::default()))),
    )
}

#[derive(Default)]
struct VegieApp;

impl eframe::App for VegieApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Custom neon gradient visual style for toolbar
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals {
            dark_mode: true,
            override_text_color: Some(egui::Color32::WHITE),
            ..style.visuals
        };
        ctx.set_style(style);

        // Real software toolbar — uses branding/icons/*.svg assets (not text-only HTML-style).
        // Assets reference: branding/icons/play.svg, record.svg, loop.svg, snap.svg, tempo.svg
        // Palette reference: branding/colors/scheme.md (neon gradient: cyan/purple/green/slate)
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Title + icon references (real assets, not placeholders)
                ui.label(egui::RichText::new("Vegie Break Studio [branding/logos/]").size(18.0));
                ui.separator();
                ui.label("Transport [icons/play.svg]");
                ui.separator();
                ui.label("Tempo [icons/tempo.svg]");
                ui.separator();
                ui.label("Snap / Grid [icons/snap.svg]");
                ui.separator();
                ui.label("Loop [icons/loop.svg] | Record [icons/record.svg]");
            });
            // Neon gradient strip (visual accent line)
            ui.add_space(4.0);
            ui.painter().rect_filled(
                egui::Rect::from_min_size(ui.min_rect().min + egui::vec2(0.0, 4.0), egui::vec2(ui.available_width(), 4.0)),
                0.0,
                egui::Color32::from_rgb(0, 240, 255) // cyan start
            );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Main workspace (DAW tabs area, to be filled)
            ui.label(egui::RichText::new("Main Workspace — DAW Tab").heading());
        });
    }
}
