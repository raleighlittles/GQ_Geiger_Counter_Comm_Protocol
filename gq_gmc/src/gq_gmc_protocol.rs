

const MESSAGE_START : &str = "<";
const MESSAGE_END : &str = ">>";

pub fn connect_to_device(character_device_file : String, baud_rate : u32) -> serialport::SerialPortBuilder {

    return serialport::new(character_device_file, baud_rate).timeout(std::time::Duration::ZERO);
}

fn build_msg(msg_contents : String) -> String {

    let mut msg : String = "".to_string();

    msg.push_str(MESSAGE_START);
    msg.push_str(&msg_contents);
    msg.push_str(MESSAGE_END);

    return msg;
}

pub fn send_msg(serial_port : &mut dyn serialport::SerialPort, msg_contents : String) -> Result<usize, std::io::Error> {

    let msg : String = build_msg(msg_contents);

    return serial_port.write(msg.as_bytes());
}