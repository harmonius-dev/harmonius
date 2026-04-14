//! Last-used device routing hint used by glyph resolution and tutorials.

use crate::device::DeviceType;

/// Which device family last delivered meaningful input.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ActiveDeviceType {
    /// Keyboard / mouse desktop path.
    Keyboard,
    /// Gamepad-first path with optional layout family.
    Gamepad(crate::glyph::DeviceFamily),
}

impl ActiveDeviceType {
    /// Map from low-level device type after an input event.
    pub fn from_device_type(dt: DeviceType) -> Self {
        match dt {
            DeviceType::Keyboard | DeviceType::Mouse | DeviceType::Pen => {
                ActiveDeviceType::Keyboard
            }
            DeviceType::Gamepad => ActiveDeviceType::Gamepad(crate::glyph::DeviceFamily::Generic),
            DeviceType::Touch => ActiveDeviceType::Keyboard,
        }
    }
}
