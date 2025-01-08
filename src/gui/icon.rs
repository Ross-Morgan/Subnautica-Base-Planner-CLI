use eframe::egui::IconData;

pub(super) fn load_icon() -> IconData {
    // let mut rgba = Vec::new();
    // img.write_to(&mut rgba, image::ImageFormat::Png).expect("Couldn't load icon");

    let icon = image::open("assets/icon.png")
        .expect("Couldn't load icon")
        .to_rgba8();

    let (width, height) = icon.dimensions();
    

    IconData {
        rgba: icon.into_raw(),
        width,
        height,
    }
}
