//! Tooling system: selection, transform, shapes, arrows.

mod selection;
mod transform;
mod shape_helpers;
mod arrows;

pub use arrows::{ArrowEndpoint, SmartArrow};
pub use selection::SelectionTool;
pub use shape_helpers::{QuickShape, SnapToShape};
pub use transform::TransformTool;

pub struct ToolManager {
    pub selection: SelectionTool,
    pub transform: TransformTool,
    pub snap_to_shape: SnapToShape,
    pub arrows: SmartArrow,
}

impl ToolManager {
    /// Create a new tool manager with default tool states.
    pub fn new() -> Self {
        Self {
            selection: SelectionTool::new(),
            transform: TransformTool::new(),
            snap_to_shape: SnapToShape::new(),
            arrows: SmartArrow::new(),
        }
    }

    /// Update tool states per-frame.
    pub fn update(&mut self) {
        self.snap_to_shape.update();
    }
}
