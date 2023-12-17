use eframe::egui;

use crate::gq_gmc_protocol;
use crate::commands;

pub struct MyApp {

    pub serial_port : Box<dyn serialport::SerialPort>,

    pub device_name : String,

    pub alarm_enabled : bool,

    pub speaker_enabled: bool, 

    pub debug_echo_enabled : bool,

    pub wifi_enabled: bool,

}

// impl Default for MyApp {
//     fn default() -> Self {
//         Self { }
//     }
// }


impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("GQ GMC Control");

            // gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ParameterlessCommand::GETVER.to_string()).expect("Error querying device info!");

            // let mut device_info_resp_buffer : Vec<u8> = vec![0; 15];

            // self.serial_port.read(&mut device_info_resp_buffer.as_mut_slice());

            // ui.label(std::str::from_utf8(&device_info_resp_buffer).expect("Error converting device info to string"));

            //ui.label("Device buttons");

            ui.label(&self.device_name);

            ui.horizontal(|row_ui| {

                if row_ui.checkbox(&mut self.alarm_enabled, "Alarm?").clicked() {

                    if self.alarm_enabled {
                        gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ParameterlessCommand::ALARM1.to_string()).expect("Error enabling alarm on device");
                    } 
                    else {
                        gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ParameterlessCommand::ALARM0.to_string()).expect("Error disabling alarm on device");
                    }
                }

                if row_ui.checkbox(&mut self.speaker_enabled, "Speaker?").clicked() {

                    if self.speaker_enabled {
                        gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ParameterlessCommand::SPEAKER1.to_string()).expect("Error enabling speaker");
                    }

                    else {
                        gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ParameterlessCommand::SPEAKER0.to_string()).expect("Error disabling speaker");
                    }
                }

                if row_ui.checkbox(&mut self.debug_echo_enabled, "Debug (Echo)?").clicked() {

                    if self.debug_echo_enabled {

                        gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ParameterlessCommand::EchoON.to_string()).expect("Error enabling echo");
                    }
                    else {
                        gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ParameterlessCommand::EchoOFF.to_string()).expect("Error disabling echo");
                    }
                }

                if row_ui.checkbox(&mut self.wifi_enabled, "WiFi?").clicked() {
                    
                    if self.wifi_enabled {

                        gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ParameterlessCommand::WiFiON.to_string()).expect("Error enabling WiFi");
                    }
                    else {
                        gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ParameterlessCommand::WiFiOFF.to_string()).expect("Error disabling WiFi");
                    }
                }
            }); // end horizontal

            ui.separator();

            ui.vertical(|col_ui| {

                // if row_ui.add(egui::ImageButton::new(egui::include_image!("assets/power-off-solid.svg"))).clicked() {
                //     println!("Pressing Key3");
                // }

                if col_ui.add_sized([100.0, 100.0], egui::ImageButton::new(egui::include_image!("assets/power-off-solid.svg"))).clicked() {
                    //println!("Pressing Key3");
                    gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ButtonKeyCommand::KEY3.to_string()).expect("Error couldn't press power button on device");
                }
                
                // if row_ui.add(egui::ImageButton::new(egui::include_image!("assets/caret-up-solid.svg"))).clicked() {
                    if col_ui.add_sized([100.0, 100.0], egui::ImageButton::new(egui::include_image!("assets/caret-up-solid.svg"))).clicked() {

                    //println!("Pressing Key2");
                    gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ButtonKeyCommand::KEY2.to_string()).expect("Error couldn't press up arrow on device");
                }
                //if row_ui.add(egui::ImageButton::new(egui::include_image!("assets/caret-down-solid.svg"))).clicked() {
                    if col_ui.add_sized([100.0, 100.0], egui::ImageButton::new(egui::include_image!("assets/caret-down-solid.svg"))).clicked() {

                    //println!("Pressing Key1");
                    gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ButtonKeyCommand::KEY1.to_string()).expect("Error couldn't press down arrow on device");
                }

                //if row_ui.add(egui::ImageButton::new(egui::include_image!("assets/clock-rotate-left-solid.svg"))).clicked() {
                    if col_ui.add_sized([100.0, 100.0], egui::ImageButton::new(egui::include_image!("assets/clock-rotate-left-solid.svg"))).clicked() {

                        gq_gmc_protocol::send_msg(&mut *self.serial_port, commands::ButtonKeyCommand::KEY0.to_string()).expect("Error couldn't press back button on device");

                    //println!("Pressing Key0");
                }

            }); // end row

        });
    }
}