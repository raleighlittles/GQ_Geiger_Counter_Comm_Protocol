
use crate::helpers;

pub fn decode_gyro_data(sensor_bytes: &[u8]) -> (i16, i16, i16) {
    
    if sensor_bytes.len() != 7 {
        panic!("Received incorrect amount of data for gyroscope. Expected 7 bytes");
    }

    if sensor_bytes[6] != 0xAA {
        panic!("Expected byte 7 to always be 0xAA -- verify gyroscope is calibrated?");
    }

    // TODO Refactor this

    let x_pos = u16::from_be_bytes([sensor_bytes[0], sensor_bytes[1]]);
    let y_pos = u16::from_be_bytes([sensor_bytes[2], sensor_bytes[3]]);
    let z_pos = u16::from_be_bytes([sensor_bytes[4], sensor_bytes[5]]);

    println!("[RAW] X = {} Y = {} Z = {}", x_pos, y_pos, z_pos);

    return (helpers::convert_to_percentage_of_max(x_pos), helpers::convert_to_percentage_of_max(y_pos), helpers::convert_to_percentage_of_max(z_pos));

}