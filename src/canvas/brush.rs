//! Brush engine supporting pressure, tilt, dynamics, and texture.

#[derive(Debug, Clone, Copy)]
pub struct BrushSettings {
    pub base_size: f32,
    pub spacing: f32,
    pub jitter: f32,
    pub texture_strength: f32,
}

impl Default for BrushSettings {
    fn default() -> Self {
        Self {
            base_size: 12.0,
            spacing: 0.2,
            jitter: 0.0,
            texture_strength: 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BrushInput {
    pub position: (f32, f32),
    pub pressure: f32,
    pub tilt: (f32, f32),
    pub buttons: u32,
}

pub struct BrushEngine {
    settings: BrushSettings,
}

impl BrushEngine {
    /// Create a new brush engine with default settings.
    pub fn new() -> Self {
        Self {
            settings: BrushSettings::default(),
        }
    }

    /// Update brush settings from the brush editor.
    pub fn update_settings(&mut self, settings: BrushSettings) {
        self.settings = settings;
    }

    /// Apply input from the Surface Pen to generate brush strokes.
    pub fn apply_input(&mut self, input: BrushInput) {
        let _dynamic_size = self.settings.base_size * input.pressure.max(0.1);
        // TODO: generate stroke geometry using spacing, jitter, and texture.
    }
}
