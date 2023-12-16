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

        let csv_filename : String = std::env::args().nth(4).expect("No CSV filename specified");

        // Prepare CSV file and setup column headers
        let mut csv_writer = csv::Writer::from_path(csv_filename).expect("Error: Can't initialize CSV file");

        csv_writer.write_record(&["Datetime ('YYMMDDHHMMSS')", "DeviceInfo", "Voltage", "Gyroscope (X; Y; Z;)", "Counts_Per_Second", "Max_CountsPerSecond", "Counts_Per_Minute"]).expect("Unable to write column headers for CSV");

        loop {

            // get Datetime
            gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETDATETIME.to_string()).expect("Error querying device datetime");

            let mut device_datetime_resp_buffer: Vec<u8> = vec![0; 7];

            serial_port.read(&mut device_datetime_resp_buffer.as_mut_slice()).expect("Couldn't read version response from serial port");

            if device_datetime_resp_buffer[6] != 0xAA {
                
                panic!("Invalid response received after querying datetime, expected last byte to be 0xAA but got '{}' instead", device_datetime_resp_buffer[6]);
            }

            //let device_datetime = std::str::from_utf8(&device_datetime_resp_buffer[0 .. 6]).unwrap();
            //let device_datetime = format!("{:?}", device_datetime_resp_buffer[0..6]);
            let device_datetime = device_datetime_resp_buffer[0..6].into_iter().map(|i| i.to_string()).collect::<String>();

            // get "DeviceInfo"
            gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETVER.to_string()).expect("Error querying device information");
            let mut device_name_resp_buffer: Vec<u8> = vec![0; 15];
            serial_port.read(&mut device_name_resp_buffer.as_mut_slice()).expect("Couldn't read version response from serial port");
            let device_name = std::str::from_utf8(&device_name_resp_buffer).expect("Unable to decode response from serial port into Unicode string");

            // get Voltage
            gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETVOLT.to_string()).expect("Error querying device voltage");
            let mut voltage_resp_buffer : Vec<u8> = vec![0; 5];
            serial_port.read(&mut voltage_resp_buffer.as_mut_slice()).expect("Couldn't read voltage from device");
            let device_voltage = std::str::from_utf8(&voltage_resp_buffer).expect("Unable to decode voltage reading from serial port into Unicode string");

            // get Gyroscope
            gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETGYRO.to_string()).expect("Error querying device gyroscope data");
            let mut device_gyro_resp_bufer : Vec<u8> = vec![0; 7];
            serial_port.read(&mut device_gyro_resp_bufer).expect("Unable to read gyroscope response from device");
            let gyro_data = decoder::decode_gyro_data(&device_gyro_resp_bufer);
            let gyro_data_printable = format!("({}; {}; {};)", gyro_data.0, gyro_data.1, gyro_data.2);

            // get CountsPerSecond
            gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETCPS.to_string()).expect("Error querying device counts per second");
            let mut device_cps_resp_buffer : Vec<u8> = vec![0; 4];
            serial_port.read(&mut device_cps_resp_buffer).expect("Unable to read counts per second value from device");
            let counts_per_second = u32::from_le_bytes(device_cps_resp_buffer.as_slice().try_into().unwrap());

            // get Max CountsPerSecond
            gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETMAXCPS.to_string()).expect("Error querying max CPS from device");
            let mut device_max_cps_resp_buffer : Vec<u8> = vec![0; 4];
            serial_port.read(&mut device_max_cps_resp_buffer).expect("Unable to read max counts per second value");
            let max_cps = u32::from_le_bytes(device_max_cps_resp_buffer.as_slice().try_into().unwrap());

            // get CountsPerMinute
            gq_gmc_protocol::send_msg(&mut *serial_port, commands::ParameterlessCommand::GETCPM.to_string()).expect("Error querying counts per minute");
            let mut device_cpm_resp_buffer : Vec<u8> = vec![0; 4];
            serial_port.read(&mut device_cpm_resp_buffer).expect("Unable to read counts per minute");
            let counts_per_minute = u32::from_be_bytes(device_cpm_resp_buffer.as_slice().try_into().unwrap());

            println!("Writing values to CSV: {}, {}, {}", device_datetime, device_name, device_voltage);

            csv_writer.write_record(&[device_datetime.to_string(), device_name.to_string(), device_voltage.to_string(), gyro_data_printable, counts_per_second.to_string(), max_cps.to_string(), counts_per_minute.to_string()]).expect("Error couldn't write data to CSV file");

            csv_writer.flush().expect("Error flushing CSV file");

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

                gq_gmc_protocol::send_msg(&mut *serial_port, user_specified_cmd).expect("Error: couldn't send command to device");

                let mut user_cmd_resp_buffer : Vec<u8> = vec![0; 32];

                serial_port.read(&mut user_cmd_resp_buffer.as_mut_slice()).expect("Couldn't read response buffer from device");

                let user_cmd_resp = std::str::from_utf8(&user_cmd_resp_buffer).expect("Invalid UTF-8 sequence");

                println!("Response: (Parsed) '{}' (Raw) {:?}", user_cmd_resp, user_cmd_resp_buffer);


                // exit the loop
                break;
            }
        }

        if !is_valid_cmd {
            
            panic!("Error: The command provided is not a recognized command");
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