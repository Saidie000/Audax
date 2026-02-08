//! Layer model for multi-layer canvas workflows.

#[derive(Debug, Clone, Copy)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Add,
    Subtract,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub visible: bool,
    pub locked: bool,
    pub has_mask: bool,
    pub has_clipping_mask: bool,
}

impl Layer {
    /// Create a new layer with default properties.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            visible: true,
            locked: false,
            has_mask: false,
            has_clipping_mask: false,
        }
    }

    /// Placeholder for applying layer masks.
    pub fn apply_masks(&self) {
        // TODO: integrate mask and clipping mask logic.
    }
}
