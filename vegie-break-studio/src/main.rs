use eframe::egui;
mod control_bar;

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

        egui::TopBottomPanel::top("toolbar").show_separator_line(false).show(ctx, |ui| {
            control_bar::ControlBar::show(ctx, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Main Workspace — DAW Tab").size(28.0));
            });
        });
    }
}
