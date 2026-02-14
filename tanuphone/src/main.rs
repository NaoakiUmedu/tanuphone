use crate::main_window::MainWindow;

mod main_window;
mod message;
mod pjsua_wrapper;
mod setting;

fn main() -> Result<(), eframe::Error> {
    let rx = pjsua_wrapper::init();

    //let id = pjsua_wrapper::account_add(SIP_USER, SIP_PASSWD, SIP_DOMAIN);

    //pjsua_wrapper::callto(id, "sip:1002@test.u.biztel.jp");

    //pjsua_wrapper::destroy();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "tanuphone",
        options,
        Box::new(|cc| Ok(Box::new(MainWindow::new(cc, rx)))),
    )
}
