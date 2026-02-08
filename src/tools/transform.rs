//! Transform tools for scale, rotate, and translate.

pub struct TransformTool {
    pub is_active: bool,
}

impl TransformTool {
    /// Create a new transform tool.
    pub fn new() -> Self {
        Self { is_active: false }
    }

    /// Placeholder for transform update logic.
    pub fn update_transform(&mut self) {
        // TODO: implement transform logic.
    }
}
