use std::ffi::{c_int, c_uint, c_ulong};
use x11::keysym::{XK_Down, XK_e, XK_Left, XK_q, XK_Return, XK_Right, XK_Up};
use x11::xlib::{ControlMask, CurrentTime, CWBorderWidth, CWHeight, CWWidth, CWX, CWY, Display, False, GrabModeAsync, LockMask, Mod1Mask, Mod2Mask, Mod4Mask, RevertToNone, ShiftMask, SubstructureNotifyMask, SubstructureRedirectMask, XCloseDisplay, XConfigureEvent, XConfigureRequestEvent, XConfigureWindow, XCreateWindowEvent, XDefaultScreen, XDestroyWindowEvent, XErrorEvent, XEvent, XGetWindowAttributes, XGrabKey, XKeyEvent, XKeymapEvent, XKeysymToKeycode, XKillClient, XMapEvent, XMappingEvent, XMapRequestEvent, XMapWindow, XNextEvent, XOpenDisplay, XReparentEvent, XRootWindow, XSelectInput, XSetErrorHandler, XSetInputFocus, XSetWindowBorder, XUnmapEvent, XWindowAttributes, XWindowChanges};
use crate::action::{Action};
use crate::config::{Config, Monitor};
use crate::keybind::Keybind;
use crate::layout::{Window, WindowTree};
use crate::util::Direction;

extern "C" fn custom_error_handler(_display: *mut Display, error_event: *mut XErrorEvent) -> c_int {
    println!("X11 Error occurred: {:?}", error_event);
    0
}

pub struct WindowManager {
    config: Config,
    display: *mut Display,
    root_window: c_ulong,
    keybinds: Vec<Keybind>,
    layout: WindowTree,
}

impl WindowManager {
    pub unsafe fn new() -> Self {
        let monitors = vec![
            Monitor::new(2560, 1440, 0, 0),
        ];
        let config = Config {
            monitors,
            ..Config::default()
        };
        assert!(config.monitors.len() > 0);
        println!("Config: {:?}", config);

        XSetErrorHandler(Some(custom_error_handler));

        let display = XOpenDisplay(std::ptr::null());
        if display.is_null() {
            eprintln!("Failed to open X display");
        }

        let screen = XDefaultScreen(display);
        let root_window = XRootWindow(display, screen);

        let keybinds = Vec::new();

        let tree = WindowTree::new(config.monitors[0].bounds.clone());

        Self {
            config,
            display,
            root_window,
            keybinds,
            layout: tree,
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
        self.register_keybind(XK_Left, Mod4Mask, Action::MoveFocus { direction: Direction::Left });
        self.register_keybind(XK_Right, Mod4Mask, Action::MoveFocus { direction: Direction::Right });
        self.register_keybind(XK_Up, Mod4Mask, Action::MoveFocus { direction: Direction::Up });
        self.register_keybind(XK_Down, Mod4Mask, Action::MoveFocus { direction: Direction::Down });
        self.register_keybind(XK_q, Mod4Mask | ShiftMask, Action::CloseFocusedWindow);
        self.register_keybind(XK_Left, Mod4Mask | Mod1Mask, Action::ChangeTilingDirection { direction: Direction::Left });
        self.register_keybind(XK_Right, Mod4Mask | Mod1Mask, Action::ChangeTilingDirection { direction: Direction::Right });
        self.register_keybind(XK_Up, Mod4Mask | Mod1Mask, Action::ChangeTilingDirection { direction: Direction::Up });
        self.register_keybind(XK_Down, Mod4Mask | Mod1Mask, Action::ChangeTilingDirection { direction: Direction::Down });

        loop {
            let mut event: XEvent = std::mem::zeroed();
            let result = XNextEvent(self.display, &mut event);
            //println!("Event received: type={}", event.get_type());
            if result != 0 {
                eprintln!("Error on XNextEvent: {}", result);
            }

            match event.get_type() {
                x11::xlib::CreateNotify => {
                    self.on_create_notify(event.create_window);
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
                x11::xlib::KeyRelease => {
                    // let event = event.key;
                    // println!("KeyRelease: {{ keycode: {}, state: {} }}", event.keycode, event.state);
                }
                _ => {
                    // let atom_value = 367;
                    // let atom_name_ptr = XGetAtomName(display, atom_value);
                    // let atom_name = std::ffi::CStr::from_ptr(atom_name_ptr).to_string_lossy();
                    // println!("Atom {} has name: {}", atom_value, atom_name);
                    // XFree(atom_name_ptr as *mut _);
                    if self.config.debug_events {
                        println!("Other: {:?}", event);
                    }
                }
            }
        }
    }

    fn on_create_notify(&self, event: XCreateWindowEvent) {
        if self.config.debug_events {
            println!("Create: {}", event.window);
        }
    }

    unsafe fn on_configure_request(&mut self, request: XConfigureRequestEvent) { // TODO: check if a configure_request stems from a new window
        if self.config.debug_events {
            println!("Configure Request: {}", request.window);
        }

        let changed = self.layout.insert(Window::new(request.window));

        let border_width = if let Some(border) = &self.config.border { border.width } else { 0 };
        let border_space = (border_width * 2) as c_int;

        // TODO: check if there's a problem with not copying sibling and stack_mode and including request.value_mask for newly created windows
        for (window_id, bounds) in changed {
            let mut changes = XWindowChanges {
                x: bounds.x,
                y: bounds.y,
                width: bounds.width - border_space,
                height: bounds.height - border_space,
                border_width,
                sibling: 0,
                stack_mode: 0,
            };
            XConfigureWindow(self.display, window_id, (CWX | CWY | CWWidth | CWHeight | if window_id == request.window { CWBorderWidth } else { 0 }) as c_uint, &mut changes);
            if window_id == request.window {
                if let Some(border) = &self.config.border {
                    XSetWindowBorder(self.display, window_id, border.color);
                }
            }
        }
    }

    fn on_configure_notify(&self, event: XConfigureEvent) {
        if self.config.debug_events {
            println!("Configure: {}", event.window);
        }
    }

    unsafe fn on_map_request(&self, request: XMapRequestEvent) {
        if self.config.debug_events {
            println!("Map Request: {}", request.window);
        }
        XMapWindow(self.display, request.window);
        XSetInputFocus(self.display, request.window, RevertToNone, CurrentTime);
    }

    fn on_map_notify(&self, event: XMapEvent) {
        if self.config.debug_events {
            println!("Map: {}", event.window);
        }
    }

    fn on_unmap_notify(&self, event: XUnmapEvent) {
        if self.config.debug_events {
            println!("Unmap: {}", event.window);
        }
    }

    fn on_destroy_notify(&self, event: XDestroyWindowEvent) {
        if self.config.debug_events {
            println!("Destroy: {}", event.window);
        }
    }

    fn on_reparent_notify(&self, event: XReparentEvent) {
        if self.config.debug_events {
            println!("Create: {}", event.window);
        }
    }

    fn on_keymap_notify(&self, event: XKeymapEvent) {
        if self.config.debug_events {
            println!("Keymap: {:?}", event);
        }
    }

    fn on_mapping_notify(&self, event: XMappingEvent) {
        if self.config.debug_events {
            println!("Mapping: {{ request: {}, first_keycode: {}, count: {} }}", event.request, event.first_keycode, event.count);
        }
    }

    fn on_keypress(&mut self, event: XKeyEvent) {
        if self.config.debug_events {
            println!("KeyPress: {{ keycode: {}, state: {} }}", event.keycode, event.state);
        }
        for keybind in &self.keybinds {
            if event.keycode == keybind.keycode && event.state & (ShiftMask | ControlMask | Mod1Mask | Mod4Mask) == keybind.modifiers {
                keybind.action.clone().execute(self); // TODO: probably find a better way to do this
                break;
            }
        }
    }

    pub unsafe fn move_focus(&mut self, direction: Direction) {
        let window_id = self.layout.move_focus(direction);
        if let Some(window_id) = window_id {
            XSetInputFocus(self.display, window_id, RevertToNone, CurrentTime);
        }
    }

    pub unsafe fn close_focused_window(&mut self) {
        if let Some(focused_id) = self.layout.get_focused_window_id() {
            XKillClient(self.display, focused_id);
            let (new_focus, changed) = self.layout.remove_focused_window();
            if let Some(focused_id) = new_focus {
                XSetInputFocus(self.display, focused_id, RevertToNone, CurrentTime);
            }
            for (window_id, bounds) in changed {
                let border_width = if let Some(border) = &self.config.border { border.width } else { 0 };
                let border_space = (border_width * 2) as c_int;

                let mut changes = XWindowChanges {
                    x: bounds.x,
                    y: bounds.y,
                    width: bounds.width - border_space,
                    height: bounds.height - border_space,
                    border_width: 0,
                    sibling: 0,
                    stack_mode: 0,
                };
                XConfigureWindow(self.display, window_id, (CWX | CWY | CWWidth | CWHeight) as c_uint, &mut changes);
            }
        }
    }

    pub fn change_tiling_direction(&mut self, direction: Direction) {
        self.layout.change_tiling_direction(direction);
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
