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

            ui.heading("GQ GMC Control");

            ui.label("Device buttons");

            if ui.add(egui::ImageButton::new(egui::include_image!("./assets/power-off-solid.svg"))).clicked() {
                println!("Pressing Key3");
            }
            
            if ui.add(egui::ImageButton::new(egui::include_image!("./assets/caret-up-solid.svg"))).clicked() {
                println!("Pressing Key2");
            }
            if ui.add(egui::ImageButton::new(egui::include_image!("./assets/caret-down-solid.svg"))).clicked() {
                println!("Pressing Key1");
            }

            if ui.add(egui::ImageButton::new(egui::include_image!("./assets/clock-rotate-left-solid.svg"))).clicked() {
                println!("Pressing Key0");
            }

            ui.end_row();

        });
    }
}