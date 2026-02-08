//! Windows Ink API integration via windows-rs.

use crate::input::SurfacePenEvent;

pub struct WindowsInkAdapter {
    #[cfg(windows)]
    ink_presenter: Option<windows::UI::Input::Inking::InkPresenter>,
}

impl WindowsInkAdapter {
    /// Create a new Windows Ink adapter.
    pub fn new() -> Self {
        Self {
            #[cfg(windows)]
            ink_presenter: None,
        }
    }

    /// Poll Windows Ink for pen events and translate to SurfacePenEvent.
    pub fn poll_pen_event(&mut self) -> Option<SurfacePenEvent> {
        #[cfg(windows)]
        {
            // TODO: wire up InkPresenter and PointerPoint events.
            let _ = &self.ink_presenter;
        }

        None
    }
}
