//! Monitor information.

use crate::Size;

/// A monitor connected to the system, as the running shell reports it.
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorInfo {
    /// The name the platform gives the monitor (e.g. `"HDMI-A-1"`), when it
    /// has one. Only named monitors can be targeted by
    /// [`Settings::fullscreen_monitor`].
    ///
    /// [`Settings::fullscreen_monitor`]: crate::window::Settings::fullscreen_monitor
    pub name: Option<String>,
    /// The resolution in physical pixels.
    pub size: Size,
    /// Whether the system considers this the primary monitor.
    pub primary: bool,
}
