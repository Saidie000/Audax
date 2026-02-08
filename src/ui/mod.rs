//! UI module for menus, palette, and brush editor.

mod menu;
mod color_palette;
mod brush_editor;
mod icon;

pub use brush_editor::{BrushEditorState, BrushPreset};
pub use color_palette::{ColorPalette, GradientStop};
pub use icon::AppIcon;
pub use menu::MenuState;

pub struct UiState {
    pub menu: MenuState,
    pub palette: ColorPalette,
    pub brush_editor: BrushEditorState,
    pub app_icon: AppIcon,
}

impl UiState {
    /// Create the UI state container.
    pub fn new() -> Self {
        Self {
            menu: MenuState::new(),
            palette: ColorPalette::new(),
            brush_editor: BrushEditorState::new(),
            app_icon: AppIcon::new("NCOM Audax", "assets/icons/app_icon.png"),
        }
    }

    /// Update UI state per-frame.
    pub fn update(&mut self) {
        self.menu.update();
    }
}
