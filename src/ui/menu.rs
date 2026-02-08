//! Menu scaffolding for the application shell.

pub struct MenuState {
    pub is_visible: bool,
}

impl MenuState {
    /// Create a new menu state.
    pub fn new() -> Self {
        Self { is_visible: true }
    }

    /// Update menu visibility and interactions.
    pub fn update(&mut self) {
        // TODO: implement menu interactions.
    }
}
