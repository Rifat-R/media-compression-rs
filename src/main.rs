use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "File Dialog Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    picked_path: Option<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { picked_path: None }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open File").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if ui.button("Save File").clicked() {
                if let Some(path) = rfd::FileDialog::new().save_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(path) = &self.picked_path {
                ui.label(format!("Selected: {}", path));
            }
        });
    }
}
