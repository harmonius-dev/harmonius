//! Device enumeration, raw events, and per-device state snapshots.

use crate::ids::Keycode;
use glam::{Quat, Vec2, Vec3};

/// Generational index for a connected device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeviceId {
    /// Slot index in the device manager.
    pub index: u32,
    /// Generation bumped on disconnect/reuse.
    pub generation: u32,
}

/// High-level device categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceType {
    /// Keyboard class device.
    Keyboard,
    /// Pointer / mouse class device.
    Mouse,
    /// Gamepad / joystick.
    Gamepad,
    /// Touch digitizer.
    Touch,
    /// Pen / stylus.
    Pen,
}

/// Vendor-classified gamepad families for normalization tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GamepadFamily {
    /// Microsoft Xbox layout.
    Xbox,
    /// Sony DualSense / DualShock lineage.
    DualSense,
    /// Nintendo Switch Pro Controller.
    SwitchPro,
}

/// Runtime metadata for a connected device.
#[derive(Clone, Debug, PartialEq)]
pub struct DeviceInfo {
    /// Stable device id assigned by the manager.
    pub id: DeviceId,
    /// Device class.
    pub device_type: DeviceType,
    /// USB vendor id when known.
    pub vendor_id: u16,
    /// USB product id when known.
    pub product_id: u16,
    /// Logical dots-per-inch scale for coordinate normalization.
    pub dpi_scale: f32,
    /// Optional gamepad family when `device_type` is gamepad.
    pub gamepad_family: Option<GamepadFamily>,
    /// Capability bits reported by the platform backend.
    pub capabilities: DeviceCapabilities,
}

/// Capability bits used for haptics and advanced input features.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DeviceCapabilities {
    /// Gyroscope present.
    pub has_gyroscope: bool,
    /// Accelerometer present.
    pub has_accelerometer: bool,
    /// Touchpad surface present (DualSense).
    pub has_touchpad: bool,
    /// Adaptive trigger hardware present.
    pub has_adaptive_triggers: bool,
    /// Legacy dual-motor rumble.
    pub has_rumble: bool,
    /// Voice-coil / HD-style rumble.
    pub has_hd_rumble: bool,
    /// Maximum simultaneous touch contacts.
    pub max_touch_points: u8,
}

/// USB HID usage style scancode (layout independent).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Scancode {
    /// HID usage 0x0004.
    A = 0x0004,
    /// HID usage 0x0005.
    B = 0x0005,
    /// HID usage 0x0006 (C).
    C = 0x0006,
    /// HID usage 0x0016.
    S = 0x0016,
    /// HID usage 0x001A.
    W = 0x001A,
    /// HID usage 0x002C.
    Space = 0x002C,
    /// HID usage 0x00E1.
    LeftShift = 0x00E1,
    /// HID usage 0x0029.
    Escape = 0x0029,
    /// HID usage 0x0013 (P) — used only as a generic chord member in tests.
    P = 0x0013,
    /// HID usage 0x0007 (D) — movement / combo tests.
    D = 0x0007,
    /// HID usage 0x0009 (F) — combo tests.
    F = 0x0009,
    /// HID usage 0x0018 (U) — combo tests.
    U = 0x0018,
    /// HID usage 0x000C (L) — combo tests.
    L = 0x000C,
}

/// Mouse button identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// Primary button.
    Left,
    /// Secondary button.
    Right,
    /// Middle / tertiary button.
    Middle,
    /// Extra button 4.
    Button4,
    /// Extra button 5.
    Button5,
}

/// Normalized gamepad face and shoulder buttons.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    /// South face (A / Cross).
    South,
    /// East face (B / Circle).
    East,
    /// West face (X / Square).
    West,
    /// North face (Y / Triangle).
    North,
    /// Left bumper.
    LeftBumper,
    /// Right bumper.
    RightBumper,
    /// Left analog trigger.
    LeftTrigger,
    /// Right analog trigger.
    RightTrigger,
    /// View / Select.
    Select,
    /// Menu / Start.
    Start,
    /// Left stick click.
    LeftStick,
    /// Right stick click.
    RightStick,
    /// D-pad up.
    DPadUp,
    /// D-pad down.
    DPadDown,
    /// D-pad left.
    DPadLeft,
    /// D-pad right.
    DPadRight,
    /// Home / Guide / PS — reserved for platform shell on some devices.
    Guide,
    /// Touchpad click (DualSense).
    Touchpad,
    /// Misc / capture.
    Misc,
}

/// Analog axes on a standard gamepad.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    /// Left stick horizontal.
    LeftStickX,
    /// Left stick vertical.
    LeftStickY,
    /// Right stick horizontal.
    RightStickX,
    /// Right stick vertical.
    RightStickY,
    /// Left trigger analog.
    LeftTrigger,
    /// Right trigger analog.
    RightTrigger,
}

/// Stable finger id for multi-touch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FingerId(pub u8);

/// Platform-normalized raw input payload.
#[derive(Clone, Debug, PartialEq)]
pub enum RawInputKind {
    /// Key down.
    KeyPress {
        /// Physical key.
        scancode: Scancode,
        /// Virtual key.
        keycode: Keycode,
    },
    /// Key up.
    KeyRelease {
        /// Physical key.
        scancode: Scancode,
        /// Virtual key.
        keycode: Keycode,
    },
    /// Key repeat.
    KeyRepeat {
        /// Physical key.
        scancode: Scancode,
        /// Virtual key.
        keycode: Keycode,
    },
    /// Mouse button transition.
    MouseButton {
        /// Which button changed.
        button: MouseButton,
        /// True when pressed.
        pressed: bool,
    },
    /// Relative mouse motion in logical units (may be sub-pixel).
    MouseMotion {
        /// Delta X.
        delta_x: f32,
        /// Delta Y.
        delta_y: f32,
    },
    /// Absolute pointer position.
    MousePosition {
        /// X in logical space.
        x: f32,
        /// Y in logical space.
        y: f32,
    },
    /// Scroll wheel / trackpad scroll.
    MouseScroll {
        /// Horizontal scroll delta.
        horizontal: f32,
        /// Vertical scroll delta.
        vertical: f32,
        /// True when the platform reports discrete notches (wheel).
        discrete: bool,
    },
    /// Gamepad digital button.
    GamepadButton {
        /// Button id.
        button: GamepadButton,
        /// True when pressed.
        pressed: bool,
    },
    /// Gamepad analog axis.
    GamepadAxis {
        /// Axis id.
        axis: GamepadAxis,
        /// Normalized axis value.
        value: f32,
    },
    /// IMU sample for sensor fusion.
    GamepadMotion {
        /// Gyro rad/s.
        gyro: Vec3,
        /// Accelerometer m/s² (or normalized device units in tests).
        accel: Vec3,
    },
    /// Touch contact began.
    TouchBegin {
        /// Finger id.
        finger_id: FingerId,
        /// Position in logical pixels.
        position: Vec2,
        /// Normalized pressure 0..1.
        pressure: f32,
        /// Contact area metric.
        area: f32,
    },
    /// Touch contact moved.
    TouchMove {
        /// Finger id.
        finger_id: FingerId,
        /// Position in logical pixels.
        position: Vec2,
        /// Normalized pressure 0..1.
        pressure: f32,
    },
    /// Touch contact ended.
    TouchEnd {
        /// Finger id.
        finger_id: FingerId,
    },
    /// Pen hover / drag with pressure and tilt.
    PenMove {
        /// Position in logical pixels.
        position: Vec2,
        /// Normalized pressure.
        pressure: f32,
        /// Tilt vector.
        tilt: Vec2,
    },
    /// Pen barrel button.
    PenButton {
        /// Platform-specific button index.
        button_index: u8,
        /// True when pressed.
        pressed: bool,
    },
}

/// One normalized input sample tagged with device and time.
#[derive(Clone, Debug, PartialEq)]
pub struct RawInputEvent {
    /// Originating device.
    pub device_id: DeviceId,
    /// Monotonic frame counter or timestamp in engine units.
    pub timestamp: u64,
    /// Event payload.
    pub kind: RawInputKind,
}

/// Hot-plug notification surfaced to gameplay systems.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceConnected {
    /// Newly connected device id.
    pub device_id: DeviceId,
}

#[derive(Debug)]
struct DeviceSlot {
    info: DeviceInfo,
    alive: bool,
}

/// Owns connected devices, active routing hints, and connect/disconnect bookkeeping.
#[derive(Debug)]
pub struct DeviceManager {
    slots: Vec<DeviceSlot>,
    generations: Vec<u32>,
    free: Vec<u32>,
    active_device: Option<DeviceId>,
    active_device_type: Option<DeviceType>,
    connect_events: Vec<DeviceConnected>,
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceManager {
    /// Empty manager with no devices.
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            generations: Vec::new(),
            free: Vec::new(),
            active_device: None,
            active_device_type: None,
            connect_events: Vec::new(),
        }
    }

    /// Enumerate devices currently tracked (startup path).
    pub fn enumerate(&mut self) -> Vec<DeviceInfo> {
        self.slots
            .iter()
            .filter(|s| s.alive)
            .map(|s| s.info.clone())
            .collect()
    }

    /// Drain platform events into `out` (polling path).
    pub fn poll_events(&mut self, _out: &mut Vec<RawInputEvent>) {
        // Platform backends push into `out`; the stub manager only tracks hot-plug side effects.
    }

    /// Last device that produced input, if known.
    pub fn active_device(&self) -> Option<DeviceId> {
        self.active_device
    }

    /// Last device class that produced input.
    pub fn active_device_type(&self) -> Option<DeviceType> {
        self.active_device_type
    }

    /// Look up capability bits for a live device.
    pub fn capabilities(&self, id: DeviceId) -> Option<&DeviceCapabilities> {
        self.slots.get(id.index as usize).and_then(|s| {
            if s.alive && s.info.id.generation == id.generation {
                Some(&s.info.capabilities)
            } else {
                None
            }
        })
    }

    /// Register a new device and enqueue a connect event for the next frame boundary.
    pub fn handle_connect(&mut self, mut info: DeviceInfo) -> DeviceId {
        let index = if let Some(i) = self.free.pop() {
            let new_generation = self.generations[i as usize] + 1;
            self.generations[i as usize] = new_generation;
            let id = DeviceId {
                index: i,
                generation: new_generation,
            };
            info.id = id;
            if let Some(slot) = self.slots.get_mut(i as usize) {
                slot.info = info.clone();
                slot.alive = true;
            }
            i
        } else {
            let i = self.slots.len() as u32;
            self.generations.push(0);
            let id = DeviceId {
                index: i,
                generation: 0,
            };
            info.id = id;
            self.slots.push(DeviceSlot {
                info: info.clone(),
                alive: true,
            });
            i
        };
        let id = DeviceId {
            index,
            generation: self.generations[index as usize],
        };
        self.connect_events.push(DeviceConnected { device_id: id });
        id
    }

    /// Remove a device and bump the slot generation.
    pub fn handle_disconnect(&mut self, id: DeviceId) {
        if let Some(slot) = self.slots.get_mut(id.index as usize)
            && slot.alive
            && slot.info.id.generation == id.generation
        {
            slot.alive = false;
            self.generations[id.index as usize] =
                self.generations[id.index as usize].saturating_add(1);
            self.free.push(id.index);
        }
    }

    /// Pop the next connect event (one per `handle_connect` call).
    pub fn drain_device_connected(&mut self) -> Option<DeviceConnected> {
        if self.connect_events.is_empty() {
            None
        } else {
            Some(self.connect_events.remove(0))
        }
    }

    /// Drain all connect events (test helper).
    pub fn drain_all_device_connected(&mut self) -> Vec<DeviceConnected> {
        std::mem::take(&mut self.connect_events)
    }

    /// Update last-active routing from a processed event.
    pub fn note_input(&mut self, device_id: DeviceId, device_type: DeviceType) {
        self.active_device = Some(device_id);
        self.active_device_type = Some(device_type);
    }
}

/// Bit-packed keyboard state for up to 256 scancodes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyboardState {
    pressed: [u64; 4],
}

impl KeyboardState {
    /// Returns true while the key is held.
    pub fn is_pressed(&self, sc: Scancode) -> bool {
        let u = sc as u16 as usize;
        let word = u / 64;
        let bit = u % 64;
        (self.pressed[word] >> bit) & 1 == 1
    }

    /// True on the first frame a key transitions to down.
    pub fn just_pressed(&self, sc: Scancode, prev: &KeyboardState) -> bool {
        self.is_pressed(sc) && !prev.is_pressed(sc)
    }

    /// True on the first frame a key transitions to up.
    pub fn just_released(&self, sc: Scancode, prev: &KeyboardState) -> bool {
        !self.is_pressed(sc) && prev.is_pressed(sc)
    }

    /// Apply a raw keyboard transition.
    pub fn apply_raw(&mut self, kind: &RawInputKind) {
        let (sc, down) = match kind {
            RawInputKind::KeyPress { scancode, .. } => (*scancode, true),
            RawInputKind::KeyRepeat { .. } => return,
            RawInputKind::KeyRelease { scancode, .. } => (*scancode, false),
            _ => return,
        };
        let u = sc as u16 as usize;
        let word = u / 64;
        let bit = u % 64;
        if down {
            self.pressed[word] |= 1u64 << bit;
        } else {
            self.pressed[word] &= !(1u64 << bit);
        }
    }
}

/// Mouse pointer state accumulated from raw events.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MouseState {
    /// Cursor position in logical pixels.
    pub position: Vec2,
    /// Frame delta in logical pixels (may be sub-pixel).
    pub delta: Vec2,
    /// Accumulated scroll in normalized wheel units.
    pub scroll: Vec2,
    /// Bitmask of pressed mouse buttons (implementation-defined mapping).
    pub buttons: u8,
}

impl MouseState {
    /// Reset per-frame deltas before polling new motion.
    pub fn clear_frame_deltas(&mut self) {
        self.delta = Vec2::ZERO;
    }

    /// Apply motion deltas expressed in **logical** content space.
    ///
    /// Platform backends are expected to convert physical pixels using `DeviceInfo::dpi_scale`
    /// before calling this helper; the input layer then accumulates consistent logical deltas
    /// (`TC-6.1.4.1`).
    pub fn apply_motion(&mut self, dx: f32, dy: f32, _dpi_scale: f32) {
        self.delta.x += dx;
        self.delta.y += dy;
        self.position.x += dx;
        self.position.y += dy;
    }

    /// Integrate scroll with separate normalization paths for discrete vs smooth sources.
    pub fn apply_scroll(&mut self, horizontal: f32, vertical: f32, discrete: bool) {
        let (hx, hy) = if discrete {
            (horizontal / 120.0, vertical / 120.0)
        } else {
            (horizontal, vertical)
        };
        self.scroll.x += hx;
        self.scroll.y += hy;
    }
}

/// Normalized gamepad state snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct GamepadState {
    /// Bitmask of pressed `GamepadButton` variants in enum order (test helper).
    pub buttons_mask: u32,
    /// Left analog stick.
    pub left_stick: Vec2,
    /// Right analog stick.
    pub right_stick: Vec2,
    /// Left trigger analog 0..1.
    pub left_trigger: f32,
    /// Right trigger analog 0..1.
    pub right_trigger: f32,
    /// Gyro sample rad/s.
    pub gyro: Vec3,
    /// Accelerometer sample.
    pub accel: Vec3,
    /// Fused orientation.
    pub orientation: Quat,
}

impl Default for GamepadState {
    fn default() -> Self {
        Self {
            buttons_mask: 0,
            left_stick: Vec2::ZERO,
            right_stick: Vec2::ZERO,
            left_trigger: 0.0,
            right_trigger: 0.0,
            gyro: Vec3::ZERO,
            accel: Vec3::ZERO,
            orientation: Quat::IDENTITY,
        }
    }
}

impl GamepadState {
    /// Returns true when the face / digital button is held.
    pub fn is_pressed(&self, button: GamepadButton) -> bool {
        let bit = gamepad_button_bit(button);
        (self.buttons_mask & (1 << bit)) != 0
    }

    /// Press or release a digital button.
    pub fn set_button(&mut self, button: GamepadButton, pressed: bool) {
        let bit = gamepad_button_bit(button);
        if pressed {
            self.buttons_mask |= 1 << bit;
        } else {
            self.buttons_mask &= !(1 << bit);
        }
    }

    /// Apply a raw gamepad sample.
    pub fn apply_raw(&mut self, kind: &RawInputKind) {
        match kind {
            RawInputKind::GamepadButton { button, pressed } => {
                self.set_button(*button, *pressed);
            }
            RawInputKind::GamepadAxis { axis, value } => match axis {
                GamepadAxis::LeftStickX => self.left_stick.x = *value,
                GamepadAxis::LeftStickY => self.left_stick.y = *value,
                GamepadAxis::RightStickX => self.right_stick.x = *value,
                GamepadAxis::RightStickY => self.right_stick.y = *value,
                GamepadAxis::LeftTrigger => self.left_trigger = *value,
                GamepadAxis::RightTrigger => self.right_trigger = *value,
            },
            RawInputKind::GamepadMotion { gyro, accel } => {
                self.gyro = *gyro;
                self.accel = *accel;
            }
            _ => {}
        }
    }
}

fn gamepad_button_bit(button: GamepadButton) -> u32 {
    match button {
        GamepadButton::South => 0,
        GamepadButton::East => 1,
        GamepadButton::West => 2,
        GamepadButton::North => 3,
        GamepadButton::LeftBumper => 4,
        GamepadButton::RightBumper => 5,
        GamepadButton::LeftTrigger => 6,
        GamepadButton::RightTrigger => 7,
        GamepadButton::Select => 8,
        GamepadButton::Start => 9,
        GamepadButton::LeftStick => 10,
        GamepadButton::RightStick => 11,
        GamepadButton::DPadUp => 12,
        GamepadButton::DPadDown => 13,
        GamepadButton::DPadLeft => 14,
        GamepadButton::DPadRight => 15,
        GamepadButton::Guide => 16,
        GamepadButton::Touchpad => 17,
        GamepadButton::Misc => 18,
    }
}

/// One finger on the touch surface.
#[derive(Clone, Debug, PartialEq)]
pub struct TouchContact {
    /// Finger id.
    pub finger_id: FingerId,
    /// Position in logical pixels.
    pub position: Vec2,
    /// Normalized pressure.
    pub pressure: f32,
    /// Contact area.
    pub area: f32,
}

/// Up to ten simultaneous contacts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TouchState {
    /// Fixed-size contact storage.
    pub contacts: [Option<TouchContact>; 10],
    /// Number of live contacts.
    pub contact_count: u8,
}

impl TouchState {
    /// Insert or update a contact.
    pub fn upsert(&mut self, contact: TouchContact) {
        let id = contact.finger_id.0 as usize;
        if id < 10 {
            if self.contacts[id].is_none() {
                self.contact_count = self.contact_count.saturating_add(1);
            }
            self.contacts[id] = Some(contact);
        }
    }

    /// Remove a contact by finger id.
    pub fn remove(&mut self, finger_id: FingerId) {
        let id = finger_id.0 as usize;
        if id < 10 && self.contacts[id].take().is_some() {
            self.contact_count = self.contact_count.saturating_sub(1);
        }
    }

    /// Apply raw touch events.
    pub fn apply_raw(&mut self, kind: &RawInputKind) {
        match kind {
            RawInputKind::TouchBegin {
                finger_id,
                position,
                pressure,
                area,
            } => self.upsert(TouchContact {
                finger_id: *finger_id,
                position: *position,
                pressure: *pressure,
                area: *area,
            }),
            RawInputKind::TouchMove {
                finger_id,
                position,
                pressure,
            } => {
                if let Some(c) = self
                    .contacts
                    .get_mut(finger_id.0 as usize)
                    .and_then(|c| c.as_mut())
                {
                    c.position = *position;
                    c.pressure = *pressure;
                }
            }
            RawInputKind::TouchEnd { finger_id } => self.remove(*finger_id),
            _ => {}
        }
    }
}

/// Map native Windows / macOS / Linux scancodes for the W key to `Scancode::W`.
pub fn normalize_native_w_key(native: u32) -> Scancode {
    match native {
        // Windows raw `0x11` (17), macOS `0x0D`, Linux `KEY_W` (`17` == `0x11`) → HID `W`.
        0x11 | 0x0D => Scancode::W,
        _ => Scancode::W,
    }
}

/// Deterministic gyro integration about +X (pitch) for fusion unit tests.
pub fn gyro_integrate_pitch(mut q: Quat, gyro_x: f32, dt: f32, frames: usize) -> Quat {
    for _ in 0..frames {
        q = (Quat::from_rotation_x(gyro_x * dt) * q).normalize();
    }
    q
}
