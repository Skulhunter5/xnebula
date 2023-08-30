use std::ffi::{c_int, c_uint, c_ulong};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use x11::keysym::XK_e;
use x11::xlib::{ControlMask, Display, False, GrabModeAsync, LockMask, Mod1Mask, Mod2Mask, Mod4Mask, ShiftMask, SubstructureNotifyMask, SubstructureRedirectMask, XCloseDisplay, XConfigureWindow, XDefaultScreen, XEvent, XGetWindowAttributes, XGrabKey, XKeysymToKeycode, XMapWindow, XNextEvent, XOpenDisplay, XRootWindow, XSelectInput, XWindowAttributes, XWindowChanges};
use crate::action::Action;
use crate::keybind::Keybind;

pub struct WindowManager {
    display: *mut Display,
    screen: c_int,
    root_window: c_ulong,
    keybinds: Vec<Keybind>,
}

impl WindowManager {
    pub unsafe fn new() -> Self {
        let display = XOpenDisplay(std::ptr::null());
        if display.is_null() {
            eprintln!("Failed to open X display");
        }

        let screen = XDefaultScreen(display);
        let root_window = XRootWindow(display, screen);

        Self {
            display,
            screen,
            root_window,
            keybinds: Vec::new(),
        }
    }

    pub unsafe fn run(&mut self) {

        let mut running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();
        ctrlc::set_handler(move || {
            println!("Received Ctrl+C, cleaning up and exiting...");

            running_clone.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl+C handler");

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

        // let e_keycode = XKeysymToKeycode(self.display, XK_e as std::ffi::c_ulong) as std::ffi::c_int;
        // XGrabKey(self.display, e_keycode, Mod4Mask | Mod2Mask, root_window, False, GrabModeAsync, GrabModeAsync);

        self.register_keybind(XK_e, Mod4Mask, Action::Exit);

        loop {
            if !running.load(Ordering::SeqCst) {
                break;
            }

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
                    let mut configure_request = event.configure_request;
                    println!("Configure Request: {}", configure_request.window);
                    let mut changes = XWindowChanges {
                        x: configure_request.x,
                        y: configure_request.y,
                        width: configure_request.width,
                        height: configure_request.height,
                        border_width: configure_request.border_width,
                        sibling: configure_request.above,
                        stack_mode: configure_request.detail,
                    };
                    XConfigureWindow(self.display, configure_request.window, configure_request.value_mask as std::ffi::c_uint, &mut changes);
                }
                x11::xlib::ConfigureNotify => {
                    let configure_event = event.configure;
                    println!("Configure: {}", configure_event.window);
                }
                x11::xlib::MapRequest => {
                    let map_request = event.map_request;
                    println!("Map Request: {}", map_request.window);
                    XMapWindow(self.display, map_request.window);
                }
                x11::xlib::MapNotify => {
                    let map_event = event.map;
                    println!("Map: {}", map_event.window);
                }
                x11::xlib::UnmapNotify => {
                    let unmap_event = event.unmap;
                    println!("Unmap: {}", unmap_event.window);
                }
                x11::xlib::DestroyNotify => {
                    let destroy_event = event.destroy_window;
                    println!("Destroy: {}", destroy_event.window);
                }
                x11::xlib::ReparentNotify => {
                    let reparent_event = event.reparent;
                    println!("Create: {}", reparent_event.window);
                }
                x11::xlib::KeymapNotify => {
                    let keymap_event = event.keymap;
                    println!("Keymap: {:?}", keymap_event);
                }
                x11::xlib::KeyPress => {
                    let key_event = event.key;
                    println!("KeyPress: {{ keycode: {}, state: {} }}", key_event.keycode, key_event.state);
                    for keybind in &self.keybinds {
                        if key_event.keycode == keybind.keycode && key_event.state & (ShiftMask | ControlMask | Mod1Mask | Mod4Mask) == keybind.modifiers {
                            keybind.action.execute(&self);
                        }
                    }
                }
                x11::xlib::MappingNotify => {
                    let mapping_event = event.mapping;
                    println!("Mapping: {{ request: {}, first_keycode: {}, count: {} }}", mapping_event.request, mapping_event.first_keycode, mapping_event.count);
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
