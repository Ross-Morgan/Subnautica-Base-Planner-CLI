use std::collections::HashMap;

use eframe::egui::{self, include_image, Context, Image, Layout, TextureHandle};

use strum::IntoEnumIterator;

use crate::subnautica::biomes::Biome;

#[derive(Default)]
pub struct App {
    pub image_handles: HashMap<Biome, TextureHandle>,
    pub images: HashMap<Biome, Image<'static>>,
    pub current_biome: Biome,
}

impl App {
    pub fn update_background(&mut self, ui: &mut egui::Ui) {
        let image = &self.images[&self.current_biome];

        image.paint_at(ui, ui.ctx().screen_rect());
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                if ui
                    .add(egui::ImageButton::new(include_image!(
                        "../../assets/red-circle.png"
                    )))
                    .clicked()
                {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                ui.add(egui::ImageButton::new(include_image!(
                    "../../assets/yellow-circle.png"
                )));

                if ui
                    .add(egui::ImageButton::new(include_image!(
                        "../../assets/green-circle.png"
                    )))
                    .clicked()
                {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_background(ui);

            egui::ComboBox::from_label("Select Biome")
                .selected_text(format!("{}", self.current_biome.to_label_string()))
                .show_ui(ui, |ui| {
                    for biome in Biome::iter() {
                        ui.selectable_value(
                            &mut self.current_biome,
                            biome,
                            biome.to_label_string(),
                        );
                    }
                });
        });
    }
}
