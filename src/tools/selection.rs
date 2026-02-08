//! Selection tools for marquee, lasso, and magic selections.

pub struct SelectionTool {
    pub is_active: bool,
}

impl SelectionTool {
    /// Create a new selection tool.
    pub fn new() -> Self {
        Self { is_active: false }
    }

    /// Placeholder for selection update logic.
    pub fn update_selection(&mut self) {
        // TODO: implement selection logic.
    }
}
