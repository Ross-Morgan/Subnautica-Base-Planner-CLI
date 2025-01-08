use std::collections::HashMap;

use eframe::egui::{self, load::SizedTexture, Image, TextureOptions, ViewportBuilder};
use egui_extras::install_image_loaders;
use rand::Rng;
use strum::{EnumCount, IntoEnumIterator};

use crate::subnautica::biomes::Biome;

use super::splash_screen::SplashScreen;

/// # Errors
///
/// Fails if window cannot be created
pub fn app() -> eframe::Result<(), eframe::Error> {
    let splash_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([960.0, 540.0]) //TODO: Size based off monitor resolution, i.e half height, half width of monitor /
            .with_icon(super::icon::load_icon())
            .with_fullscreen(true)
            .with_maximized(false)
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
    let splash_biome = Biome::iter().nth(n - 1).unwrap_or_default();

    let mut images = HashMap::new();

    eframe::run_native(
        crate::APP_NAME,
        splash_options,
        Box::new(|_| Ok(Box::new(SplashScreen::new(splash_biome, &mut images)))),
    )?;

    eframe::run_native(
        crate::APP_NAME,
        options,
        Box::new(|cc| {
            let mut app = super::app::App {
                image_handles: HashMap::new(),
                images: HashMap::new(),
                current_biome: Biome::NoBiome,
                is_fullscreen: true,
            };

            install_image_loaders(&cc.egui_ctx);

            for biome in Biome::iter() {
                let color_image = &images[&biome];
                let handle = cc.egui_ctx.load_texture(
                    biome.to_string(),
                    color_image.clone(),
                    TextureOptions::LINEAR,
                );
                let texture = SizedTexture::new(handle.id(), egui::vec2(1920.0, 1080.0));
                let image = Image::from_texture(texture);

                app.image_handles.insert(biome, handle);
                app.images.insert(biome, image);
            }

            cc.egui_ctx.send_viewport_cmd(egui::ViewportCommand::Focus);

            Ok(Box::new(app))
        }),
    )?;

    Ok(())
}
