# Architecture

This program is split into three parts:
1. The UI
2. The HID Handler
3. The PipeWire Handler

When the program is run, the UI is the first process. The HID Handler and PipeWire handlers are then spawned as child processes.

Both the HID handler and PW handlers sit idle, wating for an event/signal.

## HID Loop
1. Event Received, require re-evaluation (HID Event, PW changed)
2. Decode change, update struct
3. Tell struct to re-check LEDs
4. Send HID event to update LEDs
5. Tell UI to update
6. Emit event to PipeWire (if not started by pw)

## PipeWire Loop
1. Event received (External volume change, other PW event)
2. Decode change, tell if applies to us
3. If yes, update struct
4. Tell struct to re-evaluate

Each knob has its own set of LED values, and so does the panel as a whole. Which is used to update the leds depends on the mode 