# Features
- Config file
- Moving windows
- Add floating mode
- Handle special windows such as docks and pop-ups correctly
- Add other dynamic tiling modes (e.g. master-slave)

# Fixes
- correct for integer division inaccuracies in bounds
- don't insert a window when a XConfigureRequest comes in, do so when it's mapped
- figure out where the XIO error comes from
