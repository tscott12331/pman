use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
                .with_inner_size([680.0, 960.0])
                .with_min_inner_size([340.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native("pman",
        options,
        Box::new(|ctx| Ok(Box::from(pman::app::PManApp::new(ctx))))
    )
}
