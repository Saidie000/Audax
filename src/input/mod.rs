//! Input handling for Surface Pen and Windows Ink.

mod palm_rejection;
mod surface_pen;
mod windows_ink;

pub use palm_rejection::{PalmRejectionConfig, PalmRejectionState};
pub use surface_pen::{SurfacePenEvent, SurfacePenState};

use crate::canvas::BrushInput;

pub struct InputManager {
    pen_state: SurfacePenState,
    palm_rejection: PalmRejectionState,
}

impl InputManager {
    /// Create a new input manager.
    pub fn new() -> Self {
        Self {
            pen_state: SurfacePenState::new(),
            palm_rejection: PalmRejectionState::new(),
        }
    }

    /// Poll input devices and convert to brush inputs.
    pub fn poll(&mut self) -> Option<BrushInput> {
        self.pen_state.poll_events();
        self.palm_rejection.pen_is_active = self.pen_state.latest_event().is_some();
        self.pen_state.latest_event().map(|event| BrushInput {
            position: event.position,
            pressure: event.pressure,
            tilt: event.tilt,
            buttons: event.buttons,
        })
    }

    /// Update palm rejection configuration.
    pub fn update_palm_rejection(&mut self, config: PalmRejectionConfig) {
        self.palm_rejection.update_config(config);
    }

    /// Decide whether a touch input should be rejected based on palm rejection.
    pub fn should_reject_touch(&self, contact_area: f32) -> bool {
        self.palm_rejection.should_reject_touch(contact_area)
    }
}
