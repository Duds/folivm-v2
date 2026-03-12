# folivm — Software Architecture Document
**Version:** 0.2
**Date:** 2026-03-12
**Status:** In review
**Depends on:** HLD.md v0.3, FR.md v0.1, NFR.md v0.1

---

## 1. Scope

Rust is the complete application. The WebView is a display terminal. This document specifies the implementation of that architecture: the crate structure, the WASM module interface, the text engine, the layout engine, the render instruction system, the shell's role, the native backend, and the build pipeline.

---

## 2. Technology Stack

### 2.1 Native Rust process

| Concern | Crate / Tool |
|---|---|
| Async runtime | `tokio` |
| IPC framework | Tauri 2.x (commands + events) |
| Serialisation | `serde` / `serde_json` |
| YAML parsing | `serde_yaml` |
| Git | `git2` (libgit2 bindings) |
| DOCX generation | `docx-rs` |
| PDF generation | `printpdf` |
| Extension runtime | `deno_core` |
| File watching | `notify` |
| Directory walk | `ignore` (respects .gitignore) |
| Text search | `grep` (ripgrep crate family) |
| Unique IDs | `uuid` |

### 2.2 folivm-core (native + WASM)

| Concern | Crate / Tool |
|---|---|
| Text layout | `cosmic-text` |
| Unicode segmentation | `unicode-segmentation` |
| Binary serialisation | `postcard` (render instruction stream) |
| WASM bindings | `wasm-bindgen` |
| WASM build tool | `wasm-pack` |
| Font rasterisation | provided by cosmic-text (`swash`) |

### 2.3 Shell (WebView chrome)

| Concern | Choice |
|---|---|
| Language | TypeScript (strict) |
| Build tool | Vite |
| Rendering | Canvas 2D API |
| Framework | Minimal — React for panel components only |
| Word processor logic | None |

The shell does not import a state management library. Panel components use local React state for display concerns (scroll position, hover state, open/closed). There is no global document state in TypeScript.

### 2.4 Why these choices hold together

`cosmic-text` is used by System76's COSMIC desktop and Zed's text rendering layer. It is production-quality, actively maintained, and handles the full Unicode text processing stack. Running it in WASM gives us the same text layout in the editor that we would have in a fully native application — without the WebView being the authority on line breaking, cursor geometry, or font metrics.

`cosmic-text` serves double duty: compiled to WASM for the editor, compiled native for the export pipeline. Both compilation targets load the same font bytes. Line break decisions, glyph positions, and page break calculations are numerically identical between the editor canvas and the exported PDF. This is the fidelity guarantee: the same code, the same fonts, the same output.

`printpdf` generates PDF from positioned glyph data. It receives what `cosmic-text` computed and draws it at those exact positions. No layout is performed in `printpdf` — layout is `cosmic-text`'s job. `printpdf` writes the PDF structure, embeds fonts as subsets, and writes PDF/UA tags for accessibility compliance.

Canvas 2D is a stable, well-specified GPU-accelerated drawing API available in all WebKit versions. It has no connection to contenteditable, no browser default text behaviours, and no DOM node model. It is a bitmap surface we draw on.

---

## 3. Codebase Structure

```
folivm/
│
├── crates/
│   │
│   ├── folivm-core/            ← the word processor
│   │   │                         compiles to: native (lib) + wasm32
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── model/
│   │       │   ├── mod.rs
│   │       │   ├── document.rs     ← DocumentModel, Block, CellBlock, Inline
│   │       │   ├── frontmatter.rs  ← Frontmatter, PageSize, Margins
│   │       │   ├── operations.rs   ← EditOperation, apply(), invert()
│   │       │   └── ids.rs          ← BlockId (newtype over Uuid)
│   │       ├── text_engine/
│   │       │   ├── mod.rs
│   │       │   ├── input.rs        ← InputHandler
│   │       │   ├── buffer.rs       ← RunBuffer, Run
│   │       │   ├── cursor.rs       ← CursorManager, ModelPosition
│   │       │   ├── selection.rs    ← SelectionManager
│   │       │   └── undo.rs         ← UndoStack, UndoEntry
│   │       ├── layout/
│   │       │   ├── mod.rs
│   │       │   ├── engine.rs       ← LayoutEngine (wraps cosmic-text)
│   │       │   ├── page.rs         ← PageLayout, pagination
│   │       │   ├── scale.rs        ← ScaleFactor, pt↔px
│   │       │   └── metrics.rs      ← FontMetrics, GlyphCache
│   │       └── render/
│   │           ├── mod.rs
│   │           ├── instructions.rs ← RenderInstruction enum
│   │           ├── frame.rs        ← FrameRenderer (full redraw)
│   │           ├── delta.rs        ← DeltaRenderer (minimal update)
│   │           └── accessibility.rs← AccessibilityNode tree
│   │
│   ├── folivm-native/          ← native Tauri process
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── commands/       ← Tauri command handlers
│   │       ├── parser/         ← .fvm ↔ DocumentModel
│   │       ├── theme/          ← .fvm-theme parser + resolver
│   │       ├── export/         ← PDF + DOCX pipelines
│   │       ├── git/            ← git2-rs abstraction
│   │       ├── search/         ← ripgrep engine
│   │       ├── library/        ← content library
│   │       ├── extensions/     ← Deno Core host
│   │       └── watcher/        ← file system watcher
│   │
│   └── folivm-wasm/            ← WASM bindings
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs          ← wasm-bindgen exports
│
└── shell/                      ← chrome (TypeScript)
    ├── index.html
    ├── src/
    │   ├── main.ts             ← entry, wasm init, Tauri event listeners
    │   ├── canvas.ts           ← RenderInstruction → Canvas 2D
    │   ├── ime.ts              ← hidden input, composition forwarding
    │   ├── accessibility.ts    ← ARIA shadow tree management
    │   ├── bridge.ts           ← typed Tauri invoke() wrappers
    │   └── chrome/             ← panel components (React)
    └── vite.config.ts
```

`folivm-core` has no `cfg(target_arch = "wasm32")` conditionals in its core logic. The split is structural: `folivm-wasm` provides the WASM entry points; `folivm-native` provides the Tauri entry points. Both depend on `folivm-core`.

---

## 4. Process Model

### 4.1 Native Rust process

Starts at application launch. Lives for the application lifetime.

Owns:
- All file I/O (read, write, atomic rename)
- All git operations
- All export operations
- Extension host (Deno Core runtimes)
- File watcher
- Search engine

Does not own:
- DocumentModel for display purposes (owned by WASM)
- Any rendering state
- Any cursor or selection state

When the native process needs to operate on a document (export, git blame, search-and-replace), it parses the `.fvm` file independently using the same `folivm-core` parser. The native process and WASM module both use the same `DocumentModel` type — they parse independently from the same source of truth (the `.fvm` file on disk).

### 4.2 WASM module (folivm-core.wasm)

Loaded once per WebView session. One instance per open document tab.

Owns (per instance):
- `DocumentModel` for the open document
- `TextEngine` state (RunBuffer, CursorManager, SelectionManager, UndoStack)
- `LayoutEngine` state (laid-out lines, page breaks, glyph positions)
- `RendererState` (glyph cache, image cache, dirty regions)
- `ThemeState` (resolved theme tokens and styles)

Communicates with the shell via:
- Exported WASM functions (shell calls WASM)
- Callbacks registered at init (WASM calls shell for: IPC requests, IME position updates, ARIA updates)

### 4.3 Shell (TypeScript)

A thin process coordinator. It:
- Initialises WASM modules and passes font data
- Forwards raw input events to WASM
- Executes render instructions on the Canvas
- Manages the IME hidden input
- Updates the ARIA shadow tree
- Makes Tauri IPC calls on behalf of WASM

The shell never reads document content. It never makes decisions about text, layout, or rendering.

### 4.4 Communication map

```
                          Shell (TS)
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
  Canvas 2D              WASM module           Tauri IPC
  (draw calls)           (word processor)      (I/O, git, export)
        │                     │                     │
        │◄────instructions─────┤                     │
        │                     │◄────events──────────┤
        │                     │────IPC requests────►│
        │◄────a11y updates─────┤                     │
        │◄────IME position──────┤
```

---

## 5. WASM Module Interface

### 5.1 Exported functions (shell calls WASM)

```rust
// folivm-wasm/src/lib.rs

#[wasm_bindgen]
pub struct FolvimInstance {
    model: DocumentModel,
    text_engine: TextEngine,
    layout: LayoutEngine,
    renderer: RendererState,
    theme: ThemeState,
}

#[wasm_bindgen]
impl FolvimInstance {

    /// Create a new editor instance for one document tab
    pub fn new(callbacks: JsCallbacks) -> FolvimInstance

    /// Load a document from .fvm file content
    pub fn load_document(&mut self, fvm: &str) -> Result<(), JsValue>

    /// Load theme from .fvm-theme content
    pub fn load_theme(&mut self, theme: &str) -> Result<(), JsValue>

    /// Register font data (called once per font at startup)
    pub fn register_font(&mut self, family: &str, weight: u16, data: &[u8])

    /// Update canvas dimensions (on resize or zoom change)
    pub fn set_canvas_size(&mut self, width: f32, height: f32, dpi: f32, zoom: f32)
        -> RenderBuffer   // full frame after resize

    /// Raw keyboard input
    pub fn on_keydown(&mut self, key: &str, modifiers: u32) -> RenderBuffer

    /// Raw mouse input
    pub fn on_mousedown(&mut self, x: f32, y: f32, modifiers: u32) -> RenderBuffer
    pub fn on_mousemove(&mut self, x: f32, y: f32) -> RenderBuffer
    pub fn on_mouseup(&mut self, x: f32, y: f32) -> RenderBuffer

    /// IME composition events (forwarded from hidden input)
    pub fn on_composition_start(&mut self) -> RenderBuffer
    pub fn on_composition_update(&mut self, text: &str) -> RenderBuffer
    pub fn on_composition_end(&mut self, text: &str) -> RenderBuffer

    /// Paste content (plain text; structured .fvm paste handled separately)
    pub fn on_paste_text(&mut self, text: &str) -> RenderBuffer
    pub fn on_paste_fvm(&mut self, fvm: &str) -> RenderBuffer

    /// Mode switch
    pub fn set_mode(&mut self, mode: &str) -> RenderBuffer  // "outline" | "design"

    /// Serialise current DocumentModel to .fvm string (for save)
    pub fn serialise(&self) -> String

    /// Undo / redo
    pub fn undo(&mut self) -> RenderBuffer
    pub fn redo(&mut self) -> RenderBuffer

    /// Apply a structural operation received from shell
    /// (e.g. style picker applied a new block style)
    pub fn apply_operation(&mut self, op_json: &str) -> RenderBuffer

    /// Theme mode change
    pub fn set_theme_mode(&mut self, collection: &str, mode: &str) -> RenderBuffer

    /// Request a full frame render (on tab focus restore, etc.)
    pub fn full_frame(&self) -> RenderBuffer
}
```

### 5.2 RenderBuffer

`RenderBuffer` is the return type of all WASM functions that produce visual output. It is a WASM-allocated byte buffer containing postcard-encoded `Vec<RenderInstruction>`. The shell reads it as a `Uint8Array` view into WASM memory — zero copy.

```rust
#[wasm_bindgen]
pub struct RenderBuffer {
    ptr: *const u8,
    len: usize,
}

#[wasm_bindgen]
impl RenderBuffer {
    pub fn ptr(&self) -> *const u8 { self.ptr }
    pub fn len(&self) -> usize { self.len }
}
```

The shell decodes the buffer using a TypeScript postcard decoder and executes the instructions. The buffer is valid until the next WASM call that modifies renderer state.

### 5.3 JsCallbacks

At construction, the shell registers callbacks the WASM module can invoke:

```typescript
interface JsCallbacks {
  // WASM needs to make an IPC call (save, structural operation, etc.)
  ipc_invoke(command: string, args_json: string): Promise<string>

  // WASM reports new IME cursor position (shell repositions hidden input)
  ime_position(x: number, y: number, height: number): void

  // WASM emits updated ARIA tree (shell updates DOM)
  aria_update(nodes_json: string): void

  // WASM requests clipboard read
  clipboard_read(): Promise<string>

  // WASM writes to clipboard
  clipboard_write(text: string, fvm: string): void
}
```

---

## 6. Text Engine

All text engine components are in `folivm-core`. They are Rust. They run in WASM.

### 6.1 InputHandler

Maps raw input events to `EditOperation` values.

```rust
pub struct InputHandler {
    modifiers: ModifierState,
    in_composition: bool,
}

impl InputHandler {
    pub fn on_keydown(&mut self, key: &Key, modifiers: ModifierState)
        -> Vec<EditOperation>

    pub fn on_composition_start(&mut self) -> Vec<EditOperation>
    pub fn on_composition_update(&mut self, text: &str) -> Vec<EditOperation>
    pub fn on_composition_end(&mut self, text: &str) -> Vec<EditOperation>
}
```

Key mapping (non-exhaustive):

| Input | Operation |
|---|---|
| Printable character | `Insert { offset, text: char }` |
| Backspace, offset > 0 | `Delete { range: grapheme_before(offset) }` |
| Backspace, offset = 0 | `Merge { block_a: previous, block_b: active }` |
| Delete | `Delete { range: grapheme_after(offset) }` |
| Enter | `Split { offset }` |
| Shift+Enter | `Insert { offset, text: "\n" }` |
| Cmd+B | `SetInline { range: selection, style: Some("Strong") }` |
| Cmd+I | `SetInline { range: selection, style: Some("Emphasis") }` |
| Cmd+Z | routed to UndoStack |
| Cmd+Shift+Z | routed to UndoStack |
| Cmd+A | SelectAll → routed to SelectionManager |
| Arrow keys | routed to CursorManager, no EditOperation |

All keydown events that produce an operation prevent browser default via the WASM return value. The shell calls `event.preventDefault()` when the WASM return signals it.

### 6.2 RunBuffer

The RunBuffer holds the text content of the currently focused block as a flat `Vec<Run>`. It is the in-memory working copy. It is committed to the `DocumentModel` on: block exit, save request, any structural operation affecting the active block.

```rust
pub struct Run {
    pub text: String,
    pub style: Option<String>,   // None = unstyled
}

pub struct RunBuffer {
    block_id: BlockId,
    runs: Vec<Run>,
    dirty: bool,
}

impl RunBuffer {
    pub fn insert(&mut self, offset: usize, text: &str)
    pub fn delete(&mut self, range: Range<usize>)
    pub fn apply_style(&mut self, range: Range<usize>, style: &str)
    pub fn strip_style(&mut self, range: Range<usize>)
    pub fn commit(&mut self) -> Vec<Inline>  // → DocumentModel inline format
    pub fn total_len(&self) -> usize         // grapheme cluster count
}
```

All offset arithmetic uses grapheme cluster indices via `unicode-segmentation`. A grapheme cluster is one cursor position. Multi-codepoint emoji, combining marks, and regional indicator sequences each occupy one cursor position.

Run normalisation runs after every mutation: adjacent runs with identical style are merged. This keeps the run array minimal and render output stable.

### 6.3 CursorManager

```rust
pub struct ModelPosition {
    pub block_id: BlockId,
    pub offset: usize,       // grapheme cluster index
}

pub struct CursorManager {
    cursor: ModelPosition,
    preferred_x: Option<f32>,  // for vertical movement across lines of different width
}

impl CursorManager {
    pub fn move_left(&mut self, model: &DocumentModel, extend: bool)
    pub fn move_right(&mut self, model: &DocumentModel, extend: bool)
    pub fn move_up(&mut self, layout: &LayoutEngine, extend: bool)
    pub fn move_down(&mut self, layout: &LayoutEngine, extend: bool)
    pub fn move_line_start(&mut self, layout: &LayoutEngine, extend: bool)
    pub fn move_line_end(&mut self, layout: &LayoutEngine, extend: bool)
    pub fn move_word_left(&mut self, model: &DocumentModel, extend: bool)
    pub fn move_word_right(&mut self, model: &DocumentModel, extend: bool)
    pub fn set_from_point(&mut self, layout: &LayoutEngine, x: f32, y: f32)
    pub fn position(&self) -> &ModelPosition
}
```

`preferred_x` preserves horizontal cursor position across vertical movement through lines of different content length — the standard word processor behaviour when moving through short lines.

`set_from_point` resolves a canvas coordinate (from a mouse click) to the nearest grapheme cluster boundary using the LayoutEngine's glyph position data.

### 6.4 SelectionManager

```rust
pub struct ModelSelection {
    pub anchor: ModelPosition,
    pub focus:  ModelPosition,
}

pub struct SelectionManager {
    selection: Option<ModelSelection>,
}

impl SelectionManager {
    pub fn begin(&mut self, anchor: ModelPosition)
    pub fn extend(&mut self, focus: ModelPosition)
    pub fn select_all(&mut self, model: &DocumentModel)
    pub fn select_word(&mut self, model: &DocumentModel, pos: ModelPosition)
    pub fn clear(&mut self)
    pub fn get(&self) -> Option<&ModelSelection>
    pub fn is_collapsed(&self) -> bool
    pub fn ordered(&self) -> Option<(&ModelPosition, &ModelPosition)>
        // returns (start, end) regardless of anchor/focus direction
}
```

### 6.5 UndoStack

```rust
pub struct UndoEntry {
    operations: Vec<EditOperation>,     // the operations applied
    inverses:   Vec<EditOperation>,     // their inverses, in reverse order
    cursor_before: ModelPosition,
    cursor_after:  ModelPosition,
    timestamp: u64,                     // for coalescing
}

pub struct UndoStack {
    stack: Vec<UndoEntry>,
    pointer: usize,
}

impl UndoStack {
    pub fn push(&mut self, entry: UndoEntry)
    pub fn undo(&mut self, model: &mut DocumentModel) -> Option<ModelPosition>
    pub fn redo(&mut self, model: &mut DocumentModel) -> Option<ModelPosition>
    pub fn clear(&mut self)   // called after version save
}
```

Coalescing: consecutive `Insert` operations with the same `block_id` and within 300ms are merged into a single `UndoEntry`. Same for consecutive `Delete` operations. This produces word-level undo granularity for rapid typing.

---

## 7. Layout Engine

### 7.1 LayoutEngine

Wraps `cosmic-text` to produce the geometric data the renderer needs.

```rust
pub struct LayoutEngine {
    font_system: cosmic_text::FontSystem,
    scale_factor: ScaleFactor,
    mode: RenderMode,              // Outline | Design
    layout_cache: HashMap<BlockId, BlockLayout>,
}

pub struct BlockLayout {
    block_id: BlockId,
    lines: Vec<LineMeasurement>,
    total_height_px: f32,
    baseline_px: f32,
}

pub struct LineMeasurement {
    y_offset: f32,
    glyphs: Vec<GlyphPosition>,
}

pub struct GlyphPosition {
    x: f32, y: f32,
    glyph_id: u32,
    font_id: u8,
    advance: f32,
    cluster: usize,   // grapheme cluster index in block
}

impl LayoutEngine {
    pub fn layout_block(&mut self, block: &Block, theme: &ResolvedTheme, width_px: f32)
        -> &BlockLayout

    pub fn layout_all(&mut self, model: &DocumentModel, theme: &ResolvedTheme)

    pub fn cursor_rect(&self, pos: &ModelPosition) -> Rect
    pub fn selection_rects(&self, sel: &ModelSelection) -> Vec<Rect>
    pub fn position_from_point(&self, x: f32, y: f32) -> ModelPosition

    pub fn paginate(&self, model: &DocumentModel, page: &PageLayout)
        -> Vec<PageContents>    // Design mode only
}
```

`cosmic-text`'s `FontSystem` holds font data and handles shaping via `rustybuzz` (HarfBuzz port). It is initialised with font bytes passed from the shell at startup.

### 7.2 ScaleFactor

```rust
pub struct ScaleFactor {
    canvas_width_px: f32,
    page_width_pt: f32,
    screen_dpi: f32,
    zoom: f32,
}

impl ScaleFactor {
    pub fn compute(&self) -> f32 {
        (self.canvas_width_px / self.page_width_pt)
            * (self.screen_dpi / 96.0)
            * self.zoom
    }

    pub fn pt_to_px(&self, pt: f32) -> f32 { pt * self.compute() }
    pub fn px_to_pt(&self, px: f32) -> f32 { px / self.compute() }
}
```

Updated on: canvas resize, display DPI change (Tauri event), zoom change. All layout calculations use `pt_to_px()`. The design canvas, ruler, margin guides, and tab stops all read from this single value.

---

## 8. Render System

### 8.1 RenderInstruction

```rust
pub enum RenderInstruction {
    // Geometry
    FillRect   { x: f32, y: f32, w: f32, h: f32, color: u32 },
    StrokeRect { x: f32, y: f32, w: f32, h: f32, color: u32, line_width: f32 },

    // Text (positioned glyphs from cosmic-text layout)
    DrawGlyph  { x: f32, y: f32, glyph_id: u32, font_id: u8, size: f32, color: u32 },

    // Editor state
    Cursor     { x: f32, y: f32, height: f32, color: u32 },
    Selection  { rects: Vec<Rect>, color: u32 },
    Composition{ rects: Vec<Rect>, text: String, color: u32 },  // IME underline

    // Images
    DrawImage  { x: f32, y: f32, w: f32, h: f32, image_id: u32 },

    // Clipping
    ClipPush   { x: f32, y: f32, w: f32, h: f32 },
    ClipPop,

    // Scroll
    ScrollTo   { y: f32 },

    // Layer control
    SaveState,
    RestoreState,
}
```

### 8.2 FrameRenderer

Produces a complete set of instructions for the full visible canvas. Called on: document open, mode switch, theme change, window resize.

```rust
pub struct FrameRenderer<'a> {
    model: &'a DocumentModel,
    layout: &'a LayoutEngine,
    theme: &'a ResolvedTheme,
    mode: RenderMode,
    cursor: &'a ModelPosition,
    selection: Option<&'a ModelSelection>,
    scroll_y: f32,
    viewport: Rect,
}

impl<'a> FrameRenderer<'a> {
    pub fn render(&self) -> Vec<RenderInstruction>
}
```

### 8.3 DeltaRenderer

Produces a minimal instruction set for a single operation — a block edit, a cursor move, a selection change. Used for every keystroke to avoid full redraws.

```rust
impl DeltaRenderer {
    pub fn after_text_op(
        layout: &LayoutEngine,
        theme: &ResolvedTheme,
        changed_block: &Block,
        cursor: &ModelPosition,
        selection: Option<&ModelSelection>,
        scroll_y: f32,
    ) -> Vec<RenderInstruction>

    pub fn cursor_only(
        layout: &LayoutEngine,
        cursor: &ModelPosition,
        prev_cursor: &ModelPosition,
    ) -> Vec<RenderInstruction>
}
```

### 8.4 Canvas renderer (shell/canvas.ts)

Executes `RenderInstruction` values against the Canvas 2D context. It maintains:

- **Glyph cache**: `Map<glyph_id, ImageData>` — pre-rasterised glyphs, keyed by glyph ID from cosmic-text layout
- **Image cache**: `Map<image_id, HTMLImageElement>` — document images, loaded on demand, keyed by ID assigned by WASM

```typescript
function executeInstructions(
  ctx: CanvasRenderingContext2D,
  buffer: Uint8Array,
  glyphCache: GlyphCache,
  imageCache: ImageCache,
): void {
  const instructions = decodePostcard(buffer)  // typed decoder
  for (const instr of instructions) {
    switch (instr.type) {
      case 'FillRect':   ctx.fillStyle = colorToHex(instr.color)
                         ctx.fillRect(instr.x, instr.y, instr.w, instr.h); break
      case 'DrawGlyph':  drawGlyph(ctx, glyphCache, instr); break
      case 'Cursor':     drawCursor(ctx, instr); break
      case 'Selection':  drawSelection(ctx, instr); break
      case 'DrawImage':  drawImage(ctx, imageCache, instr); break
      case 'ClipPush':   ctx.save(); ctx.beginPath()
                         ctx.rect(instr.x, instr.y, instr.w, instr.h)
                         ctx.clip(); break
      case 'ClipPop':    ctx.restore(); break
      // ...
    }
  }
}
```

---

## 9. IME and Clipboard

### 9.1 IME (shell/ime.ts)

A hidden `<input type="text">` element tracks the cursor position and receives OS composition events.

```typescript
class ImeManager {
  private input: HTMLInputElement

  constructor(callbacks: { onCompositionStart, onCompositionUpdate, onCompositionEnd }) {
    this.input = document.createElement('input')
    this.input.style.cssText = 'position:fixed; opacity:0; pointer-events:none; width:1px'
    document.body.appendChild(this.input)

    this.input.addEventListener('compositionstart',  e => callbacks.onCompositionStart())
    this.input.addEventListener('compositionupdate', e => callbacks.onCompositionUpdate(e.data))
    this.input.addEventListener('compositionend',    e => callbacks.onCompositionEnd(e.data))
  }

  // Called by WASM callback when cursor moves
  updatePosition(x: number, y: number, height: number): void {
    this.input.style.left = `${x}px`
    this.input.style.top  = `${y + height}px`
    this.input.focus()
  }
}
```

During an active composition session, the WASM module renders the tentative text on the canvas with a composition underline. When `compositionend` fires, the WASM module discards the tentative state and commits the final text as a single Insert operation.

### 9.2 Clipboard

Cut/copy/paste are handled by keydown events in the `InputHandler`. The WASM module signals the required clipboard operation via its `JsCallbacks` interface. The shell executes the platform clipboard API call and passes the result back to WASM.

Paste behaviour:
- `.fvm` content on clipboard → `on_paste_fvm()` → structured block insert
- Plain text on clipboard → `on_paste_text()` → unstyled text insert, prompt for style
- Anything else (HTML, RTF) → stripped to plain text → same as plain text path

---

## 10. Accessibility

### 10.1 ARIA shadow tree (shell/accessibility.ts)

The canvas is `aria-hidden="true"`. A parallel DOM tree of hidden ARIA elements mirrors the document structure for screen readers.

```typescript
// Structure mirrors DocumentModel
// Updated via wasm callback: aria_update(nodes_json)

interface AriaNode {
  role: 'document' | 'heading' | 'paragraph' | 'list' | 'listitem'
        | 'figure' | 'img' | 'region' | 'note' | 'alert'
  level?: number          // for headings: aria-level
  label?: string          // for images: aria-label (= alt text)
  live?: 'polite'         // for updated regions
  text?: string           // text content
  children?: AriaNode[]
}
```

The ARIA tree is rebuilt when: a document loads, a block's content changes, a block's style changes, a block is added or removed. Updates are batched and applied once per frame.

### 10.2 WCAG structural guarantees

The ARIA shadow tree enforces the same structural guarantees documented in FR.md and NFR.md:

| Criterion | Mechanism |
|---|---|
| Real heading structure | `role="heading"` with correct `aria-level` from block type |
| Image alt text | `aria-label` from `cell:image.alt` — required field |
| Reading order | ARIA tree reflects document block order |
| Semantic callouts | `role="note"` (Tip), `role="alert"` (Warning), `role="region"` (Remember) |
| Language | `lang` attribute on root ARIA element from frontmatter |

---

## 11. Native Backend

### 11.1 Document parse (folivm-native/parser)

The native parser converts `.fvm` file content to `DocumentModel` using `folivm-core` types. It is used by: export pipeline, git operations, search-and-replace, library resolver. The WASM module uses the same parser code (compiled to WASM target) for display.

Round-trip invariant: `parse(serialise(model)) == model`. Verified by test suite.

### 11.2 Export pipeline

Runs entirely in the native process. WASM not involved. WebKit not involved.

**PDF generation:**

```
folivm-core (native) parses .fvm → DocumentModel
    ↓
ExportResolver: resolve tokens, cell:include, cell:data, theme
    ↓
Extension export hooks (in registration order)
    ↓
LayoutEngine (cosmic-text, native)
    font bytes: identical to those loaded by the WASM module
    produces: Vec<PageLayout> — positioned glyphs per page
    ↓
PdfBuilder (printpdf)
    for each page:
        for each positioned glyph: pdf.draw_glyph(x, y, glyph_id, font_id, size)
        for each image: pdf.draw_image(x, y, w, h, image_bytes)
        for each vector element: pdf.draw_path(...)
    embed fonts as subsets
    write PDF/UA structure tags (document, headings, figures, lists)
    ↓
.pdf file (atomic write)
```

The fidelity invariant: because the native LayoutEngine and the WASM LayoutEngine are the same `folivm-core` code compiled to different targets, and because they load the same font bytes, the glyph positions and line breaks in the PDF are numerically identical to those rendered in the editor. The editor is not an approximation of the output — it is the output.

**DOCX generation:**

`docx-rs` constructs the DOCX document object from the `ExportDocument`. Named styles map to DOCX paragraph and character styles. Heading blocks map to DOCX `Heading 1–6`. `cell:math` maps to OMML. `cell:diagram` embeds as SVG image. Document language from frontmatter sets `w:lang`.

### 11.3 Git abstraction

Unchanged from HLD v0.3. git2-rs in native process. Document-language UX mapped to git operations.

### 11.4 Extension host

Extensions run in Deno Core sandboxes in the native process. They receive `DocumentModel` segments via the extension API. They return modified segments or new blocks. They cannot access the WASM module directly.

Extension UI panels are HTML loaded in sandboxed iframes in the shell. They communicate with their extension runtime via a message channel proxied through the shell's IPC bridge.

### 11.5 Search

ripgrep-based streaming text search over `.fvm` files. No document parsing required — `.fvm` is ASCII. Results stream to shell via Tauri events.

---

## 12. Build Configuration

### 12.1 Rust workspace

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "crates/folivm-core",
    "crates/folivm-native",
    "crates/folivm-wasm",
]
resolver = "2"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true

[profile.release-wasm]
inherits = "release"
opt-level = "z"    # minimise WASM binary size
```

### 12.2 WASM build (folivm-wasm)

```toml
# crates/folivm-wasm/Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
folivm-core = { path = "../folivm-core" }
wasm-bindgen = "0.2"
postcard = { version = "1", features = ["alloc"] }
```

Build command: `wasm-pack build crates/folivm-wasm --target web --release`

Output: `shell/src/wasm/folivm_wasm.js` + `folivm_wasm_bg.wasm`

The WASM binary is loaded by the shell at startup via `WebAssembly.instantiateStreaming`.

### 12.3 Shell build (Vite)

```typescript
// vite.config.ts
export default {
  root: 'shell',
  build: {
    outDir: '../dist',
    target: 'esnext',
  },
  server: {
    port: 5173,     // Tauri dev mode
  },
}
```

The WASM file is served as a static asset. In production it is bundled into the Tauri application package.

### 12.4 Native build (Tauri)

```
crates/folivm-native/  → Rust binary (platform native)
shell/dist/            → static assets (bundled into binary)
crates/folivm-wasm/pkg/→ .wasm file (bundled into static assets)
```

Build sequence:
1. `wasm-pack build` → produces `folivm_wasm.js` + `.wasm`
2. `vite build` → bundles shell + WASM assets
3. `tauri build` → compiles native Rust, bundles everything, produces installer

### 12.5 Platform targets

| Platform | Format |
|---|---|
| macOS | `.app` + `.dmg` (universal: x86_64 + aarch64) |
| Windows | `.msi` + NSIS installer |
| Linux | `.deb` + `.AppImage` |

---

## 13. Error Handling

### 13.1 Native process errors

```rust
#[derive(Debug, Serialize)]
#[serde(tag = "code")]
pub enum AppError {
    Io          { message: String, path: Option<PathBuf> },
    ParseError  { message: String, path: PathBuf, line: Option<usize> },
    GitError    { message: String },
    ExportError { message: String, stage: String },
    ExtError    { ext_id: String, message: String },
    Permission  { ext_id: String, api: String },
}
```

All Tauri commands return `Result<T, AppError>`. Errors are serialised to JSON and returned to the shell. The shell renders non-blocking toast notifications for user-visible errors and logs diagnostic detail to the developer console.

### 13.2 WASM errors

WASM functions return `Result<T, JsValue>`. On error, the WASM module:
- Restores the pre-operation model state (operations are atomic via `apply()` + `invert()`)
- Returns a `JsValue` error with code and message
- The shell logs the error and shows a non-blocking notification if user-visible

The WASM module cannot panic in production. `panic = "abort"` is set and a panic hook writes the panic message via the `JsCallbacks` error channel before aborting.

### 13.3 Document integrity

Files are written atomically (temp file + rename). A failed write leaves the original file intact. The git layer stages only after a confirmed successful write. A write failure produces no git state change.

### 13.4 Extension errors

Extension errors are caught by the Deno Core runtime host. A failed extension hook is logged. A failed extension cell renderer shows a placeholder. Extensions cannot crash the native process.

---

## 14. Testing Strategy

### 14.1 folivm-core unit tests (Rust)

- `model/operations.rs` — every `EditOperation` variant applied to known model, inverse verified, round-trip asserted
- `parser/` — round-trip corpus test: `parse(serialise(model)) == model` on 50+ `.fvm` fixture files
- `text_engine/buffer.rs` — insert, delete, apply_style, normalisation, grapheme boundary correctness (Unicode test cases)
- `text_engine/cursor.rs` — movement through multi-line, multi-block content; boundary conditions
- `text_engine/undo.rs` — undo/redo sequences, coalescing, pointer correctness
- `layout/scale.rs` — scale factor computation, pt↔px round-trip precision
- `layout/engine.rs` — layout of known text against expected line break positions

### 14.2 WASM integration tests (wasm-bindgen-test)

Run in a headless browser via `wasm-pack test`:
- Load document → serialise → assert round-trip
- Keystroke sequence → assert DocumentModel state
- Undo → redo → assert state
- Mode switch → assert render instruction set changes
- Theme load → assert ThemeState resolved correctly

### 14.3 Native unit tests (Rust)

- `parser/` — same round-trip tests as folivm-core (native parse path)
- `git/` — integration tests against a real temporary repository
- `export/` — snapshot tests: known `DocumentModel` → expected PDF/DOCX output (byte-level for DOCX structure, pixel-diff for PDF)
- `search/` — match and replace on fixture project directories

### 14.4 Fidelity regression test

Design mode measurement accuracy:
1. Load a document with known page dimensions (A4 portrait, 25mm margins)
2. Assert: cursor at first character has `x` coordinate matching left margin in canvas px
3. Assert: page height in canvas px = `pageHeightPt × scale` within ±0.5pt

Runs in CI on every pull request via wasm-pack test. Guards directly against the v1 measurement failure.

### 14.5 Accessibility test

1. Load a document with headings H1/H2/H3, an image with alt text, a Warning callout
2. Assert ARIA shadow tree contains: `role=heading level=1`, `role=heading level=2`, `role=alert`
3. Assert image ARIA node has `aria-label` matching the `alt` field

---

*Document history tracked in git.*
