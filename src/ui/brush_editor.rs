//! Brush editor scaffolding inspired by professional tools.

use crate::canvas::BrushSettings;

pub struct BrushPreset {
    pub name: String,
    pub settings: BrushSettings,
}

pub struct BrushEditorState {
    pub presets: Vec<BrushPreset>,
    pub active_preset: Option<usize>,
}

impl BrushEditorState {
    /// Create the brush editor with default presets.
    pub fn new() -> Self {
        Self {
            presets: vec![BrushPreset {
                name: "Default Pencil".to_string(),
                settings: BrushSettings::default(),
            }],
            active_preset: Some(0),
        }
    }

    /// Update brush settings based on user input.
    pub fn update_settings(&mut self, settings: BrushSettings) {
        if let Some(index) = self.active_preset {
            if let Some(preset) = self.presets.get_mut(index) {
                preset.settings = settings;
            }
        }
    }
}
