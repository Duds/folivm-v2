---
status: active
type: code
---

# folivm-v2

## Purpose

Folivm is a Rust-based document editor with WASM shell, featuring a complete
WYSIWYG text engine, orthogonal edge routing, and extension host.

## Stack

- **Core**: Rust (folivm-core, folivm-native, folivm-wasm)
- **Shell**: TypeScript/React with Canvas 2D
- **Build**: Cargo + wasm-pack

## Session Management

- **Start session**: run the `orient` skill — it reads `TASKS.md` and triages work
- **End session**: run the `close-and-learn` skill — captures lessons + handoff prompt

Keep `TASKS.md` at the project root. Orient reads it to pick up where you left off.
