mod platform;
pub use platform::SecureStorage;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
mod fallback;
