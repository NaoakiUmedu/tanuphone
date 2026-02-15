use crate::pjsua_wrapper::TPjsuaWrapper;
use crate::MainWindow;
use eframe::egui;

pub fn phone_mode_view(main: &mut MainWindow, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut main.to_number);
            ui.label(main.get_string_from_callstatus());
        });

        ui.horizontal(|ui| {
            if ui.button("通話").clicked() {
                if main.to_number != "" && main.domain != "" && main.registered == true {
                    main.pjsua.callto(&main.to_number, &main.domain);
                }
            }
            if ui.button("切断").clicked() {
                main.pjsua.hangup();
            }
        });
    });
}
