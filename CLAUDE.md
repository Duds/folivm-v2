# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Project state

Pre-implementation. The repository contains design documents only. No code exists yet. The next step is bootstrapping the Rust workspace and implementing `folivm-core`.

---

## Planned codebase structure

When implementation begins, the workspace layout will be:

```
folivm-v2/
├── Cargo.toml                  # workspace root
├── crates/
│   ├── folivm-core/            # shared Rust — DocumentModel, TextEngine, LayoutEngine, Renderer
│   ├── folivm-native/          # Tauri backend — file I/O, git, export, search, extension host
│   └── folivm-wasm/            # wasm-bindgen bindings — exposes FolvimInstance to the shell
├── shell/                      # TypeScript/React — canvas draw, event forwarding, Tauri IPC
│   ├── src/
│   │   ├── canvas.ts           # executes RenderInstruction stream from WASM
│   │   ├── ipc.ts              # Tauri command wrappers
│   │   └── ime.ts              # hidden <input> → composition events → WASM
│   └── index.html
└── docs/                       # all design documents (see below)
```

## Architecture — the one thing to internalise

**Rust is the complete application. The shell is a display device.**

- `folivm-core` compiles to both native (Tauri backend) and WASM32 (loaded by the shell). It owns the document model, text engine, layout engine, and renderer. It has zero JS dependencies.
- The shell (TypeScript/React) does three things only: execute Canvas 2D draw calls from the WASM `RenderInstruction` stream, forward raw keyboard/mouse events to WASM, and call Tauri IPC for system operations.
- There is no `contenteditable`. There is no ProseMirror, TipTap, or Slate. There is no client-side state management library (Zustand, Redux) for document state.
- `ts-rs` generates TypeScript types from Rust structs — Rust is the single source of truth for all shared types.

### WYSIWYG fidelity mechanism

`cosmic-text` is used for text layout in **both** the WASM editor (for canvas rendering) and the native export pipeline (for PDF generation). Both load the same font bytes. Line breaks and page breaks are numerically identical between the editor and the exported PDF. Do not introduce a second text layout path.

### Document model

All blocks carry stable UUIDs stored as `<!-- block:uuid -->` HTML comments in `.fvm` files. These must be preserved across parse/serialise round-trips. `parse(serialise(model)) == model` is a hard invariant.

### IPC shape

`folivm-native` exposes Tauri commands. The `document:read` command returns the raw `.fvm` string — parsing happens in `folivm-core`, not in the backend. The backend never interprets document content.

---

## Build commands

*(These are the planned commands — update this section as the workspace is bootstrapped.)*

```bash
# Build folivm-core for native
cargo build -p folivm-core

# Build folivm-wasm (requires wasm-pack)
wasm-pack build crates/folivm-wasm --target web

# Run the Tauri dev server
cargo tauri dev

# Run Rust tests
cargo test -p folivm-core

# Run a single test
cargo test -p folivm-core test_name

# Type-check the shell
cd shell && tsc --noEmit

# Lint the shell
cd shell && eslint src/
```

---

## Key invariants to enforce in code

- **No ad-hoc formatting.** The document model has no font, size, colour, or weight fields on blocks or inlines. All presentation is expressed through named styles from the active theme. Reject any change that adds raw formatting properties to `Block` or `Inline`.
- **Measurement unit is pt.** All layout calculations, margin values, ruler positions, and export measurements are in points. Never mix px into the layout path.
- **Single scale factor.** Design mode uses one scale factor: `canvas_width_px / page_width_pt × dpi/96 × zoom`. The ruler, canvas, and margin guides all derive from the same value.
- **Atomic writes.** PDF and DOCX are written to a temp file and renamed on completion. A failed export must not corrupt the previous output.
- **Extension isolation.** Extensions run in Deno Core isolates. They communicate through the structured `folivm.*` API only. Extensions cannot access `folivm-core` directly or call arbitrary Tauri commands.

---

## Design documents

Read these before making architectural decisions. They are authoritative.

| Document | Read when you are about to... |
|---|---|
| [docs/FORMAT.md](docs/FORMAT.md) | Touch the parser, serialiser, or any block/cell type |
| [docs/HLD.md](docs/HLD.md) | Change the boundary between core, native, WASM, or shell |
| [docs/SAD.md](docs/SAD.md) | Implement any part of the text engine, layout engine, or render system |
| [docs/SCOPE.md](docs/SCOPE.md) | Add a feature or decide what goes in v1.0 |
| [docs/EXT.md](docs/EXT.md) | Touch the extension host or any extension-facing API |
| [docs/UX.md](docs/UX.md) | Implement any interactive behaviour or keyboard shortcut |

---

## Scope discipline

SCOPE.md defines exactly what ships in v1.0. Before adding anything, check that document. Adding to scope requires naming an explicit deferral — scope is fixed, not a backlog. The phrase "that can be an extension" is not a reason to add a core capability.

Cell types with full v1.0 renderers: `cell:image`, `cell:math`, `cell:include`. All other cell types (`cell:data`, `cell:ai`, `cell:diagram`, `cell:citation`, `cell:signature`) render as labelled placeholders in core — their full rendering is extension territory.
