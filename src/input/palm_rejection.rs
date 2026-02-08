//! Advanced palm rejection configuration and filtering.

#[derive(Debug, Clone, Copy)]
pub struct PalmRejectionConfig {
    pub enabled: bool,
    pub max_touch_contact_area: f32,
    pub reject_while_pen_active: bool,
}

impl Default for PalmRejectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_touch_contact_area: 12.0,
            reject_while_pen_active: true,
        }
    }
}

pub struct PalmRejectionState {
    config: PalmRejectionConfig,
    pub pen_is_active: bool,
}

impl PalmRejectionState {
    /// Create a new palm rejection state tracker.
    pub fn new() -> Self {
        Self {
            config: PalmRejectionConfig::default(),
            pen_is_active: false,
        }
    }

    /// Update the palm rejection configuration at runtime.
    pub fn update_config(&mut self, config: PalmRejectionConfig) {
        self.config = config;
    }

    /// Decide whether to reject a touch input based on configuration.
    pub fn should_reject_touch(&self, contact_area: f32) -> bool {
        if !self.config.enabled {
            return false;
        }

        if self.config.reject_while_pen_active && self.pen_is_active {
            return true;
        }

        contact_area >= self.config.max_touch_contact_area
    }
}
