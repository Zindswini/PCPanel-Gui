# Protocol Notes:

## General Format:
`[Device Type, Command Type, Device Command, Command Specific Data]`
1. Device Type [1 Byte]
    - `0x05` - Pro
    - `0x06` - Mini
2. Command Type [1 Byte]
    - `0x00` - Custom Slider
    - `0x01` - Custom Slider Label
    - `0x02` - Custom Knob
    - `0x03` - Custom Logo
    - `0x04` - Light Animation
3. Command Specific Instructions [7 Bytes] (Documented Below)

## Known Commands:
###  `0x02` Custom Knob:
`[Color Mode, [Data]]`

#### Color Modes:
- `0x01` - Static Color
    - When this command is received the LED will be set to the color specified in the commmand.
    - Data: `[Static_R, Static_G, Static_B]`
- `0x02` - Gradient Color
    - When this command is received the LED will be set to a value between start and end according to the current status of the potentiometer.
    - Data: `[Start_R, Start_G, Start_B, End_R, End_G, End_B]`

> Red->Blue Gradient: `[0x06, 0x02, 0x02, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF]`


### `0x04` Universal Light Animation (Animate all LEDs)
`[Animation Selection, [Data]]`

#### Animations:
- `0x01` - Horizontal Rainbow Wave & `0x02` - Vertical Rainbow Wave
    - When either of these commands are received, all LEDs will begin an asynchronous rainbow animation.
    - Data: `[Phase Shift, Saturation, Brightness, Speed, Reverse (1 or 0)]`
- `0x03` - Horizontal Wave
    - When this command is received, all LEDs will begin an asynchronous wave animation, fading one color between knobs
    - Data: `[Hue, Saturation, Brightness, Speed, Reverse (1 or 0), Bounce at Ends (1 or 0)]`
- `0x04` - Breath
    - When this command is received, all LEDs will begin an asynchronous breath animation, fading one color in and out across all knobs.
    - Data: `[Hue, Saturation, Brightness, Speed]`

#### Data Format:
`[Hue, Saturation, Brightness, Speed, Reverse, Bounce]`
