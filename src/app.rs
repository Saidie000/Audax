//! Core application state and main loop orchestration.

use crate::canvas::Canvas;
use crate::input::InputManager;
use crate::tools::ToolManager;
use crate::ui::UiState;

pub struct AudaxApp {
    canvas: Canvas,
    input: InputManager,
    tools: ToolManager,
    ui: UiState,
}

impl AudaxApp {
    /// Create the application and initialize GPU resources.
    pub fn new() -> Self {
        let canvas = Canvas::new(1920, 1080);
        let input = InputManager::new();
        let tools = ToolManager::new();
        let ui = UiState::new();

        Self {
            canvas,
            input,
            tools,
            ui,
        }
    }

    /// Main loop placeholder. Integrate window event handling and rendering here.
    pub fn run(&mut self) {
        // Placeholder: wire up winit or another windowing system and drive the render loop.
        self.canvas.prepare_gpu();
        self.input.poll();
        self.tools.update();
        self.ui.update();
    }
}
