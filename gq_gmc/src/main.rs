use chrono::Datelike;
use commands::ParameterlessCommand;


mod commands;
mod decoder;
mod gq_gmc_protocol;
mod helpers;


fn main() {

    let serial_port_file : String = std::env::args().nth(1).expect("No serial port file provided");
    let baud_rate : String = std::env::args().nth(2).expect("No baud rate provided. Please check your device settings (Others -> Comport Baud Rate");

    let mut serial_port = gq_gmc_protocol::connect_to_device(serial_port_file, baud_rate.parse::<u32>().unwrap()).open().expect("Unable to open serial port");

    let command_type_prefix : String = std::env::args().nth(3).expect("No command type parameter received, expected either '--timesync' '--log', or '--dump'");

    // Synchronizes the geiger counter's datetime with the host PC datetime
    if command_type_prefix == "--timesync" {

        // Start by getting current time
        let current_timestamp = chrono::Utc::now();
        let current_timestamp_device_form = current_timestamp.format("%y%m%d%H%M%S");

        println!("Current timestamp in device form (YYMMDDHHMMSS) '{}'", current_timestamp_device_form);

        // Tell the device to change its time to the new one
        let set_datetime_msg = gq_gmc_protocol::build_msg("SETDATETIME[".to_owned() + &current_timestamp_device_form.to_string() + "]");

        gq_gmc_protocol::send_msg(&mut *serial_port, set_datetime_msg).unwrap();

        // Verify that the request was sent
        let mut set_datetime_resp_conf_buffer = vec![0; 1];

        serial_port.read(set_datetime_resp_conf_buffer.as_mut_slice()).expect("Error: Couldn't read response after set datetime command");

        if set_datetime_resp_conf_buffer[0] != 0xAA {
            panic!("Error: Expected 0xAA as response confirmation, instead received '{}'", set_datetime_resp_conf_buffer[0]);
        }
    }
    // Dump the geiger counter's configuration data to a file
    else if command_type_prefix == "--dump" {

        // See TODO-1
        // gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETVER.to_string()).unwrap();
        // let mut device_name_response_buffer: Vec<u8> = vec![0; 32];
        // serial_port.read(&mut device_name_response_buffer.as_mut_slice()).expect("Couldn't read version response from serial port");
        // let device_name = std::str::from_utf8(&device_name_response_buffer).expect("Unable to decode response from serial port into Unicode string").replace("+", "");
        //println!("Device name: '{}'", device_name);

        let mut device_cfg_response_buffer : Vec<u8> = vec![0; 512];
        
        gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETCFG.to_string()).unwrap();

        serial_port.read(&mut device_cfg_response_buffer.as_mut_slice()).expect("Error: Couldn't read cfg dump from serial port");

        let dump_memory_filename = "geiger_config.bin";

        println!("Creating file '{}'", dump_memory_filename);
        let dump_memory_file_path = std::path::Path::new(&dump_memory_filename);

        std::fs::write(dump_memory_file_path, device_cfg_response_buffer).unwrap();
    }
    // Log the geiger counter's measurements to a CSV file
    else if command_type_prefix == "--log" {

        // Prepare CSV file and setup column headers
        let mut csv_writer = csv::Writer::from_path("geiger_log.csv").expect("Error: Can't initialize CSV file");

        csv_writer.write_record(&["DeviceInfo", "Datetime ('YYMMDDHHMMSS')", "Voltage", "Gyroscope (X, Y, Z)", "Counts_Per_Second", "Max_CountsPerSecond", "Counts_Per_Minute"]).expect("Unable to write column headers for CSV");

        loop {

            // get "DeviceInfo"
            gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETVER.to_string()).expect("Error querying device information");
            let mut device_name_resp_buffer: Vec<u8> = vec![0; 32];
            serial_port.read(&mut device_name_resp_buffer.as_mut_slice()).expect("Couldn't read version response from serial port");
            let device_name = std::str::from_utf8(&device_name_resp_buffer).expect("Unable to decode response from serial port into Unicode string");
       
            // get Datetime
            gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETDATETIME.to_string()).expect("Error querying device datetime");

            let mut device_datetime_resp_buffer: Vec<u8> = vec![0; 7];

            if device_datetime_resp_buffer[6] != 0xAA {
                
                panic!("Invalid response received after querying datetime, expected last byte to be 0xAA but got '{}' instead", device_datetime_resp_buffer[6]);
            }

            let device_datetime = std::str::from_utf8(&device_datetime_resp_buffer[0 .. 6]).unwrap();


            // get Voltage
            gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETVOLT.to_string()).expect("Error querying device voltage");
        }

    }
    // Allow the user to call any of the parameterless commands
    else if command_type_prefix == "--command" {

        let user_specified_cmd: String = std::env::args().nth(4).expect("No command specified");

        let mut is_valid_cmd: bool = false;

        for command in commands::ParameterlessCommand::iterator() {

            if user_specified_cmd == command.to_string() {

                is_valid_cmd = true;

                // Send the command


                // exit the loop
                break;
            }
        }

        if !is_valid_cmd {
            
            panic!("Error: '{}' is not a recognized command", user_specified_cmd);
        }
        
    }



    // gq_gmc_protocol::send_msg(&mut *serial_port,  commands::ParameterlessCommand::GETGYRO.to_string()).unwrap();
    


    // println!("Returned {:?}", serial_buf);

    // let (x_pos, y_pos, z_pos) = decoder::decode_gyro_data(&serial_buf[0 .. 7]);

    // println!("X : {} | Y : {} | Z: {}", x_pos, y_pos, z_pos);

    // Print the response as a string
    // let response = std::str::from_utf8(&serial_buf).expect("invalid utf-8 sequence");
    //println!("{}", response);
}