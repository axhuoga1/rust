use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Plot Example",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    t: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { t: 0.0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.t += 0.01; // 时间步进
        egui::CentralPanel::default().show(ctx, |ui| {
            let points: Vec<_> = (0..100)
                .map(|i| {
                    let x = i as f64 * 0.1;
                    let y = (x + self.t).sin();
                    [x, y]
                })
                .collect();
            ui.label("Dynamic Sine Wave:");
            egui::plot::Plot::new("sine_wave").show(ui, |plot_ui| {
                plot_ui.line(egui::plot::Line::new(egui::plot::Values::from_values(points)));
            });
        });
    }
}
