use crate::pjsua_wrapper;
use crate::MainWindow;
use eframe::egui;
use crate::setting;

pub fn setting_mode_view(main: &mut MainWindow, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        let user_label = ui.label("SIP USER:");
        ui.text_edit_singleline(&mut main.my_number)
            .labelled_by(user_label.id);
        let password_label = ui.label("PASSWORD:");
        ui.text_edit_singleline(&mut main.password)
            .labelled_by(password_label.id);
        let domain_label = ui.label("SIP SERVER DOMAIN:");
        ui.text_edit_singleline(&mut main.domain)
            .labelled_by(domain_label.id);

        // TODO ちゃんとする
        if ui.button("レジする").clicked() {
            pjsua_wrapper::account_add(&main.my_number, &main.password, &main.domain);
        }

        if ui.button("保存").clicked() {
            save(main);
        }
    });
}

fn save(main: &mut MainWindow) {
    let settings = setting::Settings {settings: vec![
        setting::Setting {
            user : main.my_number.clone(),
            password : main.password.clone(),
            domain: main.domain.clone(),
        }
    ]};
    match setting::write_file(settings) {
        Ok(_) => main.debug_line = "Save OK".to_string(),
        Err(_) => main.debug_line = "Save NG".to_string(),
    }
}

pub fn load(main: &mut MainWindow) {
    if let Ok(settings) = setting::read_file() {
        // TODO とりあえず1端末だけ管理する
        if settings.settings.len() >= 1 {
            main.my_number = settings.settings[0].user.clone();
            main.password = settings.settings[0].password.clone();
            main.domain = settings.settings[0].domain.clone();
            return;
        }
    }
    pjsua_wrapper::print_log(pjsua_wrapper::LogLevel::LogLevel1, "No setting found");

}
