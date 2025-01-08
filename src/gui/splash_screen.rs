use std::{collections::HashMap, thread::JoinHandle};

use eframe::{
    egui::{self, load::SizedTexture, ColorImage, Image, TextureHandle},
    CreationContext,
};
use strum::IntoEnumIterator;

use crate::subnautica::biomes::Biome;

const TO_COMPLETE: u8 = 22;

pub struct SplashScreen<'a> {
    completed: u8,
    splash_image: (TextureHandle, Image<'static>),
    handles: HashMap<Biome, Option<JoinHandle<ColorImage>>>,
    images: &'a mut HashMap<Biome, ColorImage>,
}

impl<'a> eframe::App for SplashScreen<'a> {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.splash_image.1.paint_at(ui, ui.ctx().screen_rect());
            self.completed = self.handles.iter().fold(0, |acc, (_, b)| {
                acc + b.as_ref().map(|j| j.is_finished() as u8).unwrap_or(0)
            });

            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::BottomUp),
                |ui| {
                    ui.label(format!("{} / {}", self.completed, TO_COMPLETE));
                },
            );

            if self.completed == TO_COMPLETE {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        for (&biome, handle) in self.handles.iter_mut() {
            self.images
                .insert(biome, handle.take().unwrap().join().expect("Thread failed"));
        }
    }
}

impl<'a> SplashScreen<'a> {
    pub fn new(
        biome: Biome,
        cc: &CreationContext,
        images: &'a mut HashMap<Biome, ColorImage>,
    ) -> Self {
        let rgb = image::open(biome.associated_path())
            .expect("Couldn't load image (App:run)")
            .to_rgb8();
        let color_image = ColorImage::from_rgb([1920, 1080], &rgb);
        let handle =
            cc.egui_ctx
                .load_texture(biome.to_string(), color_image, egui::TextureOptions::LINEAR);
        let texture = SizedTexture::new(handle.id(), egui::vec2(1920.0, 1080.0));
        let image = Image::from_texture(texture);
        let splash_image = (handle, image);

        let mut app = Self {
            completed: 0,
            splash_image,
            handles: HashMap::new(),
            images,
        };

        for biome in Biome::iter() {
            let handle = std::thread::spawn(move || {
                let rgb = image::open(biome.associated_path())
                    .expect("Couldn't load image (App:run)")
                    .to_rgb8();
                let color_image = ColorImage::from_rgb([1920, 1080], &rgb);
                color_image
            });

            app.handles.insert(biome, Some(handle));
        }

        app
    }
}
