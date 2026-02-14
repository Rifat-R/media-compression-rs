use eframe::egui;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

use media_compression_rs::{CompressionFormat, compress_media};

const ALLOWED_FILETYPES: &[&str] = &["mp4", "mkv", "webp"];

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
                        .add_filter("Video", ALLOWED_FILETYPES)
                        .pick_file();
                    if let Some(path) = file {
                        let _ = sender.send(path.display().to_string());
                        ctx_clone.request_repaint();
                    }
                });
            }

            if let Some(path) = &self.picked_path {
                ui.label(format!("Selected: {}", path));
                if ui.button("Compress").clicked() {
                    let input_path_str = path.clone();
                    // let output_path = format!("compressed_{}", path);
                    thread::spawn(move || {
                        let input_path = Path::new(&input_path_str);
                        let compression_format = input_path
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| match ext {
                                "mkv" => CompressionFormat::Mkv,
                                "mp4" => CompressionFormat::Mp4,
                                "webp" => CompressionFormat::Webp,
                                _ => panic!("Wrong format selected"),
                            })
                            .expect("File has no valid extension");

                        let parent = input_path.parent().unwrap_or(Path::new("."));

                        let filename = input_path.file_name().unwrap();

                        let new_filename = format!("compressed_{}", filename.to_string_lossy());

                        let output_path_buf = parent.join(new_filename);

                        let output_path_str = output_path_buf.to_string_lossy().to_string();

                        let _ =
                            compress_media(&input_path_str, &output_path_str, compression_format);
                    });
                }
            }
        });
    }
}
