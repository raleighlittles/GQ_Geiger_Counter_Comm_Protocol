use gq_gmc_protocol::send_msg;

mod gq_gmc_protocol;
mod commands;

fn main() {
    let serial_port_file : String = std::env::args().nth(1).expect("No serial port file provided");
    let baud_rate : String = std::env::args().nth(2).expect("No baud rate provided. Please check your device settings (Others -> Comport Baud Rate");

    let mut serial_port = gq_gmc_protocol::connect_to_device(serial_port_file, baud_rate.parse::<u32>().unwrap()).open().expect("Unable to open serial port!");

    send_msg(&mut *serial_port,  commands::ParameterlessCommand::GETGYRO.to_string()).unwrap();
    
    let mut serial_buf: Vec<u8> = vec![0; 32];
    serial_port.read(serial_buf.as_mut_slice()).expect("Found no data!");
    println!("Returned {:?}", serial_buf);

    let response = std::str::from_utf8(&serial_buf).expect("invalid utf-8 sequence");
    println!("{}", response);
}