//! Platform-native window backends.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
pub use linux::NativeWindow;
#[cfg(target_os = "macos")]
pub use macos::NativeWindow;

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!("harmonius_platform only supports Linux and macOS in this bootstrap");
