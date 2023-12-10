use std::borrow::BorrowMut;



mod commands;
mod decoder;
mod gq_gmc_protocol;
mod helpers;


fn main() {

    let serial_port_file : String = std::env::args().nth(1).expect("No serial port file provided");
    let baud_rate : String = std::env::args().nth(2).expect("No baud rate provided. Please check your device settings (Others -> Comport Baud Rate");

    let mut serial_port = gq_gmc_protocol::connect_to_device(serial_port_file, baud_rate.parse::<u32>().unwrap()).open().expect("Unable to open serial port");

    let command_type_prefix : String = std::env::args().nth(3).expect("No command type parameter received, expected either '--timesync' '--log', or '--dump'");

    if command_type_prefix == "--timesync" {

        // Start by getting current time


    } else if command_type_prefix == "--log" {

    }

    else if command_type_prefix == "--dump" {

        gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETVER.to_string()).unwrap();

        let mut device_name_response_buffer: Vec<u8> = vec![0; 32];

        serial_port.read(&mut device_name_response_buffer.as_mut_slice()).expect("Couldn't read version response from serial port");

        let device_name = std::str::from_utf8(&device_name_response_buffer).expect("Unable to decode response from serial port into Unicode string");

        let mut device_cfg_response_buffer : Vec<u8> : vec![0, 512];
        
        gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETCFG.to_string()).unwrap();

        serial_port.read(&mut device_cfg_response_buffer.as_mut_slice()).expect("Couldn't read cfg dump from serial port");

        let dump_memory_filename = device_name.to_owned() + "__dump.bin";
        let dump_memory_file_path = std::path::Path::new(&dump_memory_filename);

        std::fs::write(dump_memory_file_path, device_cfg_response_buffer).unwrap();

    }



    // gq_gmc_protocol::send_msg(&mut *serial_port,  commands::ParameterlessCommand::GETGYRO.to_string()).unwrap();
    


    // println!("Returned {:?}", serial_buf);

    // let (x_pos, y_pos, z_pos) = decoder::decode_gyro_data(&serial_buf[0 .. 7]);

    // println!("X : {} | Y : {} | Z: {}", x_pos, y_pos, z_pos);

    // Print the response as a string
    // let response = std::str::from_utf8(&serial_buf).expect("invalid utf-8 sequence");
    //println!("{}", response);
}