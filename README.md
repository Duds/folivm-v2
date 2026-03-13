# folivm

A professional word processor for people who write documents that matter.

folivm is a local-first, fully offline desktop application built on Tauri. It is not a web app in a frame. The word processor runs as compiled Rust — a `folivm-core` crate that targets both native and WASM, paired with a thin shell that owns only the canvas and the chrome.

---

## What it is

folivm is built for consultants, legal professionals, technical writers, academics, and proposal managers — people who produce structured, high-stakes documents and need a tool that matches the way they think, not the way a general-purpose word processor was designed in 1990.

The two defining properties:

**Semantic styling only.** There is no font picker, no colour picker, no manual bold button as a primary control. Every formatting decision is expressed through a named style from an active theme. The same document renders in your firm's brand or a client's brand by switching one mode selection.

**Keyboard-first, outline-first.** The default editing mode is an outliner. Design mode exists for layout review. Neither mode is read-only. The entire editing workflow — including style application, block operations, versioning, and export — is reachable without lifting your hands from the keyboard.

---

## Architecture

```
folivm-core (Rust)
├── Compiles to native (folivm-native — Tauri backend)
└── Compiles to WASM (folivm-wasm — loaded by the shell)

Shell (TypeScript / React)
├── Canvas 2D — rendering surface, receives RenderInstruction stream from WASM
├── Event forwarding — raw keyboard/mouse events sent to WASM
└── Tauri IPC — file I/O, git, export, search, extensions

folivm-native (Rust / Tauri)
├── File I/O and auto-save
├── git2-rs — version history, drafts, diff
├── cosmic-text (native) — export layout, identical to editor layout
├── printpdf — PDF/UA generation
├── docx-rs — DOCX generation
├── ripgrep — project-wide search
└── Deno Core — extension host (one isolate per extension)
```

The key fidelity guarantee: `folivm-core` uses `cosmic-text` for text layout in both the WASM editor and the native export pipeline. Both load the same font bytes. Editor line breaks and export line breaks are numerically identical — what you see is what you get in print.

---

## File formats

Documents use the `.fvm` format: UTF-8 plain text, YAML frontmatter, extended Markdown block syntax, typed cell fences, and stable block UUIDs stored as HTML comments. Files are human-readable, git-diffable, and round-trip stable.

Themes use the `.fvm-theme` format: a Figma-inspired token cascade with Primitives, Semantic, Brand, and Spacing collections. Modes (screen/print, client-A/client-B, normal/compact) switch live with no document reload.

Both formats are fully specified in [docs/FORMAT.md](docs/FORMAT.md).

---

## Documentation

| Document | Description |
|---|---|
| [docs/CONCEPT.md](docs/CONCEPT.md) | Product vision, personas, design principles |
| [docs/FR.md](docs/FR.md) | Functional requirements |
| [docs/NFR.md](docs/NFR.md) | Non-functional requirements (performance, accessibility) |
| [docs/HLD.md](docs/HLD.md) | High-level architecture |
| [docs/SAD.md](docs/SAD.md) | Software architecture — implementation spec |
| [docs/FORMAT.md](docs/FORMAT.md) | .fvm and .fvm-theme format specification |
| [docs/SCOPE.md](docs/SCOPE.md) | v1.0 scope — what ships and what does not |
| [docs/UX.md](docs/UX.md) | UX and interaction design |
| [docs/EXT.md](docs/EXT.md) | Extension API specification |

---

## Status

Pre-implementation. All design documents are complete. The codebase does not exist yet.

The next step is `folivm-core`: `DocumentModel` structs, the `.fvm` parser, and `EditOperation` apply/invert.

---

## Platforms

v1.0 targets:
- macOS (Apple Silicon + Intel, universal binary)
- Windows 10 / 11 (x86-64)
- Linux (x86-64, .deb + AppImage)

---

## License

TBD.
