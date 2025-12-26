use eframe::{
    egui,
    epaint::text::{FontInsert, InsertFontFamily},
};

pub struct MainWindow {
    id: String,
    pass: String,
    domein: String,
    phone_number: String,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            id: "1001".to_string(),
            pass: "p@ssw0rd".to_string(),
            domein: "test.u.biztel.jp".to_string(),
            phone_number: "".to_string(),
        }
    }
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
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        replace_fonts(&cc.egui_ctx);
        add_font(&cc.egui_ctx);
        Self::default()
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.text_edit_singleline(&mut self.phone_number);

                ui.horizontal(|ui| {
                    if ui.button("通話").clicked() {
                        // TODO PJSIPを呼ぶ
                    }
                    if ui.button("切断").clicked() {
                        // TODO PJSIPを呼ぶ
                    }
                });
            });
        });
    }
}
