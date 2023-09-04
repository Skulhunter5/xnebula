# xnebula
xnebula is a tiling window manager for X11 written in Rust

## State
xnebula is still in a very early stage and doesn't even implement very basic features like moving and resizing windows yet.

## Current plans
- Implement moving windows
- Implement a config file
- Handle special window types differently, such as docks

## Problems
- After closing the last window, the X server doesn't seem to send any more events, including keypresses. Therefore xnebula becomes unresponsive and can't be closed without killing the process from another TTY.
