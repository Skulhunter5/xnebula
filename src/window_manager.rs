use std::ffi::{c_int, c_uint, c_ulong};
use x11::keysym::{XK_e, XK_Return};
use x11::xlib::{ControlMask, CWBorderWidth, CWHeight, CWSibling, CWStackMode, CWWidth, CWX, CWY, Display, False, GrabModeAsync, LockMask, Mod1Mask, Mod2Mask, Mod4Mask, ShiftMask, SubstructureNotifyMask, SubstructureRedirectMask, XCloseDisplay, XConfigureEvent, XConfigureRequestEvent, XConfigureWindow, XDefaultScreen, XDestroyWindowEvent, XEvent, XGetGeometry, XGetWindowAttributes, XGrabKey, XHeightOfScreen, XKeyEvent, XKeymapEvent, XKeysymToKeycode, XMapEvent, XMappingEvent, XMapRequestEvent, XMapWindow, XNextEvent, XOpenDisplay, XReparentEvent, XRootWindow, XRootWindowOfScreen, XScreenCount, XScreenOfDisplay, XSelectInput, XUnmapEvent, XWidthOfScreen, XWindowAttributes, XWindowChanges};
use crate::action::Action;
use crate::keybind::Keybind;

pub struct WindowManager {
    display: *mut Display,
    screen: c_int,
    root_window: c_ulong,
    keybinds: Vec<Keybind>,
    monitors: Vec<(u32, u32, u32, u32)>,
}

impl WindowManager {
    pub unsafe fn new() -> Self {
        let display = XOpenDisplay(std::ptr::null());
        if display.is_null() {
            eprintln!("Failed to open X display");
        }

        let screen = XDefaultScreen(display);
        let root_window = XRootWindow(display, screen);

        let keybinds = Vec::new();

        let monitors = vec![(2560, 1440, 0, 0), (1080, 1920, 2560, 0)];

        Self {
            display,
            screen,
            root_window,
            keybinds,
            monitors,
        }
    }

    pub unsafe fn run(&mut self) {
        let mut window_attributes: XWindowAttributes = XWindowAttributes {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            border_width: 0,
            depth: 0,
            visual: std::ptr::null_mut(),
            root: 0,
            class: 0,
            bit_gravity: 0,
            win_gravity: 0,
            backing_store: 0,
            backing_planes: 0,
            backing_pixel: 0,
            save_under: 0,
            colormap: 0,
            map_installed: 0,
            map_state: 0,
            all_event_masks: 0,
            your_event_mask: 0,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            screen: std::ptr::null_mut(),
        };
        XGetWindowAttributes(self.display, self.root_window, &mut window_attributes);

        println!("Screen size: {}x{}", window_attributes.width, window_attributes.height);
        println!("Root window ID: {}", self.root_window);

        XSelectInput(self.display, self.root_window, SubstructureRedirectMask | SubstructureNotifyMask);

        self.register_keybind(XK_e, Mod4Mask, Action::Exit);
        self.register_keybind(XK_Return, Mod4Mask, Action::ExecuteCommand { command: "alacritty".to_string() });

        loop {
            let mut event: XEvent = std::mem::zeroed();
            let result = XNextEvent(self.display, &mut event);
            if result != 0 {
                eprintln!("Error on XNextEvent: {}", result);
            }

            match event.get_type() {
                x11::xlib::CreateNotify => {
                    let create_event = event.create_window;
                    println!("Create: {}", create_event.window);
                }
                x11::xlib::ConfigureRequest => {
                    self.on_configure_request(event.configure_request);
                }
                x11::xlib::ConfigureNotify => {
                    self.on_configure_notify(event.configure);
                }
                x11::xlib::MapRequest => {
                    self.on_map_request(event.map_request);
                }
                x11::xlib::MapNotify => {
                    self.on_map_notify(event.map);
                }
                x11::xlib::UnmapNotify => {
                    self.on_unmap_notify(event.unmap);
                }
                x11::xlib::DestroyNotify => {
                    self.on_destroy_notify(event.destroy_window);
                }
                x11::xlib::ReparentNotify => {
                    self.on_reparent_notify(event.reparent);
                }
                x11::xlib::KeymapNotify => {
                    self.on_keymap_notify(event.keymap);
                }
                x11::xlib::MappingNotify => {
                    self.on_mapping_notify(event.mapping);
                }
                x11::xlib::KeyPress => {
                    self.on_keypress(event.key);
                }
                _ => {
                    // let atom_value = 367;
                    // let atom_name_ptr = XGetAtomName(display, atom_value);
                    // let atom_name = std::ffi::CStr::from_ptr(atom_name_ptr).to_string_lossy();
                    // println!("Atom {} has name: {}", atom_value, atom_name);
                    // XFree(atom_name_ptr as *mut _);
                    println!("Other: {:?}", event);
                }
            }
        }

        self.exit();
    }

    unsafe fn on_configure_request(&self, request: XConfigureRequestEvent) {
        println!("Configure Request: {}", request.window);
        let mut changes = XWindowChanges {
            x: 0,
            y: 0,
            width: self.monitors[0].0 as c_int,
            height: self.monitors[0].1 as c_int,
            border_width: 0,
            sibling: request.above,
            stack_mode: request.detail,
        };
        XConfigureWindow(self.display, request.window, (request.value_mask as c_uint | (CWX | CWY | CWWidth | CWHeight) as c_uint), &mut changes);
    }

    unsafe fn on_configure_notify(&self, event: XConfigureEvent) {
        println!("Configure: {}", event.window);
    }

    unsafe fn on_map_request(&self, request: XMapRequestEvent) {
        println!("Map Request: {}", request.window);
        XMapWindow(self.display, request.window);
    }

    unsafe fn on_map_notify(&self, event: XMapEvent) {
        println!("Map: {}", event.window);
    }

    unsafe fn on_unmap_notify(&self, event: XUnmapEvent) {
        println!("Unmap: {}", event.window);
    }

    unsafe fn on_destroy_notify(&self, event: XDestroyWindowEvent) {
        println!("Destroy: {}", event.window);
    }

    unsafe fn on_reparent_notify(&self, event: XReparentEvent) {
        println!("Create: {}", event.window);
    }

    unsafe fn on_keymap_notify(&self, event: XKeymapEvent) {
        println!("Keymap: {:?}", event);
    }

    unsafe fn on_mapping_notify(&self, event: XMappingEvent) {
        println!("Mapping: {{ request: {}, first_keycode: {}, count: {} }}", event.request, event.first_keycode, event.count);
    }

    unsafe fn on_keypress(&self, event: XKeyEvent) {
        println!("KeyPress: {{ keycode: {}, state: {} }}", event.keycode, event.state);
        for keybind in &self.keybinds {
            if event.keycode == keybind.keycode && event.state & (ShiftMask | ControlMask | Mod1Mask | Mod4Mask) == keybind.modifiers {
                keybind.action.execute(&self);
            }
        }
    }

    unsafe fn register_keybind(&mut self, key: c_uint, modifiers: c_uint, action: Action) {
        let keycode = XKeysymToKeycode(self.display, key as c_ulong);
        XGrabKey(self.display, keycode as c_int, modifiers, self.root_window, False, GrabModeAsync, GrabModeAsync);
        XGrabKey(self.display, keycode as c_int, modifiers | Mod2Mask, self.root_window, False, GrabModeAsync, GrabModeAsync);
        XGrabKey(self.display, keycode as c_int, modifiers | LockMask, self.root_window, False, GrabModeAsync, GrabModeAsync);
        self.keybinds.push(Keybind::new(keycode as c_uint, modifiers, action));
    }

    pub unsafe fn close_connection(&self) {
        XCloseDisplay(self.display);
    }

    pub unsafe fn exit(&self) {
        self.close_connection();
    }
}
