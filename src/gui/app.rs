use std::collections::HashMap;

use eframe::egui::{self, include_image, Context, Frame, Image, Layout, Margin, TextureHandle};

use strum::IntoEnumIterator;

use crate::subnautica::biomes::Biome;

const MARGIN: Margin = Margin {
    left: 0.0,
    right: 0.0,
    top: 8.0,
    bottom: 10.0,
};

#[derive(Default)]
pub struct App {
    pub image_handles: HashMap<Biome, TextureHandle>,
    pub images: HashMap<Biome, Image<'static>>,
    pub current_biome: Biome,
    pub is_fullscreen: bool,
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

                if ui
                    .add(egui::ImageButton::new(include_image!(
                        "../../assets/yellow-circle.png"
                    )))
                    .clicked()
                {
                    self.is_fullscreen = !self.is_fullscreen;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.is_fullscreen));
                };

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

        egui::TopBottomPanel::bottom("bottom_bar")
            .exact_height(50.0)
            .show(ctx, |ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), 50.0),
                    Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        Frame::none()
                            .inner_margin(MARGIN)
                            .show(ui, |ui| {
                                egui::ComboBox::from_label("Select Biome")
                                    .selected_text(self.current_biome.to_label_string().to_string())
                                    .height(800.0)
                                    .show_ui(ui, |ui| {
                                        for biome in Biome::iter() {
                                            ui.selectable_value(
                                                &mut self.current_biome,
                                                biome,
                                                biome.to_label_string(),
                                            );

                                            if biome == Biome::NoBiome {
                                                ui.separator();
                                            }
                                        }
                                    });
                            })
                    },
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_background(ui);
        });
    }
}
