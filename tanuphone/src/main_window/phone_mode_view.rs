use crate::pjsua_wrapper;
use crate::usecases;
use crate::MainWindow;
use eframe::egui;

pub fn phone_mode_view(main: &mut MainWindow, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut main.to_number);
            // TODO こんな感じにしてみる https://qiita.com/hasebems/items/2e42042d06bfb10c3b0b
            let callstatus_label = ui.label(main.get_string_from_callstatus());
            let time = ui.input(|i| i.time).round() as i32;
            if time % 2 == 0  && main.is_incomming {
                callstatus_label.highlight();
            }
        });

        ui.horizontal(|ui| {
            if ui.button("通話").clicked() {
                if main.is_incomming {
                    pjsua_wrapper::stop_ring(&mut main.ringtone);
                    usecases::answer::answer(main.incomming_call_id, &main.pjsua);
                    main.incomming_call_id = -1;
                    main.is_incomming = false;
                } else {
                    if main.to_number != "" && main.domain != "" && main.registered == true {
                        usecases::callto::callto(&main.to_number, &main.domain, &main.pjsua);
                    }
                }
            }
            if ui.button("切断").clicked() {
                usecases::hangup::hangup(&main.pjsua);
            }
        });
    });
}
