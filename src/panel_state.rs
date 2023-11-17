use hidapi::HidDevice;

pub enum AnimationType {
    HorizontalRainbowWave, // 0x01
    VerticalRainbowWave,   // 0x02
    HorizontalWave,        // 0x03
    Breath,                // 0x04
}
impl AnimationType {
    // Convert from CommandType to command value
    pub fn to_byte(&self) -> u8 {
        match self {
            AnimationType::HorizontalRainbowWave => 0x01,
            AnimationType::VerticalRainbowWave => 0x02,
            AnimationType::HorizontalWave => 0x03,
            AnimationType::Breath => 0x04,
        }
    }
}

pub struct GlobalLedData {
    hue: u8,
    saturation: u8,
    brightness: u8,
    speed: u8,
    reverse: bool,
    bounce: Option<bool>, // Only for some animations
}
pub enum ColorMode {
    StaticColor {
        r: u8,
        g: u8,
        b: u8,
    }, // 0x01

    GradientColor {
        start_r: u8,
        start_g: u8,
        start_b: u8,
        end_r: u8,
        end_g: u8,
        end_b: u8,
    }, // 0x02
}
impl ColorMode {
    // Convert from CommandType to command value
    pub fn to_byte(&self) -> u8 {
        match self {
            ColorMode::StaticColor { r: _, g: _, b: _ } => 0x01,
            ColorMode::GradientColor {
                start_r: _,
                start_g: _,
                start_b: _,
                end_r: _,
                end_g: _,
                end_b: _,
            } => 0x02,
        }
    }
}
pub struct KnobState {
    value: u8,
    pressed: bool,
    pub custom_color_data: ColorMode,
}

// The overall LED mode of the panel
pub enum LedMode {
    CustomSlider,
    CustomSliderLabel,
    CustomKnob,
    CustomLogo,
    LightAnimation,
}

impl LedMode {
    // Convert from CommandType to command value
    pub fn to_byte(&self) -> u8 {
        match self {
            LedMode::CustomSlider => 0x00,
            LedMode::CustomSliderLabel => 0x01,
            LedMode::CustomKnob => 0x02,
            LedMode::CustomLogo => 0x03,
            LedMode::LightAnimation => 0x04,
        }
    }
}

pub enum DeviceType {
    Pro,  // 0x05
    Mini, // 0x06
}
impl DeviceType {
    // Convert from CommandType to command value
    pub fn to_byte(&self) -> u8 {
        match self {
            DeviceType::Pro => 0x05,
            DeviceType::Mini => 0x06,
        }
    }
}

pub struct PCPanel {
    device_type: DeviceType,
    pub knob_values: [KnobState; 4],
    pub led_mode: LedMode,
    pub global_led_data: GlobalLedData,
    pub animation_type: AnimationType,
}

impl PCPanel {
    // Default constructor, all leds static white
    pub fn new() -> PCPanel {
        PCPanel {
            device_type: DeviceType::Mini,
            knob_values: [
                KnobState {
                    value: 0,
                    pressed: false,
                    custom_color_data: ColorMode::StaticColor {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                },
                KnobState {
                    value: 0,
                    pressed: false,
                    custom_color_data: ColorMode::StaticColor {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                },
                KnobState {
                    value: 0,
                    pressed: false,
                    custom_color_data: ColorMode::StaticColor {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                },
                KnobState {
                    value: 0,
                    pressed: false,
                    custom_color_data: ColorMode::StaticColor {
                        r: 255,
                        g: 255,
                        b: 255,
                    },
                },
            ],
            led_mode: LedMode::CustomKnob,
            global_led_data: GlobalLedData {
                hue: 128,
                saturation: 255,
                brightness: 255,
                speed: 255,
                reverse: false,
                bounce: None,
            },
            animation_type: AnimationType::HorizontalRainbowWave,
        }
    }

    // Update the virtual state with new HID input
    pub fn update_state_hid(&mut self, input: [u8; 3]) {
        if input[0] == 1 {
            println!("Knob {} turned to {}", input[1], input[2]);
            self.knob_values[input[1] as usize].value = input[2];
        }
        if input[0] == 2 {
            println!("Knob {} changed. New state: {}", input[1], input[2]);
            self.knob_values[input[1] as usize].pressed = input[2] == 1;
        }
    }

    #[allow(unused)]
    // Update the virtual state with data from PipeWire
    fn update_state_pw() {}

    #[allow(unused)]
    // Update the virtual state with data from the UI
    fn update_state_ui() {}

    pub fn send_led_state(&mut self, pcpanel: &HidDevice) {
        // Send the virtual LED state to the device

        //let header: Vec<u8> = [self].to_vec();
        //let message: Vec<u8> = [0x02, 255, 0, 0, 0, 0, 255].to_vec();
        //let full_message: Vec<u8> = [header, message.clone()].concat();

        let mut message: Vec<u8> = Vec::new();

        // Attach common data (header)
        message.push(self.device_type.to_byte());
        message.push(self.led_mode.to_byte());

        // Attach specific data
        // Each data section must be 7 bytes long + 2 bytes for header
        match self.led_mode {
            LedMode::LightAnimation => {
                // Push animation type
                message.push(self.animation_type.to_byte());
                // For animation modes, attach global data
                message.push(self.global_led_data.hue);
                message.push(self.global_led_data.saturation);
                message.push(self.global_led_data.brightness);
                message.push(self.global_led_data.speed);
                message.push(self.global_led_data.reverse as u8);
                match self.global_led_data.bounce {
                    Some(b) => message.push(b as u8),
                    None => message.push(0),
                }
            }

            LedMode::CustomKnob => {
                for i in 0..self.knob_values.len() {
                    // Push custom knob mode
                    message.push(self.knob_values[i].custom_color_data.to_byte());
                    // Push custom color data
                    match self.knob_values[i].custom_color_data {
                        ColorMode::StaticColor { r, g, b } => {
                            message.push(r);
                            message.push(g);
                            message.push(b);
                            message.push(0);
                            message.push(0);
                            message.push(0);
                        }
                        ColorMode::GradientColor {
                            start_r,
                            start_g,
                            start_b,
                            end_r,
                            end_g,
                            end_b,
                        } => {
                            message.push(start_r);
                            message.push(start_g);
                            message.push(start_b);
                            message.push(end_r);
                            message.push(end_g);
                            message.push(end_b);
                        }
                    }
                }
            }

            _ => {
                println!("Unsupported mode encountered");
            }
        }

        // Write to HID
        pcpanel.write(&message).expect("msg failed to send");
    }
}
