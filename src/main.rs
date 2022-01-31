mod ui_app;

use eframe::run_native;
use eframe::NativeOptions;
use egui::Vec2;

use std::io;

fn main() {
    let app = ui_app::Archiver::new("".to_string(), "".to_string(), ui_app::Extension::Zip, false);
    let icon = image::open("archiver.ico")
        .expect("Failed to open icon path")
        .to_rgba8();
    let size = &(icon.width(), icon.height());
    let options = eframe::NativeOptions {
        icon_data: Some(eframe::epi::IconData {
            rgba: icon.into_raw(),
            width: size.0,
            height: size.1,
        }),
        initial_window_size: Some(Vec2::new(300.0, 240.0)),
        ..Default::default()
    };

    run_native(Box::new(app), options);
}
