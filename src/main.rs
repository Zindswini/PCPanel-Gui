extern crate hidapi;

use hidapi::{HidApi, HidDevice};

fn main() {
    //println!("Printing all available hid devices:");

    match HidApi::new() {
        Ok(api) => {
            // make spot for device we are looking for
            let pcpanel: HidDevice = api.open(0x0483, 0xa3c4).unwrap();
            println!("Device: {:?}", pcpanel.get_device_info());

            // tell the lights to turn on

            // print the state of the device in a loop
            loop {
                let mut buf = [0u8; 3];
                pcpanel.read(&mut buf).unwrap();
                if buf[0] == 1 {
                    println!("Knob {} turned to {}", buf[1], buf[2]);
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
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
