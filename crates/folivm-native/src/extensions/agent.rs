use std::sync::{Arc, Mutex};
use crate::extensions::manifest::ExtensionManifest;
use crate::extensions::permissions::PermissionGuard;
use crate::extensions::hooks::HookBus;
use crate::extensions::skills::cell_render::CellRenderSkill;
use crate::extensions::skills::data_source::DataSourceSkill;
use crate::extensions::skills::export_hook::ExportHookSkill;
use crate::extensions::skills::panel::PanelSkill;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionState {
    Loaded,
    Active,
    Disabled,
    Error(String),
}

use crate::extensions::runtime::ExtensionRuntime;

pub struct ExtensionAgent {
    pub manifest: ExtensionManifest,
    pub permissions: PermissionGuard,
    pub hooks: Arc<HookBus>,
    pub state: std::sync::RwLock<ExtensionState>,
    pub runtime: Mutex<Option<ExtensionRuntime>>,
    
    // Skill registrations
    pub cell_renderers: Vec<Arc<dyn CellRenderSkill>>,
    pub data_sources: Vec<Arc<dyn DataSourceSkill>>,
    pub export_hooks: Vec<Arc<dyn ExportHookSkill>>,
    pub panel_handlers: Vec<Arc<dyn PanelSkill>>,
}

impl ExtensionAgent {
    pub fn new(manifest: ExtensionManifest, hooks: Arc<HookBus>) -> Self {
        let permissions = PermissionGuard::new(manifest.permissions.clone());
        Self {
            manifest,
            permissions,
            hooks,
            state: std::sync::RwLock::new(ExtensionState::Loaded),
            runtime: Mutex::new(None),
            cell_renderers: Vec::new(),
            data_sources: Vec::new(),
            export_hooks: Vec::new(),
            panel_handlers: Vec::new(),
        }
    }
}
