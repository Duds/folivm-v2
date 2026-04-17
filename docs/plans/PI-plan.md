# Folivm v2 — PI 1 Plan & Full Product Backlog

> **Context:** Folivm is a semantic-first professional document editor. Rust owns the full application logic (document model, text engine, layout, render). The TypeScript/Tauri shell is a display terminal only. This plan covers PI 1 in sprint-level detail and the full backlog across all PIs through v1.0 launch.
>
> **Peer review applied** — see "Reviewer Notes" sections embedded throughout.

---

## Program Increment Overview

| PI | Theme | Duration | End State |
|----|-------|----------|-----------|
| **PI 1** | Walking Skeleton | 10 wks / 5 sprints | *Current State: Scaffolded.* Corrected end state: runnable workspace, round-trip validated, basic text editing. |
| **PI 2** | Functional Base & Design Preview | 8 wks / 4 sprints | Full text engine (Undo/Redo), Layout Engine integration, Style picking, first page pagination |
| **PI 3** | Export + Versioning | 8 wks / 4 sprints | Production PDF/UA + DOCX export, git-based versioning with diff view |
| **PI 4** | Content Library + Extensions | 10 wks / 5 sprints | `cell:include` / Content Library, Deno Core extension host, `cell:image`, `cell:math` |
| **PI 5** | Polish + Platform | 8 wks / 4 sprints | Full keyboard map, accessibility (ARIA, PDF/UA), performance, cross-platform installers |

**Velocity assumption:** Target 35–40 points per sprint. **Calibrate after Sprint 1** — these are initial estimates with no baseline. First sprint is the riskiest for estimation accuracy.

---

## PI 1 — Walking Skeleton

### PI 1 Objectives

> [!IMPORTANT]
> **Correction Phase**: Sprint 1–5 were completed as structural scaffolds. PI 2 now begins with a 4-week recovery to turn these stubs into a functional editor before proceeding to Design Mode.

1. **Cargo workspace is runnable** — `cargo build` succeeds, `cargo test` runs, CI passes on every push (FIXED: missing traits added)
2. **Round-trip invariant holds** — `parse(serialise(model)) == model` for all block types in FORMAT.md
3. **Document opens in Tauri** — `.fvm` file reads from disk, parses in WASM, renders in Outline mode on canvas
4. **Basic text editing works** — Insert, Delete, cursor movement, block split/merge, undo/redo
5. **Document saves to disk** — Auto-save on 30s debounce (file write only; no git commit until PI 3)

---

### Architectural Decisions Resolved Before Sprint 1

> These must be decided before writing code, not deferred to the sprint.

**Parser strategy** (resolved: custom parser, not pulldown-cmark)
The `.fvm` format is Markdown-like but NOT Markdown-compatible. `pulldown-cmark` cannot:
- Parse and preserve `<!-- block:uuid -->` comments without mutation
- Parse cell fence syntax (`:::cell:type` + YAML metadata block)
- Distinguish section vs. callout fences by attribute presence
- Preserve unknown cell types verbatim (required for extension forward-compatibility)
- Produce canonical, deterministic serialisation

**Decision:** Write a purpose-built line-level tokeniser for block structure. Use `pulldown-cmark` only as a sub-parser for inline content within text blocks. YAML frontmatter via the `serde_yaml` crate.

**Parser location** (resolved: folivm-core, not folivm-native)
CLAUDE.md states: "parsing happens in `folivm-core`, not in the backend." HLD shows a `parser/` folder in folivm-native — that is the **export** parser path for the native process. The canonical parser lives in `folivm-core` and is compiled to both `native` and `wasm32` targets.

**Font bundling** (resolved: compile-time embedded bytes)
`cosmic-text` requires font bytes at init time. The WASM and native targets must load **identical** font bytes — this is the fidelity guarantee. Decision: embed one or more open-licence fonts (e.g., Inter + a monospace) as `include_bytes!()` in `folivm-core`. Same bytes in both targets; no runtime font loading path in PI 1.

---

### Sprint 1 — Foundation (Weeks 1–2)

**Goal:** Cargo workspace boots. Document model is complete. Parser and serialiser pass round-trip tests using FORMAT.md fixtures.

| # | Story | Points | Crate | Notes |
|---|-------|--------|-------|-------|
| S1-1 | Initialize Cargo workspace (`Cargo.toml`, `folivm-core`, `folivm-native`, `folivm-wasm` crates) | 2 | workspace | Feature flags: `native`, `wasm` on `folivm-core`; `Cargo.lock` committed (binary project) |
| S1-2 | CI: GitHub Actions — `cargo test -p folivm-core` + `tsc --noEmit` on PR | 2 | CI | |
| S1-3 | Implement `Block` enum — all v1.0 types: Paragraph, Heading, List, Blockquote, Table, Section, Callout, Cell | 5 | folivm-core | Include `BlockId` (UUID v4 via `uuid` crate) |
| S1-4 | Implement `Inline` enum — Text, Styled, Token | 2 | folivm-core | |
| S1-5 | Implement `DocumentModel` + `Frontmatter` structs; derive `#[derive(TS)]` on all shared types | 3 | folivm-core | `ts-rs` derives configured here; `build.rs` exports TS types to `shell/src/types/` |
| S1-6 | Implement `EditOperation` enum — Insert, Delete, Split, Merge, SetBlock, SetInline, InsertCell, SetCell | 3 | folivm-core | `apply()` and `invert()` stubs (full impl in Sprint 2) |
| S1-7 | Custom parser — line-level tokeniser for block structure (detects UUID comments, fence blocks, YAML metadata) | 5 | folivm-core | Lives in `folivm-core/src/parser/`; pulldown-cmark used only for inline content |
| S1-8 | Custom parser — body blocks: Paragraph, Heading, List, Blockquote, Table, Callout, Section, all Cell types | 8 | folivm-core | Unknown cell types preserved verbatim; all block UUIDs extracted and stored |
| S1-9 | Implement `.fvm` serialiser — `DocumentModel → String`, deterministic output | 5 | folivm-core | Must produce canonical output (sequential list numbers, stable block ordering) |
| S1-10 | Round-trip test suite — fixture-based: `parse(serialise(model)) == model` for all FORMAT.md examples | 3 | folivm-core | Fixture files, not proptest — proptest overhead is not justified in Sprint 1 |
| S1-11 | Font bundling — embed Inter + a monospace font via `include_bytes!()` in `folivm-core`; expose `font_data()` fn | 3 | folivm-core | Same bytes used by both WASM and native targets; critical for fidelity guarantee |

**Sprint 1 total:** 41 points
**DoD:** `cargo test -p folivm-core` passes, round-trip invariant green, TS types generated, CI runs.

> **Risk:** Sprint 1 total (41 pts) is above the unvalidated 35–40pt target. If behind by mid-sprint, defer S1-11 (font bundling) to Sprint 3 where `cosmic-text` is integrated — but flag this dependency.

---

### Sprint 2 — Text Engine (Weeks 3–4)

**Goal:** In-memory text buffer accepts edits. Cursor and selection track correctly. Undo/redo works.

| # | Story | Points | Crate | Notes |
|---|-------|--------|-------|-------|
| S2-1 | Implement `Run` and `RunBuffer` — `Vec<Run>`, insert/delete/apply_style operations | 5 | folivm-core | Each `Run` is a styled text span with inline style name |
| S2-2 | Implement `EditOperation::apply()` and `invert()` fully for text ops (Insert, Delete, Split, Merge) | 5 | folivm-core | Invariant: `apply(invert(op), state) == original_state` |
| S2-3 | Implement `CursorManager` — `ModelPosition {block_id, offset}`, left/right/up/down/Home/End/Cmd+arrows | 5 | folivm-core | |
| S2-4 | Implement `SelectionManager` — anchor/focus model, cross-block selection | 3 | folivm-core | Shift+navigation extends selection |
| S2-5 | Implement `UndoStack` — operation stack with inverses; word-level coalescing (5s window for consecutive inserts) | 5 | folivm-core | |
| S2-6 | Implement block split (Enter) and merge (Backspace at block boundary) | 5 | folivm-core | Primary block keeps its `BlockId`; new block gets fresh UUID |
| S2-7 | Implement `InputHandler` — map key events to `EditOperation` (printable chars, Backspace, Delete, Enter, Shift+Enter) | 5 | folivm-core | Runs in WASM context; full keyboard map is PI 2 (C-1) |
| S2-8 | Unit tests — text engine: all ops, cursor movement, selection, undo/redo round-trips | 3 | folivm-core | |

**Sprint 2 total:** 36 points
**DoD:** `apply(invert(op), model) == original_model` for all text ops. Cursor, selection, undo tested at edges.

---

### Sprint 3 — Layout Engine + Render Instructions (Weeks 5–6)

**Goal:** Documents are laid out using `cosmic-text`. A stream of `RenderInstruction`s can be produced from a `DocumentModel`. WASM binary size is measured.

| # | Story | Points | Crate | Notes |
|---|-------|--------|-------|-------|
| S3-1 | Integrate `cosmic-text`; implement `ScaleFactor` — `canvas_width_px / page_width_pt × dpi/96 × zoom` | 3 | folivm-core | Pin `cosmic-text` to a specific tag; abstract behind `LayoutEngine` trait to insulate from API changes |
| S3-2 | Implement `LayoutEngine` — wraps `cosmic-text`, loads embedded font bytes, shapes lines from a block's `RunBuffer` | 8 | folivm-core | Unicode shaping, BiDi, line breaking; this is the **critical path item** in PI 1 |
| S3-3 | Implement cursor geometry — pixel rect from `ModelPosition` | 3 | folivm-core | |
| S3-4 | Implement selection geometry — `Vec<Rect>` from selection range | 3 | folivm-core | |
| S3-5 | Implement `PageLayout` (Outline mode) — block heights, scroll layout, no pagination yet | 3 | folivm-core | Pagination deferred to PI 2 (Design mode) |
| S3-6 | Define `RenderInstruction` enum — FillRect, StrokeRect, DrawGlyph, DrawImage, Cursor, Selection, ClipPush, ClipPop | 2 | folivm-core | |
| S3-7 | Implement `FrameRenderer` — full frame from `DocumentModel` + layout → `Vec<RenderInstruction>` (Outline mode only) | 8 | folivm-core | Depends on S3-2 |
| S3-8 | Implement `DeltaRenderer` — minimal instruction set after a single `EditOperation` | 5 | folivm-core | Reflow only the affected block(s) |
| S3-9 | Snapshot tests — render a fixture document, assert `RenderInstruction` stream matches expected output | 3 | folivm-core | |
| S3-10 | **WASM binary size spike** — build `folivm-wasm` with all Sprint 3 deps; measure `.wasm` size; apply LTO + `wasm-opt`; report result and flag if >5MB | 2 | folivm-wasm | **Must do now**, not in Sprint 4. If size is a problem, options are: lazy font loading, split crate, defer KaTeX |

**Sprint 3 total:** 40 points
**Critical dependency:** S3-2 (cosmic-text integration) blocks S3-7 (FrameRenderer) which blocks S4-7 (canvas.ts). If S3-2 slips, the entire sprint slips. Flag on Day 1 of Sprint 3.
**DoD:** Given a `DocumentModel`, `FrameRenderer` produces a correct `RenderInstruction` stream. `DeltaRenderer` produces a strict subset after any single op. WASM size measured and documented.

---

### Sprint 4 — WASM Bindings + Shell (Weeks 7–8)

**Goal:** WASM module loads in browser/WebView. Shell executes render instructions on canvas. Events flow from shell to WASM.

> **Reviewer note:** This sprint is the most technically risky in PI 1. S4-7 (canvas.ts with glyph atlas) was originally estimated at 8pts but is more realistically 13pts. This sprint is adjusted: canvas.ts glyph caching is a stretch goal, and S4-10 (e2e test) moves to Sprint 5. If canvas.ts spills, Sprint 5 can absorb it.

| # | Story | Points | Crate/Layer | Notes |
|---|-------|--------|-------------|-------|
| S4-1 | Set up `folivm-wasm` crate with `wasm-bindgen`; `wasm-pack build --target web` succeeds | 3 | folivm-wasm | |
| S4-2 | Expose `FolivmInstance` to JS — `new FolivmInstance(canvas_width, canvas_height, dpi)` | 3 | folivm-wasm | |
| S4-3 | Expose `FolivmInstance::load_document(fvm_string: &str)` — parse, init layout, return render frame | 2 | folivm-wasm | |
| S4-4 | Expose event dispatch — `on_key_down`, `on_key_up`, `on_mouse_down`, `on_mouse_up`, `on_mouse_move` — returns delta | 3 | folivm-wasm | |
| S4-5 | `RenderInstruction` serialisation — postcard binary via WASM memory (preferred) or JSON fallback; decide based on S3-10 binary size result | 3 | folivm-wasm | Zero-copy WASM memory approach: `wasm_bindgen::memory()` + `Uint8Array` view. If complexity exceeds 2 days, use JSON and revisit in PI 2 |
| S4-6 | Bootstrap shell — Vite + TypeScript in `shell/`, `index.html`, Tauri integration; `cd shell && tsc --noEmit` passes | 3 | shell | |
| S4-7 | Implement `canvas.ts` — load WASM module, receive `RenderInstruction` stream, execute against Canvas 2D API | **13** | shell | Glyph atlas (font_id+glyph_id → ImageBitmap), image cache, delta application. **This is the sprint's largest item.** |
| S4-8 | Implement `ime.ts` — hidden `<input>` element, composition events → WASM `on_composition_*` calls | 5 | shell | CJK input support; `preventDefault()` on composition keys |
| S4-9 | Keyboard/mouse event forwarding — raw DOM events → WASM dispatch → delta applied to canvas | 3 | shell | `preventDefault()` on all editor-bound keys |

**Sprint 4 total:** 38 points (with S4-7 at 13pts)
**Stretch:** S4-10 (end-to-end browser test) — if ahead of schedule, add it here; otherwise it moves to Sprint 5.
**DoD:** Browser (not Tauri yet) loads a `.fvm` string, renders it on canvas, and accepts keystrokes that update the canvas via delta instructions.

---

### Sprint 5 — Tauri Integration + Outline Mode (Weeks 9–10)

**Goal:** Real Tauri application. Open a project, open a `.fvm` document, edit it, save it. Outline mode is navigable. PI 1 demo ready.

| # | Story | Points | Crate/Layer | Notes |
|---|-------|--------|-------------|-------|
| S4-10 | End-to-end test — load WASM in browser, render fixture document, assert canvas draw calls (moved from Sprint 4) | 3 | shell | Playwright or manual screenshot comparison |
| S5-1 | Bootstrap `folivm-native` — Tauri app setup, custom title bar (macOS), `cargo tauri dev` opens a window | 3 | folivm-native | |
| S5-2 | Tauri command `document:read` — reads `.fvm` file from path, returns raw string (no parsing in native) | 2 | folivm-native | CLAUDE.md invariant: "The backend never interprets document content" |
| S5-3 | Tauri command `document:save` — writes `.fvm` string atomically (temp file + rename); failure leaves original intact | 3 | folivm-native | Atomic write is non-negotiable |
| S5-4 | Tauri command `project:open` — opens directory, returns `ProjectMeta` (name, root path, `.fvm` file list) | 3 | folivm-native | |
| S5-5 | Tauri command `project:create` — creates directory + empty `.fvm` + `git init` (git plumbing, no commit yet) | 3 | folivm-native | |
| S5-6 | Implement `ipc.ts` — typed wrappers for all Tauri commands using `ts-rs`-generated types from Sprint 1 (S1-5) | 2 | shell | Types already generated; this is just the wrapper layer |
| S5-7 | Shell app frame — activity bar (icons), left sidebar (file explorer), main canvas area, status bar | 5 | shell | Hard-coded layout; no animations; sidebar is a static placeholder |
| S5-8 | Auto-save — 30s debounce after any edit → `document:save`; "Saving…" indicator in status bar; **no git commit** (git versioning is PI 3) | 3 | shell | |
| S5-9 | Outline mode block display — blocks rendered via canvas, style badge (text label), word count per block | 5 | shell + folivm-core | Themes not yet wired; style names shown as-is from the model |
| S5-10 | Keyboard navigation in Outline mode — arrows, Tab/Shift-Tab (heading promote/demote), Alt+↑/↓ (block move) | 5 | folivm-core | |
| S5-11 | Status bar — word count (total), cursor position (block #, offset), active mode badge | 2 | shell | |
| S5-12 | PI 1 demo — fixture document (covers all block types), acceptance test checklist, demo script | 2 | — | Fixture doubles as the round-trip test source from Sprint 1 |

**Sprint 5 total:** 41 points
**DoD (PI 1 Done Criteria — all 8 must pass):**
1. `cargo build` succeeds on all three crates
2. `cargo test -p folivm-core` passes — round-trip invariant + text engine + render snapshot tests
3. `cd shell && tsc --noEmit` succeeds
4. `cargo tauri dev` opens a native window
5. User can open a project folder and open a `.fvm` file; blocks appear in Outline mode
6. User can type, delete, use Backspace to merge blocks, and Undo (`Cmd+Z`)
7. Auto-save writes the `.fvm` file; the saved file round-trips correctly
8. CI passes on a fresh `main` push

---

### PI 1 Risk Register (ROAM)

| # | Risk | Probability | Impact | Status | Mitigation |
|---|------|-------------|--------|--------|------------|
| R1 | `cosmic-text` API changes or is unstable | Medium | High | **Mitigate** | Pin to specific commit/tag; wrap behind `LayoutEngine` trait |
| R2 | WASM binary too large (>5MB after wasm-opt) | Medium | High | **Mitigate** | Measure in Sprint 3 (S3-10); options: lazy font loading, split crate |
| R3 | Canvas 2D render performance too slow for delta updates | Low | High | **Own** | Measure in Sprint 4; WebGL fallback is a post-v1.0 concern |
| R4 | `postcard` zero-copy WASM memory complex to implement | Medium | Medium | **Mitigate** | Time-box to 2 days; JSON fallback is acceptable for PI 1 |
| R5 | `cosmic-text` integration (S3-2) takes >1 week | Medium | High | **Own** | S3-2 is on the critical path; flag on Day 1 of Sprint 3 |
| R6 | Round-trip invariant fails for complex nested blocks | Low | High | **Resolve** | FORMAT.md is the spec; if ambiguous, update FORMAT.md first |
| R7 | Sprint velocity significantly lower than 35–40pts | High | Medium | **Accept** | Calibrate after Sprint 1; scope Sprint 2 accordingly |

---

## Full Product Backlog — v1.0

### Prioritisation Key
- **P0** — PI 1 (Walking Skeleton)
- **P1** — PI 2 (Design Mode + Style/Theme)
- **P2** — PI 3 (Export + Versioning)
- **P3** — PI 4 (Content Library + Extensions)
- **P4** — PI 5 (Polish + Platform)

---

### Epic A — Workspace Bootstrap ✅ PI 1

### Epic B — Document Model & Format ✅ PI 1

### Epic C — Text Engine ✅ PI 1 (core); PI 2 (full keyboard map)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| C-1 | Full `InputHandler` — all UX.md keyboard shortcuts mapped to `EditOperation` | 8 | PI 2 | `Cmd+1–6` heading, `Cmd+B/I/backtick/./,` inline, block move shortcuts |
| C-2 | Cut/copy/paste — structured paste from folivm clipboard, plain-text paste with style prompt | 5 | PI 2 | Clipboard API; folivm clipboard format is `application/x-folivm-blocks` |
| C-3 | Find within document — `Cmd+F`, find bar at canvas bottom, regex + literal, case-sensitive, match highlighting | 5 | PI 2 | |
| C-4 | Token substitution — resolve `{title}`, `{author}`, `{date}`, `{page}`, `{pages}`, `{version}` at render time | 3 | PI 2 | Tokens read from `Frontmatter` |

---

### Epic D — Layout Engine ✅ PI 1 (Outline); PI 2 (Design mode)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| D-1 | `PageLayout` — paginate blocks to pages, account for margins + header/footer zones | 8 | PI 2 | |
| D-2 | Header/footer layout — three-zone (left/centre/right) per page, with token substitution | 5 | PI 2 | |
| D-3 | Page break before `Section` when `page-break-before: true` | 2 | PI 2 | |
| D-4 | Tab stop layout — stops defined in theme, applied in layout and shown in ruler | 3 | PI 2 | |

---

### Epic E — Render System ✅ PI 1 (Outline); PI 2 (Design mode)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| E-1 | Design Mode `FrameRenderer` — paginated canvas, margin guides, ruler, zoom | 8 | PI 2 | |
| E-2 | Horizontal ruler — margin drag handles, tab stop markers, all in pt; derives from `ScaleFactor` | 5 | PI 2 | |
| E-3 | Margin guides — visual overlays at page margin boundaries | 2 | PI 2 | |
| E-4 | Zoom control — `Cmd++/-/0`, scroll-to-zoom, zoom % in status bar, 50%–200% range | 3 | PI 2 | |
| E-5 | Focus mode — dim blocks outside active section | 3 | PI 2 | `Cmd+Shift+F` |
| E-6 | ARIA / accessibility tree — `AccessibilityNode` tree posted to shell; basic ARIA roles for headings, paragraphs, lists | 5 | **PI 2** | Moved from PI 5; must scaffold before UI is considered "done" for accessibility |
| E-7 | Full ARIA compliance — figures, tables, reading order, screen reader testing (VoiceOver, NVDA) | 5 | PI 5 | Full testing deferred; scaffolding in PI 2 |

---

### Epic F — Style System (PI 2)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| F-1 | `SetBlock` + `SetInline` operations wired to UI | 4 | PI 2 | Already in `EditOperation`; wire to pickers |
| F-2 | Block style picker — `Cmd+/`, keyboard-searchable, lists all block styles from active theme | 5 | PI 2 | Must be mouse-free |
| F-3 | Inline style picker — appears on text selection, lists inline styles | 3 | PI 2 | |
| F-4 | Insert picker — `/` at block start, filterable list of block types and styles | 5 | PI 2 | |
| F-5 | Paste normalisation — strip formatting from external paste, prompt for target style | 3 | PI 2 | |

---

### Epic G — Theme System (PI 2)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| G-1 | `.fvm-theme` parser — YAML, all four collections (Primitives, Semantic, Brand, Spacing) | 5 | PI 2 | `theme:read` Tauri command |
| G-2 | Alias resolution — `{Collection.variable-name}` chains, max 5 deep, cycle detection | 3 | PI 2 | |
| G-3 | Style definition parsing — extends chain, block + inline style specs | 5 | PI 2 | |
| G-4 | Default bundled theme — ships with application, covers all block/inline styles | 5 | PI 2 | |
| G-5 | Theme application to `LayoutEngine` — style name → font, size, spacing, colour | 5 | PI 2 | |
| G-6 | Live mode switching — Screen/Print/Brand/Spacing; full re-render `<300ms` | 3 | PI 2 | |
| G-7 | Variables panel UI — right rail, mode dropdowns per collection | 3 | PI 2 | |
| G-8 | Styles panel UI — right rail, all styles with preview | 3 | PI 2 | |
| G-9 | Theme validation — contrast ratio warnings, missing variable warnings | 3 | PI 2 | |
| G-10 | File watcher — `theme:updated` event on disk change → live reload | 3 | PI 2 | |
| G-11 | Callout icons — Font Awesome Free subset bundled, icon per callout style | 2 | PI 2 | |

---

### Epic H — Application Shell Frame (PI 2)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| H-1 | Native menu bar — File, Edit, View, Format, Insert, Versioning, Extensions, Help | 5 | PI 2 | Most commands stub initially |
| H-2 | Tabbed documents — up to 20, modified indicator, `Cmd+1–9` switching | 5 | PI 2 | |
| H-3 | Activity bar + sidebar panels — Explorer, Search, Versioning, Extensions, TOC icons | 3 | PI 2 | |
| H-4 | TOC panel — auto-generated from headings, click to navigate | 3 | PI 3 | |
| H-5 | HiDPI awareness — `dpi:changed` event → `ScaleFactor` update → canvas re-render | 3 | PI 2 | |
| H-6 | Mode switch UI — `Outline | Design` segmented button, `Cmd+Shift+O/D` | 2 | PI 2 | |
| H-7 | Empty states — document, project, library, search, version history | 3 | PI 4 | All copy defined in UX.md |
| H-8 | Error states — save fail toast, git fail, export fail, theme not found banner | 3 | PI 3 | All states defined in UX.md |
| H-9 | Keyboard shortcut reference modal — `Help` menu, full UX.md table | 2 | PI 5 | |
| H-10 | Placeholder cell renderers — `cell:data`, `cell:ai`, `cell:diagram`, `cell:citation`, `cell:signature` shown as labelled boxes | 3 | PI 2 | Correct height; warning border; not error state |

---

### Epic I — Export Pipeline (PI 3)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| I-1 | `ExportDocument` type + Stage 1 resolution — token substitution, `cell:include` inline, theme applied | 5 | PI 3 | Runs in native process; not WASM |
| I-2 | PDF export — `cosmic-text` native layout → `printpdf`, positioned glyphs, font subset embedding | 13 | PI 3 | Same font bytes as editor; identical line breaks |
| I-3 | PDF/UA — structure tags: heading hierarchy, figure alt text, reading order, document language | 8 | PI 3 | Non-negotiable |
| I-4 | DOCX export — `docx-rs`, named styles → DOCX styles, Headings → DOCX Heading 1–6 | 8 | PI 3 | |
| I-5 | DOCX accessibility — `w:descr` alt text, `w:lang`, heading mapping | 3 | PI 3 | |
| I-6 | `cell:math` → OMML in DOCX | 3 | PI 3 | |
| I-7 | Export options dialog — format, page range, PDF/UA flag, embed fonts | 3 | PI 3 | |
| I-8 | Streaming progress — `export:progress` events → slim toolbar progress bar | 3 | PI 3 | |
| I-9 | Extension pre-export hooks — Stage 2 of pipeline, registered transform hooks called in order | 3 | PI 4 | |

---

### Epic J — Versioning / Git (PI 3)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| J-1 | Git integration — `git2` crate, repo init on project create | 3 | PI 3 | |
| J-2 | `document:save_version` — dirty auto-save first, then git commit; `Cmd+Shift+S` flow | 5 | PI 3 | |
| J-3 | `versioning:list` — all commits, `Vec<VersionEntry>` | 2 | PI 3 | |
| J-4 | Version history panel — reverse chronological, message + author + date + hash | 3 | PI 3 | |
| J-5 | Read-only version preview — history entry → new tab, labelled, exportable | 3 | PI 3 | |
| J-6 | `versioning:create_draft` — `git checkout -b {name}`, spaces → hyphens | 2 | PI 3 | |
| J-7 | Drafts panel + Switch draft — dirty-save prompt, `git checkout` | 5 | PI 3 | |
| J-8 | Combine draft (merge) — `git merge`, fast path for no conflicts | 3 | PI 3 | |
| J-9 | `versioning:diff` — block-level diff between two commits | 8 | PI 3 | |
| J-10 | Diff view UI — added/removed/modified with word-level highlighting | 8 | PI 3 | |
| J-11 | Conflict blocks UI — both versions shown, Accept button; all must resolve before Save Version | 5 | PI 3 | |
| J-12 | External change detection — `file:changed` event → reload prompt | 2 | PI 3 | |
| J-13 | Author attribution — human / `folivm auto-update` / `{ext} via folivm` | 2 | PI 3 | |

---

### Epic K — Search & Replace (PI 2/3)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| K-1 | In-document find — `Cmd+F`, find bar at canvas bottom, prev/next, regex + literal, case toggle | 5 | PI 2 | |
| K-2 | `search:query` Tauri command — ripgrep wrapper, streaming `SearchResult` events | 5 | PI 3 | |
| K-3 | Project-wide search panel — Activity Bar, streaming results grouped by file | 5 | PI 3 | |
| K-4 | Project-wide replace — preview dialog, per-file deselect, atomic apply, single git commit | 8 | PI 3 | |

---

### Epic L — Content Library (PI 4)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| L-1 | Library index — project `.folivm/library/` + global OS app support directory | 3 | PI 4 | |
| L-2 | `library:list` + `library:resolve` Tauri commands | 5 | PI 4 | Version-pinned resolution |
| L-3 | Library panel UI — Activity Bar book icon, project + global collapsible trees | 3 | PI 4 | |
| L-4 | Insert from library — double-click/Enter → `cell:include` block at cursor | 3 | PI 4 | |
| L-5 | `cell:include` renderer — inline-resolved, version-pinned | 5 | PI 4 | |
| L-6 | Version update indicator + explicit accept flow — creates `folivm auto-update` git commit | 5 | PI 4 | |
| L-7 | Save selection as library item — right-click dialog (name, description, version, project/global) | 5 | PI 4 | |
| L-8 | Edit library item — opens `.fvm` fragment as tab; save increments minor version | 3 | PI 4 | |
| L-9 | Library item hover preview — 500ms tooltip, first 3 blocks | 2 | PI 4 | |

---

### Epic M — Cell Type Renderers (PI 4)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| M-1 | `cell:image` renderer — raster + inline SVG, alt text required, `{fig}` token, caption, width/align | 8 | PI 4 | |
| M-2 | `cell:math` renderer — LaTeX block + inline, bundled KaTeX WASM | 8 | PI 4 | |

---

### Epic N — Extension Host (PI 4)

> **Risk note:** Deno Core + Tauri is the second-riskiest integration in the project. Spike in Sprint 16 Day 1.

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| N-1 | Deno Core runtime — one V8 isolate per extension; `deno_core` crate; spike on Day 1 of PI 4 | 8 | PI 4 | |
| N-2 | Extension sandbox — no direct folivm-core access, no arbitrary Tauri commands | 3 | PI 4 | |
| N-3 | Permission declaration + user approval on install | 3 | PI 4 | |
| N-4 | `folivm.*` JS API surface per EXT.md spec | 8 | PI 4 | |
| N-5 | Cell type handler registration — `folivm.cells.register(type, renderFn, exportFn)` | 5 | PI 4 | |
| N-6 | UI panel registration — sandboxed iframe, message-passing to Deno runtime | 5 | PI 4 | |
| N-7 | Export hooks — `folivm.export.hook(fn)` called in export Stage 2 | 3 | PI 4 | |
| N-8 | Lifecycle hooks — `document:opened`, `document:closed`, `application:quit` | 2 | PI 4 | |
| N-9 | Extension install from `.fvmext` — parse manifest, copy, register, restart isolate | 3 | PI 4 | |
| N-10 | Enable/disable without uninstall | 1 | PI 4 | |
| N-11 | Extension developer console — `Cmd+Shift+I`, log per extension | 3 | PI 4 | |
| N-12 | Extension error indicator — Activity Bar badge, panel error state | 2 | PI 4 | |
| N-13 | First-party example extension — functional and installable (v1.0 success criterion 7) | 5 | PI 4 | Recommend: `cell:diagram` renderer (clear scope, demonstrates renderFn + exportFn) |

**PI 4 Risk Register:**

| # | Risk | Status | Mitigation |
|---|------|--------|------------|
| N-R1 | Deno Core + Tauri WebView conflict (both use V8) | **Mitigate** | Spike Day 1; Deno Core runs in native process, not WebView — should be isolated |
| N-R2 | Deno Core version incompatible with Tauri's dependency graph | **Mitigate** | Check compatibility before committing to PI 4 backlog |
| N-R3 | `folivm.*` API surface scope creep | **Own** | API surface is frozen by EXT.md v0.1; any addition requires EXT.md update first |

---

### Epic O — Platform & Release (PI 5)

| # | Story | Points | PI | Notes |
|---|-------|--------|----|-------|
| O-1 | macOS Universal binary (Apple Silicon + Intel) | 3 | PI 5 | |
| O-2 | Windows MSI + NSIS installer | 5 | PI 5 | |
| O-3 | Linux `.deb` + AppImage | 3 | PI 5 | |
| O-4 | Performance: project-wide find-replace on 100+ files `<5s` | 5 | PI 5 | Benchmark + optimise |
| O-5 | Measurement fidelity: cursor at left margin ±0.5pt | 3 | PI 5 | Scale factor accuracy test |
| O-6 | `<300ms` theme mode switch — measure + optimise | 3 | PI 5 | |
| O-7 | Full ARIA compliance + screen reader testing (E-7) | 5 | PI 5 | VoiceOver + NVDA |
| O-8 | v1.0 acceptance: Legal professional workflow (keyboard-only, PDF/UA export) | 3 | PI 5 | SCOPE.md criterion 1 |
| O-9 | v1.0 acceptance: Consultant workflow (Outline → Design → brand theme → PDF) | 2 | PI 5 | SCOPE.md criterion 2 |
| O-10 | v1.0 acceptance: Technical writer workflow (library + images + equations + export) | 2 | PI 5 | SCOPE.md criterion 3 |
| O-11 | Crash reporter + opt-in telemetry | 3 | PI 5 | |

---

## Backlog Summary by PI

| PI | Theme | Est. Stories | Est. Points | Notes |
|----|-------|-------------|-------------|-------|
| PI 1 | Walking Skeleton | ~50 | ~196 | Velocity calibration sprint |
| PI 2 | Design Mode + Style/Theme | ~38 | ~165 | Includes ARIA scaffolding (E-6) |
| PI 3 | Export + Versioning + Search | ~36 | ~140 | PDF export is the highest-risk story |
| PI 4 | Content Library + Extensions | ~32 | ~140 | Deno Core spike drives PI 4 schedule |
| PI 5 | Polish + Platform + Acceptance | ~20 | ~65 | — |
| **Total** | | **~176** | **~706** | |

---

## PI 2–3 Sprint Sketches (for visibility)

**PI 2 Sprints (Pivoted):**

| Sprint | Focus |
|--------|-------|
| S6 | **Recovery**: Text engine implementation (S2-1 → S2-8), Undo/Redo recovery |
| S7 | **Recovery**: Layout engine implementation (S3-1 → S3-10), cosmic-text wiring |
| S8 | **Shell Integration**: WASM bindings, canvas render instructions, IME support |
| S9 | **Design Preview**: Theme system (G-1), Style pickers (F-1), `PageLayout` pagination (D-1) |

**PI 3 Sprints:**

| Sprint | Focus |
|--------|-------|
| S10 | Export foundation (I-1), PDF pipeline (I-2) |
| S11 | PDF/UA (I-3), DOCX export (I-4–I-6), export dialog + progress (I-7–I-8) |
| S12 | Git integration (J-1–J-5), save version flow, history panel |
| S13 | Diff view (J-9–J-11), conflicts UI, project-wide search (K-2–K-3), replace (K-4) |

---

## Key Invariants (Enforce Throughout All PIs)

1. **No ad-hoc formatting** — `Block` and `Inline` have no font/size/colour fields. Reject any story that adds them.
2. **Measurement unit is pt** — All layout in points. `px` is forbidden in the layout path.
3. **Single scale factor** — Ruler, canvas, and margin guides all read from `ScaleFactor`. No duplicates.
4. **Atomic writes** — All file writes (save, export) via temp + rename.
5. **Extension isolation** — Extensions in Deno Core isolates only; no direct folivm-core access.
6. **Round-trip invariant** — `parse(serialise(model)) == model`. Never regress.
7. **One text layout path** — `cosmic-text` only. Editor (WASM) and export (native) share the same font bytes.

---

## Critical Files (When Implementation Begins)

| File | Purpose |
|------|---------|
| `crates/folivm-core/src/model/document.rs` | `DocumentModel`, `Block`, `Inline`, `EditOperation` |
| `crates/folivm-core/src/parser/body.rs` | Custom block-level tokeniser |
| `crates/folivm-core/src/parser/cell.rs` | Cell fence + YAML metadata parsing |
| `crates/folivm-core/src/parser/serializer.rs` | `DocumentModel → .fvm` string (canonical) |
| `crates/folivm-core/src/text_engine/buffer.rs` | `RunBuffer` |
| `crates/folivm-core/src/text_engine/input.rs` | `InputHandler` |
| `crates/folivm-core/src/layout/engine.rs` | `LayoutEngine` (cosmic-text wrapper) |
| `crates/folivm-core/src/layout/scale.rs` | `ScaleFactor` — single source of truth |
| `crates/folivm-core/src/render/instructions.rs` | `RenderInstruction` enum |
| `crates/folivm-core/src/render/frame.rs` | `FrameRenderer` |
| `crates/folivm-wasm/src/lib.rs` | `FolivmInstance` WASM API surface |
| `shell/src/canvas.ts` | Canvas 2D instruction executor + glyph atlas |
| `shell/src/ime.ts` | IME composition bridge |
| `shell/src/ipc.ts` | Tauri command wrappers (ts-rs typed) |
| `crates/folivm-native/src/commands/document.rs` | `document:read`, `document:save` |
| `docs/FORMAT.md` | Ground truth for parser/serialiser |
| `docs/HLD.md` | Component boundary — consult before crossing crate lines |
