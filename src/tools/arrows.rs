//! Smart arrows inspired by Excalidraw.

#[derive(Debug, Clone, Copy)]
pub struct ArrowEndpoint {
    pub position: (f32, f32),
    pub attaches_to_shape: bool,
}

pub struct SmartArrow {
    pub start: Option<ArrowEndpoint>,
    pub end: Option<ArrowEndpoint>,
    pub is_active: bool,
}

impl SmartArrow {
    /// Create a new smart arrow tool.
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
            is_active: false,
        }
    }

    /// Begin an arrow stroke.
    pub fn begin(&mut self, position: (f32, f32)) {
        self.start = Some(ArrowEndpoint {
            position,
            attaches_to_shape: false,
        });
        self.is_active = true;
    }

    /// Update arrow endpoint and resolve attachment to shapes or text boxes.
    pub fn update_end(&mut self, position: (f32, f32)) {
        self.end = Some(ArrowEndpoint {
            position,
            attaches_to_shape: false,
        });
        // TODO: resolve attachment to nearby shapes or text boxes.
    }

    /// Finish the arrow stroke.
    pub fn finish(&mut self) {
        self.is_active = false;
    }
}
