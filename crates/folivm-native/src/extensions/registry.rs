use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::extensions::error::{ExtensionError, Result};
use crate::extensions::manifest::ExtensionManifest;
use crate::extensions::agent::ExtensionAgent;
use crate::extensions::hooks::HookBus;
use crate::extensions::storage::StorageBackend;

pub struct ExtensionRegistry {
    pub agents: Arc<Mutex<HashMap<String, Arc<ExtensionAgent>>>>,
    pub hooks: Arc<HookBus>,
    pub storage: Arc<StorageBackend>,
}

impl ExtensionRegistry {
    pub fn new(hooks: Arc<HookBus>, storage_path: &std::path::Path) -> Result<Self> {
        let storage = Arc::new(StorageBackend::new(storage_path)?);
        Ok(Self {
            agents: Arc::new(Mutex::new(HashMap::new())),
            hooks,
            storage,
        })
    }

    pub fn load_manifest(&self, json: &str) -> Result<Arc<ExtensionAgent>> {
        let manifest = ExtensionManifest::from_json(json)?;
        let agent = Arc::new(ExtensionAgent::new(manifest.clone(), self.hooks.clone()));
        
        {
            let mut agents = self.agents.lock().unwrap();
            if agents.contains_key(&manifest.id) {
                return Err(ExtensionError::Manifest(format!("Extension '{}' already loaded", manifest.id)));
            }

            // Initialize Runtime
            let runtime = crate::extensions::runtime::ExtensionRuntime::new(
                manifest.clone(),
                self.storage.clone()
            )?;
            {
                let mut agent_runtime = agent.runtime.lock().unwrap();
                *agent_runtime = Some(runtime);
            }

            agents.insert(manifest.id.clone(), agent.clone());
        }

        // Skill Registration Logic (Placeholder for P1)
        // In P2, this would involve scanning the manifest and the entry point.
        // For P1, we assume the agent is now 'Active' if it loaded successfully.
        {
            let mut state = agent.state.write().unwrap();
            *state = crate::extensions::agent::ExtensionState::Active;
        }
        
        Ok(agent)
    }

    pub fn get_agent(&self, id: &str) -> Option<Arc<ExtensionAgent>> {
        let agents = self.agents.lock().unwrap();
        agents.get(id).cloned()
    }

    pub fn unload(&self, id: &str) -> Result<()> {
        let mut agents = self.agents.lock().unwrap();
        if let Some(agent) = agents.remove(id) {
            // Transition state to Disabled
            let mut state = agent.state.write().unwrap();
            *state = crate::extensions::agent::ExtensionState::Disabled;
            
            // Unsubscribe from hooks
            self.hooks.unsubscribe(id);
            
            // In future, we would also kill the Deno isolate here.
            Ok(())
        } else {
            Err(ExtensionError::NotFound(id.into()))
        }
    }

    pub fn list_extensions(&self) -> Vec<ExtensionManifest> {
        let agents = self.agents.lock().unwrap();
        agents.values().map(|a| a.manifest.clone()).collect()
    }
}
