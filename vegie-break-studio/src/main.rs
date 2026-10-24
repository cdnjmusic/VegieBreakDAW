use eframe::egui;
mod control_bar;
mod icons;

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
struct App {
    control_bar: control_bar::ControlBar,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Brand colors from branding/colors/scheme.md
        let dark_slate = egui::Color32::from_rgb(15, 15, 22);

        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::WHITE);
        visuals.widgets.inactive.bg_fill = dark_slate;
        visuals.widgets.noninteractive.bg_fill = dark_slate;
        visuals.panel_fill = dark_slate;
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(255, 102, 204); // hot pink
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(255, 204, 0); // gold
        visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(0, 240, 255); // cyan
        visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgb(0, 240, 255);
        ctx.set_visuals(visuals);

        egui::TopBottomPanel::top("toolbar").show_separator_line(false).show(ctx, |ui| {
            self.control_bar.show(ctx, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Main Workspace — DAW Tab").size(28.0));
            });
        });
    }
}
