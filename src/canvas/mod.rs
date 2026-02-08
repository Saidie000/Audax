//! Canvas module containing render state, GPU configuration, and layer stack.

mod layer;
mod brush;

pub use brush::{BrushEngine, BrushInput, BrushSettings};
pub use layer::{BlendMode, Layer};

use wgpu::Instance;

pub struct Canvas {
    width: u32,
    height: u32,
    resolution: f32,
    layers: Vec<Layer>,
    brush_engine: BrushEngine,
    is_infinite: bool,
}

impl Canvas {
    /// Create a new canvas with an initial layer stack.
    pub fn new(width: u32, height: u32) -> Self {
        let mut layers = Vec::new();
        layers.push(Layer::new("Background"));

        Self {
            width,
            height,
            resolution: 1.0,
            layers,
            brush_engine: BrushEngine::new(),
            is_infinite: false,
        }
    }

    /// Update canvas resolution and pixel density.
    pub fn set_resolution(&mut self, resolution: f32) {
        self.resolution = resolution;
    }

    /// Enable or disable infinite canvas mode.
    pub fn set_infinite(&mut self, enabled: bool) {
        self.is_infinite = enabled;
    }

    /// Handle brush input and route to the active layer.
    pub fn apply_brush_input(&mut self, input: BrushInput) {
        self.brush_engine.apply_input(input);
    }

    /// Configure GPU resources via wgpu and Vello.
    pub fn prepare_gpu(&self) {
        // Placeholder: create wgpu instance, adapter, device, queue, and Vello render context.
        let instance = Instance::default();
        let _adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }));
    }
}
