pub mod knobConfig;

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
                pcpanel.read(&mut buf).unwrap(); // Blocking, waits until data is available
                if buf[0] == 1 {
                    println!("Knob {} turned to {}", buf[1], buf[2]);

                    // Tell LEDs to update
                    let header = [0x06, 0x04].to_vec();
                    let message = [0x03, 128, 255, 255, 200, 0x00, 0x01].to_vec();
                    let full_message = [header, message.clone()].concat();
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
