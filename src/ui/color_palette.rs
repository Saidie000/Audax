//! Color palette, picker, and gradient scaffolding.

#[derive(Debug, Clone, Copy)]
pub struct GradientStop {
    pub position: f32,
    pub color: (f32, f32, f32, f32),
}

pub struct ColorPalette {
    pub colors: Vec<(f32, f32, f32, f32)>,
    pub gradient: Vec<GradientStop>,
}

impl ColorPalette {
    /// Create a new color palette.
    pub fn new() -> Self {
        Self {
            colors: vec![(0.0, 0.0, 0.0, 1.0), (1.0, 1.0, 1.0, 1.0)],
            gradient: vec![
                GradientStop {
                    position: 0.0,
                    color: (0.0, 0.0, 0.0, 1.0),
                },
                GradientStop {
                    position: 1.0,
                    color: (1.0, 1.0, 1.0, 1.0),
                },
            ],
        }
    }

    /// Placeholder for color picker interactions.
    pub fn pick_color(&self, _position: (f32, f32)) -> (f32, f32, f32, f32) {
        self.colors.first().copied().unwrap_or((0.0, 0.0, 0.0, 1.0))
    }
}
