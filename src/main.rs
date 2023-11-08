/* extern crate hidapi;

use hidapi::HidApi;
use pipewire::sys::pw_protocol_native_message;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let api = HidApi::new()?;
    let pcpanel = api.open(0x0483, 0xa3c4)?;

    let prefix_mini: u8 = 0x06;
    let custom_knob: u8 = 0x02;
    let brightness: u8 = 100; // 100 is full brightness

    // Define commands
    let _static_color_command: u8 = 0x01; // Format: [prefix_mini, custom_knob, static_color_command, red, green, blue]
    let _gradient_color_command = 0x02; // Format: [prefix_mini, custom_knob, gradient_color_command, [color1], [color2]]

    let red: u8 = apply_brightness(255, brightness); // Full brightness red
    let green: u8 = apply_brightness(0, brightness); // No green
    let blue: u8 = apply_brightness(0, brightness); // No blue

    // General format: [prefix_mini, custom_knob, command, [data]]

    // Construct the message
    let header = [prefix_mini, custom_knob].to_vec();
    let mut knob1 = [0x01, red, green, blue].to_vec();
    knob1.resize(7, 0x00);

    let message = [
        header,
        knob1.clone(),
        knob1.clone(),
        knob1.clone(),
        knob1.clone(),
    ]
    .concat();

    // Send the message
    pcpanel.write(&message)?;

    Ok(())
}

fn apply_brightness(color_value: u8, brightness: u8) -> u8 {
    // Apply brightness to the color value
    // Assuming brightness is a percentage (0-100)
    ((color_value as f32 * (brightness as f32 / 100.0)) as u8) & 0xFF
} */

extern crate hidapi;

use hidapi::{HidApi, HidDevice};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match HidApi::new() {
        Ok(api) => {
            // make spot for device we are looking for
            let pcpanel: HidDevice = api.open(0x0483, 0xa3c4).unwrap();
            println!("Device: {:?}", pcpanel.get_device_info());

            // tell the lights to turn on

            // print the state of the device in a loop
            loop {
                let mut buf = [0u8; 3];
                println!("Reading...");
                pcpanel.read(&mut buf).unwrap();
                if buf[0] == 1 {
                    println!("Knob {} turned to {}", buf[1], buf[2]);

                    // Tell LEDs to update
                    let header = [0x06, 0x02].to_vec();
                    let message = [0x02, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF].to_vec();
                    let full_message = [
                        header,
                        message.clone(),
                        message.clone(),
                        message.clone(),
                        message.clone(),
                    ]
                    .concat();
                    pcpanel.write(&full_message)?;
                }
                if buf[0] == 2 {
                    if buf[2] == 1 {
                        println!("Button {} pressed", buf[1]);
                    }
                    if buf[2] == 0 {
                        println!("Button {} released", buf[1]);
                    }
                }
                //println!("State: {:?}", buf);
            }
        }
        Err(_) => todo!(),
    }
}
