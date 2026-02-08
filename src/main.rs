//! Entry point for NCOM Audax.

mod app;
mod canvas;
mod export;
mod input;
mod tools;
mod ui;

use app::AudaxApp;

fn main() {
    // Placeholder: initialize logging, panic hooks, and windowing here.
    let mut app = AudaxApp::new();
    app.run();
}
