//! Surface Pen input handling with Windows Ink integration.

use crate::input::windows_ink::WindowsInkAdapter;

#[derive(Debug, Clone, Copy)]
pub struct SurfacePenEvent {
    pub position: (f32, f32),
    pub pressure: f32,
    pub tilt: (f32, f32),
    pub buttons: u32,
}

pub struct SurfacePenState {
    latest: Option<SurfacePenEvent>,
    windows_ink: WindowsInkAdapter,
}

impl SurfacePenState {
    /// Create a new Surface Pen state tracker.
    pub fn new() -> Self {
        Self {
            latest: None,
            windows_ink: WindowsInkAdapter::new(),
        }
    }

    /// Poll Windows Ink for new events.
    pub fn poll_events(&mut self) {
        if let Some(event) = self.windows_ink.poll_pen_event() {
            self.latest = Some(event);
        }
    }

    /// Return the most recent pen event.
    pub fn latest_event(&self) -> Option<SurfacePenEvent> {
        self.latest
    }
}
