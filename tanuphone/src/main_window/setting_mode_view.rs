use crate::pjsua_wrapper;
use crate::MainWindow;
use eframe::egui;

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
    });
}
