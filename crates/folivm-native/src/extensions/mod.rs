// Extension host backed by Deno Core.
//
// Responsibilities:
// - Load and isolate `.fvmext` extension packages
// - Expose the structured `folivm.*` API surface to extensions
// - Route extension events to the shell via Tauri events
// - Enforce permission manifests
//
//
// structured `folivm.*` API only. Extensions cannot access folivm-core
// directly or call arbitrary Tauri commands.

pub mod error;
pub mod manifest;
pub mod permissions;
pub mod hooks;
pub mod skills;
pub mod registry;
pub mod runtime;
pub mod storage;
pub mod agent;

#[cfg(test)]
pub mod tests;
