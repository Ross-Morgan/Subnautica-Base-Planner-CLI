use eframe::egui::{self, load::SizedTexture, ColorImage, Image, TextureOptions, ViewportBuilder};
use egui_extras::install_image_loaders;

use rand::Rng;
use strum::{EnumCount, IntoEnumIterator};

use crate::subnautica::biomes::Biome;

use super::splash_screen::SplashScreen;

pub fn run_app() -> eframe::Result<(), eframe::Error> {
    let splash_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([480.0, 270.0])
            .with_icon(super::icon::load_icon())
            .with_fullscreen(false)
            .with_resizable(false)
            .with_taskbar(false)
            .with_drag_and_drop(false)
            .with_close_button(false)
            .with_minimize_button(false)
            .with_maximize_button(false)
            .with_window_level(egui::WindowLevel::AlwaysOnTop),
        centered: true,
        ..Default::default()
    };

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_icon(super::icon::load_icon())
            .with_fullscreen(true)
            .with_resizable(false),
        ..Default::default()
    };

    let n = rand::thread_rng().gen_range(0..Biome::COUNT);
    let biome = Biome::iter().skip(n-1).next().unwrap();

    eframe::run_native(crate::APP_NAME, splash_options, Box::new(|_| Ok(Box::new(SplashScreen::new(biome)))))?;

    eframe::run_native(crate::APP_NAME, options, Box::new(|cc| {
        let mut app = super::app::App::default();

        install_image_loaders(&cc.egui_ctx);

        for biome in Biome::iter() {
            let rgb = image::open(biome.associated_path()).expect("Couldn't load image (App:run)").to_rgb8();
            let color_image = ColorImage::from_rgb([1920, 1080], &rgb);
            let handle = cc.egui_ctx.load_texture(biome.to_string(), color_image, TextureOptions::LINEAR);
            let texture = SizedTexture::new(handle.id(), egui::vec2(1920.0, 1080.0));
            let image = Image::from_texture(texture);

            app.image_handles.insert(biome, handle);
            app.images.insert(biome, image);
        }

        Ok(Box::new(app))
    }))?;

    Ok(())
}