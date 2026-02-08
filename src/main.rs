//! Entry point for NCOM Audax.

mod app;
mod canvas;
mod export;
mod input;
mod package;
mod tools;
mod ui;

use app::AudaxApp;
use package::PackagingConfig;

fn main() {
    // Placeholder: initialize logging, panic hooks, and windowing here.
    let mut app = AudaxApp::new();
    let _packaging = PackagingConfig::new();
    app.run();
}
