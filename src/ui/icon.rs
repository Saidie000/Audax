//! Application icon metadata and placeholder handling.

#[derive(Debug, Clone)]
pub struct AppIcon {
    pub name: String,
    pub description: String,
    pub asset_path: String,
}

impl AppIcon {
    /// Create a new icon descriptor for the app icon asset.
    pub fn new(name: impl Into<String>, asset_path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: "Primary app icon asset".to_string(),
            asset_path: asset_path.into(),
        }
    }
}
