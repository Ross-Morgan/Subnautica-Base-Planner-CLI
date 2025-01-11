use std::{collections::HashMap, thread::JoinHandle};

use eframe::egui::{self, load::SizedTexture, ColorImage, Image, TextureHandle};
use strum::IntoEnumIterator;

use crate::subnautica::biomes::Biome;

const TO_COMPLETE: u8 = 22;

pub struct SplashScreen<'a> {
    completed: u8,
    splash_biome: Biome,
    splash_image: Option<(TextureHandle, Image<'static>)>,
    handles: HashMap<Biome, Option<JoinHandle<ColorImage>>>,
    images: &'a mut HashMap<Biome, ColorImage>,
}

impl<'a> eframe::App for SplashScreen<'a> {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.splash_image.get_or_insert_with(|| {
            let rgb = image::open(self.splash_biome.associated_path())
                .expect("Couldn't load image (App:run)")
                .to_rgb8();

            let color_image = ColorImage::from_rgb([1920, 1080], &rgb);
            let handle = ctx.load_texture(
                self.splash_biome.to_string(),
                color_image,
                egui::TextureOptions::LINEAR,
            );
            let texture = SizedTexture::new(handle.id(), egui::vec2(1920.0, 1080.0));
            let image = Image::from_texture(texture);
            let splash_image = (handle, image);

            for biome in Biome::iter() {
                let handle = std::thread::spawn(move || {
                    let rgb = image::open(biome.associated_path())
                        .expect("Couldn't load image (App:run)")
                        .to_rgb8();

                    ColorImage::from_rgb([1920, 1080], &rgb)
                });

                self.handles.insert(biome, Some(handle));
            }

            splash_image
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.splash_image
                .as_ref()
                .unwrap()
                .1
                .paint_at(ui, ui.ctx().screen_rect());

            let completed = self.handles.iter().fold(0, |acc, (_, b)| {
                acc + b.as_ref().map_or(0, |j| u8::from(j.is_finished()))
            });

            if completed > self.completed {
                self.completed += 1;
            }

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
        for (&biome, handle) in &mut self.handles {
            self.images
                .insert(biome, handle.take().unwrap().join().expect("Thread failed"));
        }
    }
}

impl<'a> SplashScreen<'a> {
    pub fn new(splash_biome: Biome, images: &'a mut HashMap<Biome, ColorImage>) -> Self {
        Self {
            completed: 0,
            splash_image: None,
            splash_biome,
            handles: HashMap::new(),
            images,
        }
    }
}
