//! Shape helper tools such as snap-to-shape and QuickShape.

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum QuickShape {
    Rectangle,
    Ellipse,
    Line,
    Polygon,
}

pub struct SnapToShape {
    hold_duration: Duration,
    hold_start: Option<Instant>,
    pub active_shape: Option<QuickShape>,
}

impl SnapToShape {
    /// Create a new snap-to-shape helper.
    pub fn new() -> Self {
        Self {
            hold_duration: Duration::from_secs(2),
            hold_start: None,
            active_shape: None,
        }
    }

    /// Start tracking a pen hold for snap-to-shape.
    pub fn begin_hold(&mut self) {
        self.hold_start = Some(Instant::now());
    }

    /// Update snap-to-shape state and choose a shape if held long enough.
    pub fn update(&mut self) {
        if let Some(start) = self.hold_start {
            if start.elapsed() >= self.hold_duration {
                self.active_shape = Some(QuickShape::Rectangle);
                // TODO: infer actual shape from stroke geometry.
            }
        }
    }

    /// Reset snap-to-shape when pen is released.
    pub fn end_hold(&mut self) {
        self.hold_start = None;
    }
}
