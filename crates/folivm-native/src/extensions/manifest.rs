use serde::{Deserialize, Serialize};
use crate::extensions::error::{ExtensionError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub min_folivm_version: String,
    pub entry: String,
    pub permissions: Permissions,
    #[serde(default)]
    pub panels: Vec<PanelManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Permissions {
    #[serde(default)]
    pub document_read: bool,
    #[serde(default)]
    pub document_write: bool,
    #[serde(default)]
    pub cell_render: Vec<String>,
    #[serde(default)]
    pub cell_export: Vec<String>,
    #[serde(default)]
    pub data_source: bool,
    #[serde(default)]
    pub panel: bool,
    #[serde(default)]
    pub export_hook: bool,
    #[serde(default)]
    pub library_contribute: bool,
    #[serde(default)]
    pub network: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelManifest {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub file: String,
}

impl ExtensionManifest {
    pub fn from_json(json: &str) -> Result<Self> {
        let manifest: Self = serde_json::from_str(json)?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<()> {
        if self.id.is_empty() {
            return Err(ExtensionError::Manifest("Extension 'id' is required".into()));
        }
        if self.entry.is_empty() {
            return Err(ExtensionError::Manifest("Extension 'entry' is required".into()));
        }
        Ok(())
    }
}
