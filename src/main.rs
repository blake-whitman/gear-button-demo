use eframe::{egui, epi};
use egui::{FontId, Vec2};

const GEAR: &str = "⚙";

struct Footer;

impl Footer {
    fn animated_gear_button(&self, ui: &mut egui::Ui, ctx: &egui::Context) -> egui::Response {
        let size_factor = ctx.animate_value_with_time(
            ui.next_auto_id().with("gear_size"),
            if ui.rect_contains_pointer(ui.min_rect()) { 1.2 } else { 1.0 },
            0.2,
        );

        let gear_size = 24.0 * size_factor;

        let response = ui.allocate_response(Vec2::splat(gear_size), egui::Sense::click());

        if response.hovered() {
            ctx.request_repaint();
            let rotation = (ctx.input(|i| i.time) * 2.0) % std::f64::consts::TAU;

            ui.painter().with_rotation(
                rotation as f32,
                response.rect.center(),
                |painter| {
                    painter.text(
                        response.rect.center(),
                        egui::Align2::CENTER_CENTER,
                        GEAR,
                        FontId::proportional(gear_size),
                        ui.visuals().text_color(),
                    );
                },
            );
        } else {
            ui.painter().text(
                response.rect.center(),
                egui::Align2::CENTER_CENTER,
                GEAR,
                FontId::proportional(gear_size),
                ui.visuals().text_color(),
            );
        }

        response
            .on_hover_text("Settings")
            .on_hover_cursor(egui::CursorIcon::PointingHand)
    }
}

struct DemoApp;

impl epi::App for DemoApp {
    fn name(&self) -> &str {
        "Animated Gear Button"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            let footer = Footer;
            footer.animated_gear_button(ui, ctx);
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Animated Gear Demo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(DemoApp)),
    )
}
