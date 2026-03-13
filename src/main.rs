use autopilot::key::{Code, Flag};
// Import necessary stuff from the eframe crate
use eframe::egui;
extern crate autopilot;

// Main entry point of the program
fn main() -> Result<(), eframe::Error> {
    // Default window settings
    let options = eframe::NativeOptions::default();

    // Start the egui application!
    eframe::run_native(
        "AutoTyper", // Window title
        options,     // Window settings
        // This creates our app state. Don't worry too much about Box::new for now.
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

// This struct holds our application's data (state)
// `#[derive(Default)]` makes it easy to create a starting instance
struct MyApp {
    label: String, // For a text input field
    value: f64,    // For a slider
                   // We will add more fields here later!
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "test".to_string(),
            value: 200.0,
        }
    }
}

// Implement the eframe::App trait for MyApp, telling eframe how to run it
impl eframe::App for MyApp {
    // The `update` method is called every frame to draw the UI
    //
    // `&mut self` allows this method to modify MyApp's fields
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Show a central panel where we'll put our UI

        egui::CentralPanel::default().show(ctx, |ui| {
            // `ui` is our tool to add widgets

            // Add a heading
            ui.heading("My fucking super duper AutoTyper");

            ui.text_edit_multiline(&mut self.label);

            ui.add(
                egui::Slider::new(&mut self.value, 0.0..=1000.0)
                    .text("Words Per Minute (Adding more can cause AutoTypping issues)"),
            );

            // Add a button
            if ui.button("AutoType").clicked() {
                // If clicked, increase `self.value`
                autopilot::key::tap(&Code(autopilot::key::KeyCode::Tab), &[Flag::Alt], 10, 50);
                autopilot::key::type_string(&self.label, &[], self.value, 0.);
            }

            // Display the current state in a label
            ui.label(format!("Hello '{}', value: {}", self.label, self.value));
        });
    }
}
