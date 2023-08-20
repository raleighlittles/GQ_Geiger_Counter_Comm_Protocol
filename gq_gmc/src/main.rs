use gq_gmc_protocol::send_msg;


mod gq_gmc_protocol;

fn main() {
    let serial_port_file : String = std::env::args().nth(1).expect("No serial port file provided");
    let baud_rate : String = std::env::args().nth(2).expect("No baud rate provided. Please check your device settings (Others -> Comport Baud Rate");

    let mut serial_port = gq_gmc_protocol::connect_to_device(serial_port_file, baud_rate.parse::<u32>().unwrap()).open().expect("Unable to open serial port!");

    //send_msg(serial_port, "GETVER");
    //send_msg(&*serial_port, "GETVER".to_string()); // expected &mut dyn SerialPort, found &dyn SerialPort
    send_msg(&mut *serial_port, "KEY3".to_string()).unwrap();
}