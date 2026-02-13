use eframe::egui;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Threaded Dialog Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}

struct MyApp {
    picked_path: Option<String>,
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl MyApp {
    fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            picked_path: None,
            sender,
            receiver,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(path) = self.receiver.try_recv() {
            self.picked_path = Some(path);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open Media File").clicked() {
                let sender = self.sender.clone();
                let ctx_clone = ctx.clone();

                thread::spawn(move || {
                    let file = rfd::FileDialog::new()
                        .add_filter("Video", &["mp4", "webp", "mkv"])
                        .pick_file();
                    if let Some(path) = file {
                        let _ = sender.send(path.display().to_string());
                        ctx_clone.request_repaint();
                    }
                });
            }

            if let Some(path) = &self.picked_path {
                ui.label(format!("Selected: {}", path));
            }
        });
    }
}
