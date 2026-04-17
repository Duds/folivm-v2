use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use crate::extensions::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum HookEvent {
    ApplicationReady,
    ApplicationQuit,
    DocumentOpened { id: String, title: String },
    DocumentClosed { id: String },
    DocumentSaved { id: String },
    ThemeChanged { name: String, mode: String },
}

pub type HookHandler = Box<dyn Fn(HookEvent) -> Result<()> + Send + Sync>;

#[derive(Default)]
pub struct HookBus {
    // Map of Extension ID -> List of handlers
    handlers: Arc<RwLock<std::collections::HashMap<String, Vec<HookHandler>>>>,
}

impl HookBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe(&self, extension_id: String, handler: HookHandler) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.entry(extension_id).or_default().push(handler);
    }

    pub fn unsubscribe(&self, extension_id: &str) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.remove(extension_id);
    }

    pub fn emit(&self, event: HookEvent) -> Result<()> {
        let handlers = self.handlers.read().unwrap();
        for extension_handlers in handlers.values() {
            for handler in extension_handlers {
                handler(event.clone())?;
            }
        }
        Ok(())
    }
}
