mod ops;

use std::sync::Arc;
use deno_core::{extension, JsRuntime, RuntimeOptions};
use crate::extensions::manifest::ExtensionManifest;
use crate::extensions::error::Result;

use crate::extensions::storage::StorageBackend;

extension!(
    folivm_core,
    ops = [
        ops::op_folivm_print, 
        ops::op_get_extension_id, 
        ops::op_get_extension_name,
        ops::op_document_get_metadata,
        ops::op_document_get_blocks,
        ops::op_cell_register_renderer,
        ops::op_storage_get,
        ops::op_storage_set,
        ops::op_storage_delete
    ],
);

pub struct ExtensionRuntime {
    pub manifest: ExtensionManifest,
    runtime: JsRuntime,
}

impl ExtensionRuntime {
    pub fn new(manifest: ExtensionManifest, storage: Arc<StorageBackend>) -> Result<Self> {
        let mut runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![folivm_core::init_ops()],
            ..Default::default()
        });
        
        // Initialize State with manifest data and storage for Ops to use
        {
            let state = runtime.op_state();
            let mut state = state.borrow_mut();
            state.put(manifest.id.clone());
            state.put((manifest.id.clone(), manifest.name.clone()));
            state.put(storage);
        }

        let mut this = Self {
            manifest,
            runtime,
        };

        // Load bootstrap script
        let bootstrap = include_str!("bootstrap.js");
        this.execute_script("folivm:bootstrap", bootstrap)?;
        
        Ok(this)
    }

    pub fn execute_script(&mut self, name: &'static str, source: &str) -> Result<()> {
        self.runtime.execute_script(name, source.to_string())?;
        Ok(())
    }
}
