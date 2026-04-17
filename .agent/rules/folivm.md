# Folivm-v2 Project Rules

Persistent constraints and standards for the `folivm-v2` project.

## Rust Development
- **Edition**: 2021.
- **Safety**: Prefer safe Rust. Use `unsafe` only for FFI or performance-critical sections with clear justification.
- **Testing**:
    - Every new feature must have corresponding unit tests.
    - Integration tests should be preferred for cross-crate functionality.
    - Native tests in `folivm-native` often require specialized setup (e.g., icons, Tauri context).

## Architecture Boundaries
- **Core Library (`folivm-core`)**:
    - Pure library. Must not depend on `tauri` or native UI components.
    - Responsible for parsing, serialization, and document models.
- **Native Host (`folivm-native`)**:
    - Tauri-based entry point.
    - Responsible for extension host management, filesystem access, and system integration.

## Extension Host Patterns
- **Skill Traits**: New extension capabilities must be defined as traits in `crates/folivm-native/src/extensions/skills/`.
- **Hooks**: Use the `HookBus` for event-driven logic rather than hardcoding side effects.
- **Permissions**: Always check permissions via `PermissionGuard` before executing privileged operations.
