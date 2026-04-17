use std::sync::Arc;
use crate::extensions::storage::StorageBackend;
use deno_core::{op2, OpState};

#[op2(fast)]
pub fn op_folivm_print(#[string] msg: String) {
    println!("{}", msg);
}

#[op2]
#[string]
pub fn op_get_extension_id(state: &mut OpState) -> String {
    state.borrow::<String>().clone()
}

#[op2]
#[string]
pub fn op_get_extension_name(state: &mut OpState) -> String {
    state.borrow::<(String, String)>().1.clone()
}

#[op2]
#[serde]
pub fn op_document_get_metadata(_state: &mut OpState) -> serde_json::Value {
    // Mock for now, will pull from DocumentManager in P3
    serde_json::json!({
        "title": "Untitled Document",
        "author": "Folivm User"
    })
}

#[op2]
#[serde]
pub fn op_document_get_blocks(_state: &mut OpState) -> serde_json::Value {
    // Mock for now
    serde_json::json!([
        { "type": "paragraph", "content": "Welcome to Folivm." }
    ])
}

#[op2(fast)]
pub fn op_cell_register_renderer(_state: &mut OpState, #[string] cell_type: String) {
    // This op just notifies Rust that a JS renderer exists for this type.
    // In a real implementation, we'd store a V8 handle to the callback.
    println!("Extension registered renderer for: {}", cell_type);
}

#[op2]
#[string]
pub fn op_storage_get(state: &mut OpState, #[string] key: String) -> Option<String> {
    let storage = state.borrow::<Arc<StorageBackend>>();
    let extension_id = state.borrow::<String>();
    storage.get(extension_id, &key).ok().flatten()
}

#[op2(fast)]
pub fn op_storage_set(state: &mut OpState, #[string] key: String, #[string] value: String) {
    let storage = state.borrow::<Arc<StorageBackend>>();
    let extension_id = state.borrow::<String>();
    let _ = storage.set(extension_id, &key, &value);
}

#[op2(fast)]
pub fn op_storage_delete(state: &mut OpState, #[string] key: String) {
    let storage = state.borrow::<Arc<StorageBackend>>();
    let extension_id = state.borrow::<String>();
    let _ = storage.delete(extension_id, &key);
}
