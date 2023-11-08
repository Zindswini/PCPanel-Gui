# Protocol Notes:

## General Format:
`[Device Specifier, Device Command, Command Specific Instructions]`
1. Device Specifier [1 Byte]
    - `0x05` - Pro
    - `0x06` - Mini
2. Device Command [1 Byte]
    - `0x00` - Custom Slider
    - `0x01` - Custom Slider Label
    - `0x02` - Custom Knob
    - `0x03` - Custom Logo
3. Command Specific Instructions [7 Bytes] (Documented Below)

## Known Commands:
###  `0x02` Custom Knob:
[`Color Mode`, [`Data`]]

#### Color Modes:
- `0x01` - Static Color
    - When this command is received the LED will be set to the color specified in the commmand.
    - Data: [`Static_R`, `Static_G`, `Static_B`]
- `0x02` - Gradient Color
    - When this command is received the LED will be set to a value between start and end according to the current status of the potentiometer.
    - Data: [`Start_R`, `Start_G`, `Start_B`, `End_R`, `End_G`, `End_B`]

> Red->Blue Gradient: [`0x06`, `0x02`, `0x02`, `0xFF`, `0x00` `0x00`, `0x00` `0x00` `0xFF`]


### 