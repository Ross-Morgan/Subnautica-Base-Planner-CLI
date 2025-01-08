use eframe::egui;

use crate::subnautica::biomes::Biome;

#[derive(Default)]
pub struct SplashScreen {
    pub biome: Biome,
    pub completed: u8,
}

impl eframe::App for SplashScreen {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::ProgressBar::new(self.completed as f32 / 11.0).animate(true));
        });
    }
}

impl SplashScreen {
    pub fn new(biome: Biome) -> Self {
        Self { biome, completed: 0 }
    }
}