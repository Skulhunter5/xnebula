# Features
- Config file
- Moving windows
- Add floating mode
- Add workspaces
- Handle special windows such as docks and pop-ups correctly
- Add other dynamic tiling modes (e.g. master-slave)
- Add multi-monitor support

# Fixes
- Don't insert a window when a XConfigureRequest comes in, do so when it's mapped
- Figure out where the XIO error comes from
- Keep working after closing the last window
- Handle windows that unmap or destroy themselves
