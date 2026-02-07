use std::sync::mpsc::Receiver;

use eframe::{
    egui::{self, Context},
    epaint::text::{FontInsert, InsertFontFamily},
};

use crate::{
    message::{Message, MessageType},
    pjsua_wrapper::{self, print_log},
};

#[derive(PartialEq, Debug, Clone, Copy)]
enum CallStatus {
    Disconnected,
    Calling,
    Connecting,
    Confirmed,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum ViewMode {
    Phone,
    Setting,
}

pub struct MainWindow {
    my_number: String,
    password: String,
    domain: String,
    to_number: String,
    call_status: CallStatus,
    view_mode: ViewMode,
    rx: Receiver<Message>,
    debug_line: String,
    registered: bool,
}

// Demonstrates how to add a font to the existing ones
fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "my_font",
        egui::FontData::from_static(include_bytes!("../fonts/Cica/Cica-Regular.ttf")),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));
}

// Demonstrates how to replace all fonts.
fn replace_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../fonts/Cica/Cica-Regular.ttf"
        ))),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

impl MainWindow {
    pub fn new(cc: &eframe::CreationContext<'_>, rx: Receiver<Message>) -> Self {
        replace_fonts(&cc.egui_ctx);
        add_font(&cc.egui_ctx);
        Self {
            my_number: "".to_string(),
            password: "".to_string(),
            domain: "".to_string(),
            to_number: "".to_string(),
            call_status: CallStatus::Disconnected,
            view_mode: ViewMode::Phone,
            rx: rx,
            debug_line: "".to_string(),
            registered: false,
        }
    }

    fn phone_mode_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.to_number);
                ui.label(self.get_string_from_callstatus());
            });

            ui.horizontal(|ui| {
                if ui.button("通話").clicked() {
                    if self.to_number != "" && self.domain != "" && self.registered == true {
                        pjsua_wrapper::callto(&self.to_number, &self.domain);
                    }
                }
                if ui.button("切断").clicked() {
                    pjsua_wrapper::hangup();
                }
            });
        });
    }

    fn setting_mode_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let user_label = ui.label("SIP USER:");
            ui.text_edit_singleline(&mut self.my_number)
                .labelled_by(user_label.id);
            let password_label = ui.label("PASSWORD:");
            ui.text_edit_singleline(&mut self.password)
                .labelled_by(password_label.id);
            let domain_label = ui.label("SIP SERVER DOMAIN:");
            ui.text_edit_singleline(&mut self.domain)
                .labelled_by(domain_label.id);

            // TODO ちゃんとする
            if ui.button("レジする").clicked() {
                pjsua_wrapper::account_add(&self.my_number, &self.password, &self.domain);
            }
        });
    }

    fn handle_message(&mut self, ctx: &Context) {
        while let Ok(message) = self.rx.try_recv() {
            if message.message_type == MessageType::RegisterComplete {
                self.registered = true;
            }
            match message.message_type {
                MessageType::RegisterComplete => self.on_register_complete(),
                MessageType::OnCallState => self.on_call_state(message.message),
                _ => print_log(pjsua_wrapper::LogLevel::LogLevel1, &format!("@@@@@ Action not defined for {:?}", message.message_type)),
            }
            ctx.request_repaint();
        }
    }

    fn on_register_complete(&mut self) {
        self.registered = true;
    }

    fn on_call_state(&mut self, message: String) {
        print_log(pjsua_wrapper::LogLevel::LogLevel1, &format!("@@@@@ received status = {}", message));
        match &*message {
            "DISCONNECTED" => self.call_status = CallStatus::Disconnected,
            "CALLING" => self.call_status = CallStatus::Calling,
            "CONNECTING" => self.call_status = CallStatus::Connecting,
            "CONFIRMED" => self.call_status = CallStatus::Confirmed,
            _ => print_log(pjsua_wrapper::LogLevel::LogLevel1, &format!("@@@@@ Action not defined for {}", message)),
        }
        self.debug_line = message;
    }

    fn get_string_from_callstatus(&mut self) -> String {
        match self.call_status {
            CallStatus::Disconnected => "".to_string(),
            CallStatus::Calling => "発信中/着信中".to_string(),
            CallStatus::Connecting => "発信中/着信中".to_string(),
            CallStatus::Confirmed => "通話中".to_string(),
            _ => "".to_string(),
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.debug_line);
            ui.horizontal(|ui| {
                ui.label("Mode");
                ui.radio_value(&mut self.view_mode, ViewMode::Phone, "Phone");
                ui.radio_value(&mut self.view_mode, ViewMode::Setting, "Setting");
            });
            match self.view_mode {
                ViewMode::Phone => self.phone_mode_view(ui),
                ViewMode::Setting => self.setting_mode_view(ui),
            }
        });

        self.handle_message(ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        pjsua_wrapper::destroy();
    }
}
