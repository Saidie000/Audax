//! Input handling for Surface Pen and Windows Ink.

mod surface_pen;
mod windows_ink;

pub use surface_pen::{SurfacePenEvent, SurfacePenState};

use crate::canvas::BrushInput;

pub struct InputManager {
    pen_state: SurfacePenState,
}

impl InputManager {
    /// Create a new input manager.
    pub fn new() -> Self {
        Self {
            pen_state: SurfacePenState::new(),
        }
    }

    /// Poll input devices and convert to brush inputs.
    pub fn poll(&mut self) -> Option<BrushInput> {
        self.pen_state.poll_events();
        self.pen_state.latest_event().map(|event| BrushInput {
            position: event.position,
            pressure: event.pressure,
            tilt: event.tilt,
            buttons: event.buttons,
        })
    }
}
