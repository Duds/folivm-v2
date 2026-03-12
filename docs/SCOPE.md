# folivm — v1.0 Scope
**Version:** 1.0 (draft)
**Date:** 2026-03-13
**Status:** In review
**Depends on:** CONCEPT.md v0.4, FR.md v0.1, HLD.md v0.3

---

## 1. Purpose

This document defines exactly what ships in folivm v1.0 and what does not. It exists because scope ambiguity killed folivm v1. Every feature decision that is not in this document requires an explicit scope change — not a quiet addition.

The guiding question for every item: **does this make folivm genuinely useful to the primary personas on day one, and does it differentiate folivm from Word and Google Docs?** If yes to both, it is a candidate for v1.0. If either answer is no, it is deferred.

---

## 2. Scope Principles

**Ship the core, not the complete.** A word processor that does the fundamental job superbly is more valuable than one that does everything adequately. folivm v1.0 is the best-in-class tool for structured professional document production on the desktop. Everything else comes later.

**Extensions are not scope escapes.** "That can be an extension" is not a reason to omit something from the core. Extensions that require core capabilities not present in v1.0 cannot ship. The extension API must be real and functional, not a placeholder.

**The format is complete.** FORMAT.md v1.0 is the specification. The parser supports all cell types. Unrendered cells show a placeholder — they are not ignored. A document written in folivm v1.0 is forward-compatible with future versions.

**No half features.** A feature either works correctly in both Outline and Design mode, exports correctly to PDF and DOCX, and handles edge cases, or it does not ship. A feature that works "mostly" is a bug, not a feature.

---

## 3. In Scope — v1.0

### 3.1 Core editor

| Feature | Notes |
|---|---|
| All standard block types | Paragraph, Heading (H1–H6), List (ordered + unordered + nested), Table, Blockquote |
| Callout blocks | `:::StyleName` fence, any theme-defined style with or without icon |
| Section blocks | `:::section` fence with id, title, numbered, page-break-before attributes |
| Full inline syntax | Built-in: Emphasis, Strong, Code, Superscript, Subscript. Named: `[text]{.StyleName}` |
| Token substitution | `{title}`, `{author}`, `{date}`, `{page}`, `{pages}`, `{version}`. Extension tokens rendered as placeholder if source unavailable |
| Keyboard-driven editing | Complete keyboard shortcut map for all editing operations |
| Undo / redo | Within-session, operation-level, with word-level coalescing |
| Cut / copy / paste | Paste from folivm (structured), paste from external (plain text → style prompt) |
| Find within document | Regex and literal, case-sensitive option, match highlighting |
| IME support | Full composition event handling for CJK and other input methods |
| Block identity | All blocks carry stable UUIDs, preserved across saves |

### 3.2 Outline mode

| Feature | Notes |
|---|---|
| Keyboard-first navigation | Arrow keys, Tab/Shift-Tab, block movement shortcuts |
| Style badge per block | Every block shows its applied style name |
| Cell type indicator | Cell blocks show their type icon |
| Word count per block | Live count on the active block; document total in status bar |
| Section folding | Fold/unfold any section block |
| Focus mode | Narrows view to one section, greys the rest |
| Mode switch | Instant, lossless, no re-parse |

### 3.3 Design mode

| Feature | Notes |
|---|---|
| Physical page layout | Accurate page dimensions for A4, USLetter, A5, Legal |
| Ruler | Horizontal ruler with margin guides, tab stop indicators, all in pt |
| Measurement fidelity | Single scale factor (`canvas_width_px / page_width_pt × dpi/96 × zoom`), uniform across ruler, canvas, margins |
| Margin guides | Visual overlays at exact margin positions |
| Zoom | 50%–200%, keyboard shortcuts, scroll-to-zoom |
| Headers and footers | Rendered per page, token substitution, three-zone (left/centre/right) |
| Page break indicators | Visual separator between pages |
| Pagination | Blocks flow across pages; page count shown in status bar |
| Mode switch | Instant, lossless, same document |

### 3.4 Style system

| Feature | Notes |
|---|---|
| Block style picker | Prominent in both modes, keyboard accessible, searchable |
| Inline style picker | Selection-triggered, shows applicable inline styles from active theme |
| Style application | Keyboard shortcut for every style in the active theme |
| No ad-hoc formatting | No font picker, no colour picker, no raw bold/italic buttons as primary controls |
| Style badge | Applied style visible on every block in Outline mode |
| Paste normalisation | External paste strips formatting, prompts for style assignment |

### 3.5 Theme system

| Feature | Notes |
|---|---|
| Collections | Primitives, Semantic, Brand, Spacing — all four |
| Variable types | Color, Number, String, Boolean |
| Alias resolution | `{Collection.variable-name}` resolved at theme load |
| Modes | Screen/print (Semantic), client modes (Brand), normal/compact (Spacing) |
| Mode switching | Live, no document reload — all blocks re-render |
| Style inheritance | `extends` chain, up to five levels, resolved at theme load |
| Callout icons | Font Awesome Free subset (bundled, offline), SVG rendering |
| Theme loading | From `.fvm-theme` file, relative path from document |
| Theme file watching | Live reload when `.fvm-theme` changes on disk |
| Default theme | Bundled with the application, used when no theme is specified |
| Variables panel | Read-only display of active modes per collection. Mode switching UI |
| Styles panel | List of all styles in active theme with preview |
| Theme validation | Contrast ratio warnings for color token pairs, missing alias warnings |

### 3.6 Cell types (v1.0 renderers)

| Cell type | v1.0 status | Notes |
|---|---|---|
| `cell:image` | **Full renderer** | Raster (external src) and inline SVG. Alt text required. `{fig}` token. Caption. Width, align |
| `cell:math` | **Full renderer** | LaTeX syntax. Block and inline display. Rendered via bundled KaTeX |
| `cell:include` | **Full renderer** | Version-pinned Content Library reference. Inlines resolved blocks. Update indicator |
| `cell:data` | Placeholder | Renders field references as `[source.field]`. Resolved only if data source extension installed |
| `cell:ai` | Placeholder | Renders prompt and status. Content body rendered if status is `accepted`. Generation requires extension |
| `cell:diagram` | Placeholder | Renders source preview and "renderer extension required" indicator |
| `cell:citation` | Placeholder | Renders `[citation: key]`. Formatting requires citation extension |
| `cell:signature` | Placeholder | Renders label and "unsigned" indicator. Collection requires signature extension |

The parser reads and preserves all cell types. Cells without a v1.0 renderer are displayed as labelled placeholders, not errors. A document with placeholder cells is a valid document.

### 3.7 Project workspace

| Feature | Notes |
|---|---|
| Project folder | Open a directory as a project. All `.fvm` and `.fvm-theme` files in the directory tree are the project |
| File explorer | Sidebar tree of project files and directories |
| Tabbed open documents | Up to 20 simultaneously open documents |
| Modified indicator | Tab shows unsaved changes |
| Project-wide find | Full-text search across all `.fvm` files in the project (ripgrep, streaming results) |
| Project-wide replace | Multi-file replace with preview, per-file deselect, single git commit |
| Recent projects | Last 10 projects in File menu |

### 3.8 Content Library

| Feature | Notes |
|---|---|
| Library panel | Sidebar panel listing all library items (project-level and global-level) |
| Project library | `.folivm/library/` in the project directory |
| Global library | OS application support directory |
| Insert from library | Drag or double-click inserts a `cell:include` reference |
| Version pinning | `version: "N.N"` in `cell:include` — exact version required |
| Version update indicator | Shown when a newer version of a referenced item is available |
| Explicit update accept | Update changes the version pin and creates a git commit |
| Library item creation | Save any selection of blocks as a new library item with a version |
| Library item editing | Edit items directly as `.fvm` fragment files |

### 3.9 Versioning

| Feature | Notes |
|---|---|
| Auto-save | 30-second debounce. Writes `.fvm` to disk. Does not create a git commit |
| Save Version | User-initiated. Requires a message. Creates a git commit |
| Version history | Panel listing all versions (commits) with message, author, date |
| Create Draft | Creates a git branch. Named by the user |
| Switch Draft | Checkout a different branch. Prompts to save if dirty |
| List Drafts | Panel listing all branches |
| Diff view | Compare any two versions. Displays added/removed/changed blocks |
| Conflict display | If a merge produces conflicts, each conflict is shown as a diff block for user resolution |
| Author attribution | Human edits: user's configured name. Library updates: `folivm auto-update`. ai-accepted content: `{extension} via folivm` |
| External change detection | File watcher detects external edits. Prompts to reload |

Track changes as an inline editing mode (Word-style redline) is **not in v1.0**. The versioning diff view provides version-to-version comparison. Inline track changes is v1.1.

### 3.10 Export

| Feature | Notes |
|---|---|
| PDF export | cosmic-text layout → printpdf. Identical line breaks to editor. PDF/UA structure tags |
| DOCX export | docx-rs. Named styles → DOCX paragraph/character styles. Headings → DOCX Heading 1–6 |
| Export options dialog | Page range, include comments (future), accessible PDF flag |
| Font embedding | Fonts embedded as subsets in PDF |
| PDF/UA | Tagged PDF with real heading structure, figure alt text, reading order, document language |
| DOCX accessibility | Heading mapping, image alt text via `w:descr`, document language `w:lang` |
| Export progress | Streaming progress indicator for large documents |
| Atomic write | PDF and DOCX written to temp file, renamed on completion. Failure leaves original intact |

### 3.11 Extension host

| Feature | Notes |
|---|---|
| Deno Core runtime | One isolated runtime per extension |
| Local install | Install from `.fvmext` file. No marketplace in v1.0 |
| Permission declaration | Extensions declare permissions in manifest. User approves on install |
| Cell type handlers | Register render and export handlers for any cell type |
| UI panels | Register sidebar panels (HTML, rendered in sandboxed iframe) |
| Export hooks | Register pre-export transforms on `ExportDocument` |
| Data sources | Register a data source to resolve `cell:data` tokens |
| Library contributions | Extensions may contribute library items |
| Extension lifecycle hooks | `document:opened`, `document:closed`, `application:quit` |
| Enable / disable | Extensions can be enabled/disabled without uninstalling |
| Extension developer console | Log output visible in a developer panel |

### 3.12 Application shell

| Feature | Notes |
|---|---|
| Native window | Tauri. Custom title bar on macOS. Standard frame on Windows and Linux |
| Native menus | Full menu bar: File, Edit, View, Format, Insert, Versioning, Extensions, Help |
| Native file dialogs | OS open/save dialogs |
| Status bar | Word count, page count, cursor position, active mode, zoom level |
| Activity bar | Icons for Explorer, Search, Versioning, Extensions, TOC |
| TOC panel | Auto-generated from document headings. Click to navigate |
| Display DPI awareness | Correct rendering on HiDPI displays and on display change |

### 3.13 Platform targets

| Platform | v1.0 |
|---|---|
| macOS (Apple Silicon + Intel) | Universal binary |
| Windows 10 / 11 (x86-64) | MSI + NSIS installer |
| Linux (x86-64) | .deb + AppImage |

---

## 4. Out of Scope — v1.0

### 4.1 Deferred to v1.1

These features are planned but do not block a useful v1.0:

| Feature | Reason deferred |
|---|---|
| Inline track changes (Word-style redline) | Complex UI; versioning diff provides version-to-version comparison |
| Comments / margin annotations | Requires annotation storage format (addendum to FORMAT.md) |
| Section auto-numbering | Needs numbering scheme configuration; manual numbering works for v1.0 |
| Table of contents generation | Requires cross-document anchor resolution |
| Cross-reference resolution | `{section.id}` token resolution across documents |
| Footnotes / endnotes | Requires pagination-aware layout pass |
| Custom page sizes beyond presets | Custom width/height in frontmatter is parsed; UI for setting it deferred |
| Hyphenation | cosmic-text supports it; needs dictionary data and UI opt-in |
| Multi-column layout | Significant layout engine work |
| Print directly | Export to PDF, print from OS. Direct print API deferred |
| Spell check | Requires OS spell API integration or bundled dictionary |

### 4.2 Extension territory (not core, not deferred — always extensions)

These will never be in the folivm core regardless of version:

| Feature | Why extension |
|---|---|
| AI content generation | `cell:ai` rendering is core; generation is provider-specific |
| CRM / data source integration | `cell:data` rendering is core; data sources are provider-specific |
| Citation management | Citation formatting is citation-style-specific (Zotero, Mendeley, etc.) |
| Diagram rendering | Renderer is syntax-specific (Mermaid, PlantUML, etc.) |
| Digital signatures | Workflow is platform-specific (DocuSign, AdobeSign, etc.) |
| Grammar / style checking | External service or local model |
| DITA / XML export | Specialist format requiring document mapping |
| Web publishing | Not a core export target |
| Mail merge execution | Data population is data-source-specific |

### 4.3 Post-launch (v2.x or cloud milestone)

| Feature | Milestone |
|---|---|
| Cloud / web version | After desktop v1.0 ships and stabilises |
| Multi-user real-time collaboration | Requires CRDT document model; major architectural addition |
| Extension marketplace | After sufficient extension ecosystem develops |
| Team / enterprise admin console | Cloud milestone |
| SSO / SAML integration | Cloud milestone |
| Audit logging for compliance | Cloud milestone |
| Mobile (iOS / Android / iPadOS) | Long-term; different input model entirely |

### 4.4 Will not do

| Feature | Reason |
|---|---|
| Ad-hoc formatting (font picker, colour picker) | Violates semantic styling constraint. Removes the primary architectural differentiator |
| Binary document format | Violates ASCII-first constraint |
| Table formulas / spreadsheet | Out of product scope. `cell:data` handles data; computation is extension territory |
| Built-in AI (no extension) | Core product has no AI dependency. This is a product principle, not a scope decision |
| Read-only modes | Both Outline and Design modes are fully editable. This was resolved in product definition |

---

## 5. Extension API Completeness Requirement

v1.0 ships with a functional extension API. This means at least one published extension exists and is tested against the API before release. The extension API is not a placeholder.

The minimum viable extension API for v1.0:
- A working Zotero citation extension **or** a working Mermaid diagram extension demonstrates the cell type handler API
- A working CRM stub extension (CSV data source) demonstrates the data source API
- Extensions are installable from `.fvmext` files without a marketplace

If the extension API cannot support these examples, the extension host is not ready and the feature is not in v1.0.

---

## 6. Quality Bar for v1.0

A feature is not complete until it meets all of these conditions:

| Criterion | Standard |
|---|---|
| Both modes | Works correctly in Outline mode and Design mode |
| Export | Renders correctly in PDF export and DOCX export |
| Round-trip | `parse(serialise(model)) == model` for any document using the feature |
| Accessibility | ARIA shadow tree correctly represents the feature's semantic content |
| Measurement fidelity | In Design mode, all measurements are accurate to ±0.5pt |
| No data loss | Any operation that could lose content requires explicit user confirmation |

---

## 7. v1.0 Success Criteria

folivm v1.0 is ready to ship when:

1. A legal professional can open a project, use a firm clause library via `cell:include`, populate a contract template, apply a firm theme, and export a PDF/UA-compliant PDF — without touching the mouse for the editing workflow
2. A consultant can write a proposal in Outline mode, switch to Design mode to verify layout, apply a client brand theme, and produce a PDF in the client's brand — and switch to a second client brand by changing one mode selection
3. A technical writer can assemble a manual from Content Library sections, embed images and equations, apply a style system, and export to both PDF and DOCX with correct heading structure in both outputs
4. Project-wide find-and-replace works correctly across 100+ `.fvm` files in under 5 seconds
5. The measurement fidelity test passes: cursor at left margin in Design mode is within ±0.5pt of the margin value
6. The round-trip invariant holds for the complete example document in FORMAT.md
7. At least one extension is functional and installable

---

## 8. Scope Change Process

Any addition to the v1.0 scope requires:
1. An explicit entry in this document under the correct section
2. A corresponding update to FR.md with P0 priority
3. A note on what is deferred to accommodate the addition (scope is fixed; adding requires removing)

Scope additions are not made during implementation. If a feature is discovered to be necessary during implementation, it is either already in this document or it is deferred.

---

*This document is the scope commitment for folivm v1.0. It is not a roadmap. Roadmap lives in a separate document.*
