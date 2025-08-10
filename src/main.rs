mod midi;
mod music;
mod window;
use window::MyEguiApp;
fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.inner_size = Some((400.0, 180.0).into());
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .unwrap();
}
