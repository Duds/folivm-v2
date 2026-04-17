---
name: extension-host
description: Expert guidance on maintaining and extending the folivm-v2 extension host.
---

# Skill: Extension Host Development

Expert procedures for maintaining the `folivm-native` extension host and its skill/hook system.

## 1. Adding a New Skill Trait
When adding a new extension capability (e.g., `cell:ai` or `cell:diagram`):
1.  **Define Interface**: Create a new trait in `crates/folivm-native/src/extensions/skills/[skill_name].rs`.
2.  **Generic Export**: Add the submodule to `crates/folivm-native/src/extensions/skills/mod.rs`.
3.  **Agent Integration**: Add a registration field (`Vec<Arc<dyn NewSkill>>`) to the `ExtensionAgent` struct in `agent.rs`.
4.  **Registration Logic**: Update `ExtensionRegistry::load_extension` in `registry.rs` to detect and register the new skill from the extension manifest.

## 2. Emitting or Handling Hooks
- **Standard Events**: Check `crates/folivm-native/src/extensions/hooks.rs` for existing `HookEvent` variants.
- **Emitting**: Use `bus.emit(event)` where relevant in the native code (e.g., after successful file save).
- **Subscribing**: Extensions subscribe to hooks via the IPC layer (implemented in future phases).

## 3. Maintenance and Debugging
- **Compilation**: If `tauri::generate_context!` fails during testing due to missing icons, ensure `main.rs` is guarded with `#[cfg(not(test))]`.
- **Borrow Checker**: Skill traits are `dyn Skill + Send + Sync`. Use `Arc` and `RwLock` appropriately when managing state within the `ExtensionRegistry`.
