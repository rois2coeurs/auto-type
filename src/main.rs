use autopilot::key::{Code, Flag};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "AutoTyper",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    text: String,
    wpm: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: "test".to_string(),
            wpm: 200.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let alt_v = egui::KeyboardShortcut::new(egui::Modifiers::ALT, egui::Key::V);

            ui.input_mut(|i| {
                if i.consume_shortcut(&alt_v) {
                    paste(&self.text, self.wpm);

                    i.events
                        .retain(|event| !matches!(event, egui::Event::Text(_)));
                }
            });
            ui.heading("My fucking super duper AutoTyper");

            ui.text_edit_multiline(&mut self.text);

            ui.add(
                egui::Slider::new(&mut self.wpm, 0.0..=1000.0)
                    .text("Words Per Minute (Adding more can cause AutoTypping issues)"),
            );

            if ui.button("AutoType").clicked() {
                paste(&self.text, self.wpm);
            }
        });
    }
}

fn paste(text: &String, wpm: f64) {
    autopilot::key::tap(&Code(autopilot::key::KeyCode::Tab), &[Flag::Alt], 10, 50);
    autopilot::key::type_string(text, &[], wpm, 0.);
}
