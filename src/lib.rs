use eframe::{egui, App, CreationContext};
use egui::{FontId, Vec2};
use wasm_bindgen::prelude::*;

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

        ctx.request_repaint();

        ui.painter().text(
            response.rect.center(),
            egui::Align2::CENTER_CENTER,
            GEAR,
            FontId::proportional(gear_size),
            ui.visuals().text_color(),
        );

        response
            .on_hover_text("Settings")
            .on_hover_cursor(egui::CursorIcon::PointingHand)
    }
}

struct DemoApp;

impl App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            let footer = Footer;
            footer.animated_gear_button(ui, ctx);
        });
    }
}

#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), JsValue> {
    let native_options = eframe::WebOptions::default();
    eframe::start_web(canvas_id, Box::new(|_cc: &CreationContext| Box::new(DemoApp)), native_options)
}
