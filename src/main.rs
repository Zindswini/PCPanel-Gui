extern crate hidapi;
extern crate pipewire;

pub mod panel_state;

use std::{thread::sleep, time::Duration, alloc::handle_alloc_error};

use hidapi::{HidApi, HidDevice};
use pipewire::{MainLoop, Context};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup instances
    let mut panel_state = panel_state::PCPanel::new();

    // Spawn pipewire thread
    let pw_handle = std::thread::spawn(move || {
        run_pipewire().unwrap();
    });


    match HidApi::new() {
        Ok(api) => {
            // make spot for device we are looking for
            let pcpanel: HidDevice = api.open(0x0483, 0xa3c4).unwrap();
            println!("Device: {:?}", pcpanel.get_device_info());

            // Testing program, update the colors and send them to the device for each mode

            // Testing static per-knob color
            panel_state.individual_led_data[0] =
                panel_state::ColorMode::StaticColor { r: 255, g: 0, b: 0 };

            panel_state.individual_led_data[1] =
                panel_state::ColorMode::StaticColor { r: 0, g: 255, b: 0 };

            panel_state.individual_led_data[2] =
                panel_state::ColorMode::StaticColor { r: 0, g: 0, b: 255 };

            panel_state.individual_led_data[3] = panel_state::ColorMode::StaticColor {
                r: 255,
                g: 255,
                b: 255,
            };

            panel_state.send_led_state(&pcpanel);
            println!("Static per-knob color");
            sleep(Duration::from_millis(2000));

            // Testing Horizontal Rainbow Wave
            panel_state.global_led_data.led_mode = panel_state::LedMode::LightAnimation;
            panel_state.global_led_data.animation_type = panel_state::AnimationType::HorizontalRainbowWave;
            panel_state.send_led_state(&pcpanel);
            println!("Horizontal Rainbow Wave");
            sleep(Duration::from_millis(2000));

            // Testing Vertical Rainbow Wave
            panel_state.global_led_data.led_mode = panel_state::LedMode::LightAnimation;
            panel_state.global_led_data.animation_type = panel_state::AnimationType::VerticalRainbowWave;
            panel_state.send_led_state(&pcpanel);
            println!("Vertical Rainbow Wave");
            sleep(Duration::from_millis(2000));

            // Testing Breath
            panel_state.global_led_data.led_mode = panel_state::LedMode::LightAnimation;
            panel_state.global_led_data.animation_type = panel_state::AnimationType::Breath;
            panel_state.send_led_state(&pcpanel);
            println!("Breath");
            sleep(Duration::from_millis(2000));

            // set back to custom knob
            panel_state.global_led_data.led_mode = panel_state::LedMode::CustomKnob;
            panel_state.send_led_state(&pcpanel);
        }
        Err(_) => todo!(),
    }
    pw_handle.join().unwrap();

    Ok(())
}

fn run_pipewire() -> Result<(), Box<dyn std::error::Error>> {
    let pw_main_loop = MainLoop::new()?;
    let pw_context = Context::new(&pw_main_loop)?;
    let pw_core = pw_context.connect(None)?;
    let pw_registry = pw_core.get_registry()?;

    let _listener = pw_registry
        .add_listener_local()
        .global(|global| println!("New global: {:?}", global))
        .register();

    pw_main_loop.run();

    Ok(())
}