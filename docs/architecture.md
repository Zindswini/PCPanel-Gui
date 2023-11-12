# Architecture

This program is split into three parts:
1. The UI
2. The HID Handler
3. The PipeWire Handler

When the program is run, the UI is the first process. The HID Handler and PipeWire handlers are then spawned as child processes.