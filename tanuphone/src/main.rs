use crate::main_window::MainWindow;

mod main_window;
mod message;
mod pjsua_wrapper;
mod setting;
mod usecases;

fn main() -> Result<(), eframe::Error> {
    let pj = pjsua_wrapper::PjsuaImpl{};

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "tanuphone",
        options,
        Box::new(|cc| Ok(Box::new(MainWindow::new(cc, pj)))),
    )
}
