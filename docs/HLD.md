# folivm — High Level Design
**Version:** 0.3
**Date:** 2026-03-12
**Status:** In review
**Depends on:** CONCEPT.md v0.4, FR.md v0.1, NFR.md v0.1

---

## 1. System Overview

folivm is a Tauri desktop application. Rust is the complete application. The WebView is a display terminal.

There are two Rust targets and one thin shell:

```
┌─────────────────────────────────────────────────────────────────────┐
│  OS (macOS / Windows / Linux)                                       │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  Tauri                                                       │   │
│  │                                                             │   │
│  │  ┌───────────────────────┐   IPC (file I/O, git, export)   │   │
│  │  │  Native Rust Process   │◄────────────────────────────┐  │   │
│  │  │                        │                             │  │   │
│  │  │  • File I/O            │────────────────────────────►│  │   │
│  │  │  • Git abstraction     │                             │  │   │
│  │  │  • Export pipeline     │  ┌──────────────────────────┴┐ │   │
│  │  │  • Search engine       │  │  WebView                   │ │   │
│  │  │  • Extension host      │  │                            │ │   │
│  │  └───────────────────────┘  │  ┌──────────────────────┐  │ │   │
│  │                              │  │  folivm-core.wasm     │  │ │   │
│  │  ┌───────────────────────┐  │  │  (Rust → WASM)        │  │ │   │
│  │  │  File System           │  │  │                       │  │ │   │
│  │  │  • .fvm documents      │  │  │  • DocumentModel      │  │ │   │
│  │  │  • .fvm-theme files    │  │  │  • TextEngine         │  │ │   │
│  │  │  • .git repository     │  │  │  • LayoutEngine       │  │ │   │
│  │  │  • assets/             │  │  │  • Renderer           │  │ │   │
│  │  │  • .folivm/config      │  │  └──────────┬───────────┘  │ │   │
│  │  └───────────────────────┘  │             │               │ │   │
│  │                              │  ┌──────────▼───────────┐  │ │   │
│  │                              │  │  Canvas (editor)      │  │ │   │
│  │                              │  └──────────────────────┘  │ │   │
│  │                              │  ┌──────────────────────┐  │ │   │
│  │                              │  │  Shell (chrome only)  │  │ │   │
│  │                              │  │  TypeScript / HTML    │  │ │   │
│  │                              │  └──────────────────────┘  │ │   │
│  │                              └───────────────────────────┘ │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

**The boundary:**
- The word processor (DocumentModel, TextEngine, LayoutEngine, Renderer) is `folivm-core`, compiled to WASM, running inside the WebView
- File I/O, git, export, search, and extension hosting run in the native Rust process
- The shell (TypeScript) draws what `folivm-core.wasm` instructs and forwards raw input events to it
- IPC crosses the boundary only for operations that require disk access or system services

---

## 2. WebView Shell

The shell is the chrome around the word processor. It is TypeScript and HTML. It contains no word processor logic.

### 2.1 Shell component tree

```
AppShell
├── TitleBar
│   ├── WindowControls
│   ├── MenuBar (File, Edit, Selection, View, Help)
│   └── PanelToggles
│
├── MainBody
│   ├── ActivityBar
│   │   └── ActivityItem[] (Explorer, Search, Versioning, Extensions, TOC)
│   │
│   ├── SidebarPanel
│   │   └── PanelRouter → ExplorerPanel | SearchPanel |
│   │                      VersioningPanel | ExtensionsPanel | TOCPanel
│   │
│   ├── EditorArea
│   │   ├── TabBar
│   │   ├── EditorToolbar
│   │   └── EditorCanvas    ← <canvas> element owned by folivm-core.wasm
│   │
│   └── RightRail
│       ├── StylesPanel
│       ├── PageSettingsPanel
│       ├── HeaderFooterPanel
│       └── VariablesPanel
│
└── StatusBar
```

### 2.2 Shell state

The shell owns only display state. It holds no document content.

```
Shell state (TypeScript)
├── activeProjectPath
├── openTabs: Tab[]          ← metadata: path, title, dirty indicator
├── activeTabId
├── activeView               ← which sidebar panel is visible
├── showLeftSidebar
├── showRightRail
├── railTab: 'draft' | 'design'
└── zoomLevel
```

### 2.3 Shell responsibilities

```
Shell does:
├── Render application chrome (panels, toolbar, sidebar, status bar)
├── Capture raw input events → forward to folivm-core.wasm
├── Receive RenderInstructions from folivm-core.wasm → draw to Canvas
├── Manage IME hidden input → forward composition events to wasm
├── Maintain ARIA shadow tree → update from wasm accessibility events
├── Call Tauri IPC → for file operations, git, export, search
└── Listen to Tauri events → forward updates to wasm

Shell does not:
├── Know what a Block is
├── Know what a style is
├── Know where the cursor is
├── Know whether the document is in Outline or Design mode
└── Make any decision about document content or layout
```

---

## 3. folivm-core — The Word Processor

`folivm-core` is a Rust crate that compiles to both native and WASM. It is the word processor. When running in the WebView it operates as a WASM module — same code, different target.

### 3.1 Crate structure

```
crates/folivm-core/
├── model/
│   ├── document.rs       ← DocumentModel, Block, Inline
│   ├── frontmatter.rs    ← Frontmatter
│   └── operations.rs     ← EditOperation, apply(), invert()
│
├── text_engine/
│   ├── input.rs          ← InputHandler: key/mouse/composition events → EditOperation
│   ├── buffer.rs         ← RunBuffer: Vec<Run>, insert/delete/apply_style
│   ├── cursor.rs         ← CursorManager: ModelPosition, movement, sync
│   ├── selection.rs      ← SelectionManager: anchor/focus, cross-block
│   └── undo.rs           ← UndoStack: operations + inverses, coalescing
│
├── layout/
│   ├── engine.rs         ← LayoutEngine: wraps cosmic-text, produces laid-out lines
│   ├── page.rs           ← PageLayout: pagination, header/footer, margins
│   └── scale.rs          ← ScaleFactor: pt → canvas px conversion
│
└── render/
    ├── instructions.rs   ← RenderInstruction enum
    ├── frame.rs          ← FrameRenderer: full frame from DocumentModel + layout
    ├── delta.rs          ← DeltaRenderer: minimal update after single operation
    └── accessibility.rs  ← AccessibilityNode tree generation
```

### 3.2 The document model

The `DocumentModel` is the authoritative in-memory state of the open document. It lives in `folivm-core` — accessible to both the WASM module (for editing) and the native process (for I/O and export).

```rust
struct DocumentModel {
    path: PathBuf,
    frontmatter: Frontmatter,
    blocks: Vec<Block>,
    fvm_version: String,
}

enum Block {
    Paragraph  { id: BlockId, style: String, inlines: Vec<Inline> },
    Heading    { id: BlockId, level: u8, style: String, inlines: Vec<Inline> },
    List       { id: BlockId, style: String, items: Vec<ListItem>, ordered: bool },
    Blockquote { id: BlockId, style: String, blocks: Vec<Block> },
    Table      { id: BlockId, style: String, rows: Vec<TableRow> },
    Section    { id: BlockId, attrs: SectionAttrs, blocks: Vec<Block> },
    Callout    { id: BlockId, style: String, blocks: Vec<Block> },
    Cell(CellBlock),
}

enum Inline {
    Text   (String),
    Styled { style: String, inlines: Vec<Inline> },
    Token  { path: String },
}

enum EditOperation {
    Insert    { block_id: BlockId, offset: usize, text: String },
    Delete    { block_id: BlockId, range: Range<usize> },
    Split     { block_id: BlockId, offset: usize },
    Merge     { block_a: BlockId, block_b: BlockId },
    SetBlock  { block_id: BlockId, style: String },
    SetInline { block_id: BlockId, range: Range<usize>, style: Option<String> },
    InsertCell{ position: BlockPosition, cell: CellBlock },
    SetCell   { block_id: BlockId, key: String, value: Value },
}
```

### 3.3 The text engine

The text engine processes all editing operations within `folivm-core`. It is Rust. It operates on the `DocumentModel` directly.

```
User input (raw event from shell)
    ↓
InputHandler
    maps: keydown / beforeinput / mouse → EditOperation
    ↓
RunBuffer (active block in-memory text, Vec<Run>)
    applies: Insert, Delete, SetInline (text-level ops, no IPC)
    ↓
CursorManager
    advances: ModelPosition {block_id, offset}
    ↓
LayoutEngine
    reflows: affected block(s) via cosmic-text
    ↓
DeltaRenderer
    produces: minimal RenderInstruction set
    ↓
Shell canvas.ts
    executes: Canvas 2D draw calls
```

### 3.4 The layout engine

`cosmic-text` provides: Unicode shaping, bidirectional text, font fallback, line breaking, and text metrics. The layout engine wraps it to produce:

- **Line layout**: which runs appear on which line, at which pixel positions
- **Cursor geometry**: the pixel rectangle for a given model offset
- **Selection geometry**: the set of pixel rectangles for a selection range
- **Page layout** (Design mode): which blocks appear on which page, accounting for margins, headers, and footers

The scale factor `pt → canvas px` is owned by the layout engine:

```
scale = (canvas_width_px / page_width_pt) × (screen_dpi / 96) × zoom
```

This is the same formula as before, but it now lives entirely in Rust. The shell sends `canvas_width_px` on resize and `screen_dpi` on display change. The WASM module holds and applies the scale.

### 3.5 The render instruction system

The WASM module communicates with the shell's Canvas renderer via a typed instruction stream. Instructions are serialised to a compact binary buffer (postcard encoding) and passed to JS via a shared WASM memory view — zero copy.

```rust
pub enum RenderInstruction {
    FillRect   { x: f32, y: f32, w: f32, h: f32, color: u32 },
    StrokeRect { x: f32, y: f32, w: f32, h: f32, color: u32, width: f32 },
    DrawGlyph  { x: f32, y: f32, glyph_id: u32, font_id: u8, size: f32, color: u32 },
    DrawImage  { x: f32, y: f32, w: f32, h: f32, image_id: u32 },
    Cursor     { x: f32, y: f32, height: f32, color: u32 },
    Selection  { rects: Vec<Rect>, color: u32 },
    ClipPush   { x: f32, y: f32, w: f32, h: f32 },
    ClipPop,
}
```

The shell's `canvas.ts` executes each instruction against the Canvas 2D context. It holds a glyph cache and image cache keyed by ID. It does not interpret instructions — it executes them.

---

## 4. Native Backend (Rust / Tauri)

### 4.1 Module map

```
crates/folivm-native/
│
├── commands/              ← Tauri command handlers (IPC entry points)
│   ├── project.rs
│   ├── document.rs        ← read_document, save_document, save_version
│   ├── theme.rs
│   ├── search.rs
│   ├── export.rs
│   ├── versioning.rs
│   ├── library.rs
│   └── extensions.rs
│
├── parser/                ← .fvm ↔ DocumentModel (uses folivm-core types)
│   ├── frontmatter.rs
│   ├── body.rs
│   ├── cell.rs
│   └── serializer.rs
│
├── theme/
│   ├── parser.rs
│   ├── resolver.rs
│   └── validator.rs
│
├── export/
│   ├── resolver.rs
│   ├── pdf.rs
│   ├── docx.rs
│   └── accessibility.rs
│
├── git/
│   ├── repo.rs
│   ├── commit.rs
│   ├── branch.rs
│   ├── diff.rs
│   └── blame.rs
│
├── search/
│   ├── engine.rs
│   └── replace.rs
│
├── library/
│   ├── index.rs
│   ├── versioning.rs
│   └── resolver.rs
│
├── extensions/
│   ├── host.rs
│   ├── sandbox.rs         ← Deno Core
│   ├── api.rs
│   └── marketplace.rs
│
└── watcher/
    └── watcher.rs
```

The native backend uses `folivm-core` types directly — `DocumentModel`, `Block`, `EditOperation`. There is no type translation between the native process and the WASM module. They share the same Rust type definitions.

### 4.2 IPC command surface

**Commands (shell calls native):**

| Command | Input | Output |
|---|---|---|
| `project:open` | `path: String` | `ProjectMeta` |
| `project:create` | `name, path: String` | `ProjectMeta` |
| `document:read` | `path: String` | `String` (.fvm file content) |
| `document:save` | `path: String, content: String` | `()` |
| `document:save_version` | `path, message: String` | `VersionEntry` |
| `theme:read` | `path: String` | `String` (.fvm-theme file content) |
| `search:query` | `project_path, query: String, opts: SearchOpts` | `Stream<SearchResult>` |
| `search:replace` | `project_path, query, replacement: String, opts` | `ReplaceResult` |
| `export:pdf` | `doc_path, opts: ExportOpts` | `Stream<ExportProgress>` |
| `export:docx` | `doc_path, opts: ExportOpts` | `Stream<ExportProgress>` |
| `versioning:list` | `project_path: String` | `Vec<VersionEntry>` |
| `versioning:diff` | `project_path, from, to: String` | `TrackChangesDiff` |
| `versioning:create_draft` | `project_path, name: String` | `()` |
| `library:list` | `project_path: String` | `Vec<LibraryItem>` |
| `library:resolve` | `project_path, ref_id, version: String` | `String` (.fvm fragment) |

Note: `document:read` returns the raw `.fvm` string. Parsing to `DocumentModel` happens inside `folivm-core.wasm` in the WebView — the native process is not involved in the parse for display purposes. The native process parses independently for export and git operations.

**Events (native pushes to shell):**

| Event | Payload | Trigger |
|---|---|---|
| `file:changed` | `path: String` | External file modification detected |
| `theme:updated` | `path: String` | Theme file changed on disk |
| `dpi:changed` | `dpi: f32` | Window moved to different display |
| `export:progress` | `percent: f32, stage: String` | Export pipeline progress |
| `search:result` | `SearchResult` | Streaming search result |
| `extension:event` | `ext_id, event_name, payload` | Extension emits to shell |

---

## 5. Export Pipeline

Export runs in the native Rust process using `folivm-core` types. The WASM module is not involved in export. WebKit is not involved in export.

```
.fvm file (disk)
    │  native parser (folivm-core)
    ▼
DocumentModel
    │
    ▼
[Stage 1: Resolution]
ExportDocument  (tokens resolved, cell:include inlined,
                 cell:data populated, theme applied)
    │
    ▼
[Stage 2: Extension hooks]
    (registered export hooks called in order)
    │
    ▼
[Stage 3: Layout — cosmic-text (native)]
    same engine as the editor
    produces positioned glyphs, line breaks, page breaks
    font data identical to what the WASM module loaded
    │
    ├──► PDF
    │    • positioned glyphs → printpdf draw calls
    │    • fonts embedded as subsets
    │    • PDF/UA structure tags written directly
    │    • icons rendered as embedded SVG paths
    │    → output: .pdf
    │
    └──► DOCX
         • ExportDocument → docx-rs object
         • Named styles → DOCX paragraph/character styles
         • Headings → DOCX Heading 1–6
         • cell:math → OMML
         • cell:diagram → embedded SVG
         → output: .docx
```

**The fidelity guarantee:** cosmic-text runs in two contexts from the same source crate — as WASM in the editor, as native in the export pipeline — with identical font bytes. Line breaks, character positions, and page breaks computed by the editor are identical to those in the PDF. What the editor shows is what the PDF contains. No reconciliation, no approximation.

---

## 6. Extension Architecture

Extensions run in Deno Core sandboxes in the native process. They receive and return `folivm-core` types via the extension API. Extension UI panels are HTML fragments rendered in isolated iframes in the shell — they do not render to the editor canvas.

```
Extension Package (.fvmext)
├── manifest.yaml
├── index.ts              ← runs in Deno Core (native process)
└── panel.html            ← optional: sidebar panel UI (runs in iframe in shell)
```

Extension lifecycle, API surface, and isolation model are unchanged from HLD v0.2. See SAD for implementation detail.

---

## 7. Git Abstraction Layer

Unchanged from HLD v0.2. git2-rs in the native process. Document-language UX mapped to git operations. Author attribution preserved.

---

## 8. Search Architecture

Unchanged from HLD v0.2. Streaming ripgrep over `.fvm` files in the native process. Results streamed to shell via Tauri events. Replace pipeline writes atomically and commits.

---

## 9. Content Library Architecture

Unchanged from HLD v0.2. `.folivm/library/` directory structure. `cell:include` resolution via `LibraryResolver` in the native process. Version pinning, update-on-explicit-action.

---

## 10. Key Architectural Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Desktop framework | Tauri | Native performance, small binary, OS integration, offline-first |
| Application language | Rust (complete) | One language, one type system, one compiler. The word processor is not a web application |
| WASM module | folivm-core compiled to wasm32 | Same Rust codebase runs in WebView for text editing — zero IPC latency for keystrokes |
| Text layout | cosmic-text | Production Rust text layout: Unicode shaping, BiDi, font fallback, line breaking. Owned |
| Rendering surface | Canvas 2D | No contenteditable. No browser default text behaviour. Rust controls all rendering |
| Shell language | TypeScript (minimal) | Chrome-only: draw calls, event forwarding, Tauri IPC. No word processor logic |
| IME | Hidden input element | Standard approach for canvas editors. Composition events forwarded to WASM |
| Accessibility | Explicit ARIA shadow tree | Canvas has no implicit accessibility. We build what the document actually is |
| Extension runtime | Deno Core (native process) | JS/TS authoring, Rust-enforced sandboxing, offline |
| PDF export | printpdf + cosmic-text (native) | Same layout engine as editor — identical line breaks, identical pagination. True WYSIWYG. No WebKit in export path |
| DOCX export | docx-rs | Pure Rust, no external dependency |
| Git operations | git2-rs | Embedded libgit2, no git binary dependency |
| Search | ripgrep crate | Fast streaming text search on ASCII .fvm files |
| Render instructions | Postcard binary via WASM memory | Zero-copy WASM→JS boundary, typed, compact |
| Type sharing | folivm-core shared by native + WASM | No type duplication, no generated bindings |

---

## 11. Data Flows

### 11.1 Document open

```
User opens file
    → shell: invoke('document:read', { path })
    → native: read .fvm bytes from disk
    → native: return raw .fvm string to shell
    → shell: wasm.load_document(fvm_string)
    → folivm-core/wasm: parse .fvm → DocumentModel
    → folivm-core/wasm: LayoutEngine.layout_all()
    → folivm-core/wasm: FrameRenderer.full_frame()
    → shell: drawInstructions(ctx, frame)
    → user sees document
```

### 11.2 Edit → save cycle

```
User types
    → shell: keydown captured on canvas
    → shell: wasm.on_keydown(key, modifiers)
    → folivm-core/wasm: InputHandler → Insert operation
    → folivm-core/wasm: RunBuffer updated (<1ms)
    → folivm-core/wasm: LayoutEngine.reflow(block)
    → folivm-core/wasm: DeltaRenderer.delta()
    → shell: drawInstructions(ctx, delta)
    Total: <2ms, zero IPC

Auto-save (30s debounce)
    → shell: wasm.request_serialise()
    → folivm-core/wasm: DocumentModel → .fvm string
    → shell: invoke('document:save', { path, content })
    → native: atomic file write
    → dirty indicator remains

Save Version (Cmd+Shift+S)
    → shell: wasm.request_serialise()
    → folivm-core/wasm: DocumentModel → .fvm string
    → shell: version message dialog
    → shell: invoke('document:save_version', { path, content, message })
    → native: write file + git commit
    → shell: dirty indicator cleared
```

### 11.3 Theme switch

```
User switches Brand mode
    → shell: mode change in VariablesPanel
    → shell: wasm.set_mode('brand', 'client-b')
    → folivm-core/wasm: ThemeResolver re-runs
    → folivm-core/wasm: FrameRenderer.full_frame()
    → shell: drawInstructions(ctx, frame)
    Total: <300ms
```

### 11.4 Export to PDF

```
User triggers Export → PDF
    → shell: ExportOpts dialog
    → shell: wasm.request_serialise()
    → folivm-core/wasm: DocumentModel → .fvm string
    → shell: invoke('export:pdf', { path, content, opts })
    → native: parse .fvm → DocumentModel (folivm-core, native target)
    → native: Stage 1 resolution (tokens, includes, data cells, theme)
    → native: extension export hooks
    → native: LayoutEngine (cosmic-text, native) → positioned glyphs per page
    → native: PdfBuilder (printpdf) → .pdf bytes
    → native: PDF/UA structure tags written
    → export:progress events → shell progress bar
    → native: atomic file write → native save dialog
```

---

*See SAD.md for implementation specification.*
