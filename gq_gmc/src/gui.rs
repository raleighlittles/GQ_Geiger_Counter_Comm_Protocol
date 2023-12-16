use eframe::egui;

pub struct MyApp {

}

impl Default for MyApp {
    fn default() -> Self {
        Self { }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("GQ GMC Geiger Counter control app");

            if ui.button("⏻").clicked() {
                println!("Pressing Key3");
            }

        });
    }
}