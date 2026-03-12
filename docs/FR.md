# folivm — Functional Requirements
**Version:** 0.1 (draft)
**Date:** 2026-03-12
**Status:** In review
**Depends on:** CONCEPT.md v0.3

---

## Conventions

**Requirement IDs** follow the pattern `FR-[MODULE]-[NNN]`.

**Priority:**
- `P0` — Must have. v1.0 does not ship without it.
- `P1` — Should have. v1.0 is significantly weaker without it.
- `P2` — Nice to have. Targeted for v1.1.

**Modules:**
| Code | Module |
|---|---|
| SHELL | Application shell, chrome, navigation |
| PROJ | Project and workspace management |
| FMT | The `.fvm` file format |
| OUTLINE | Outline mode |
| DESIGN | Design mode and page canvas |
| STYLE | Semantic styling system |
| THEME | Theme system (`.fvm-theme`) |
| SEARCH | Find and replace |
| LIB | Content library |
| VER | Versioning and track changes |
| EXPORT | Export (PDF, DOCX) |
| EXT | Extension architecture |
| SETTINGS | Application settings |

---

## 1. Application Shell (SHELL)

The persistent chrome wrapping the entire application: title bar, menu, activity bar, sidebar, right rail, status bar, and tab bar.

### 1.1 Title bar

| ID | Priority | Requirement |
|---|---|---|
| FR-SHELL-001 | P0 | The application shall display a native-style title bar showing the current project name and active document filename. |
| FR-SHELL-002 | P0 | The title bar shall include a macOS-style window control area (traffic lights) on macOS and native controls on Windows. |
| FR-SHELL-003 | P0 | The title bar shall include a menu bar with menus: File, Edit, Selection, View, Help. |
| FR-SHELL-004 | P0 | The title bar shall include toggle buttons to show and hide the left sidebar and right rail independently. |
| FR-SHELL-005 | P1 | The title bar shall indicate unsaved changes in the active document via a modified indicator (e.g. dot on the window close button on macOS, asterisk in title on Windows). |

### 1.2 Activity bar

| ID | Priority | Requirement |
|---|---|---|
| FR-SHELL-010 | P0 | The application shall display a vertical activity bar providing navigation between sidebar panels. |
| FR-SHELL-011 | P0 | The activity bar shall include icons for: Explorer, Search, Versioning, Extensions, Document Outline (TOC). |
| FR-SHELL-012 | P0 | Clicking an active activity bar item shall collapse the sidebar. Clicking an inactive item shall expand the sidebar and switch to that panel. |
| FR-SHELL-013 | P0 | The active activity bar item shall be visually distinct from inactive items. |

### 1.3 Left sidebar

| ID | Priority | Requirement |
|---|---|---|
| FR-SHELL-020 | P0 | The left sidebar shall be collapsible and restorable via the title bar toggle and via keyboard shortcut. |
| FR-SHELL-021 | P0 | The sidebar shall render the panel corresponding to the active activity bar selection. |
| FR-SHELL-022 | P1 | The sidebar width shall be user-resizable by dragging its border, with a minimum and maximum width enforced. |
| FR-SHELL-023 | P1 | The sidebar width shall persist across sessions per project. |

### 1.4 Right rail

| ID | Priority | Requirement |
|---|---|---|
| FR-SHELL-030 | P0 | The right rail shall be collapsible and restorable via the title bar toggle and keyboard shortcut. |
| FR-SHELL-031 | P0 | The right rail shall display contextual document and design controls: page settings, header/footer templates, theme variables. |
| FR-SHELL-032 | P0 | The right rail shall switch between Draft and Design tabs, controlling both the right rail content and the editor body mode. |
| FR-SHELL-033 | P1 | The right rail width shall be fixed at a comfortable reading width. It shall not be resizable in v1.0. |

### 1.5 Status bar

| ID | Priority | Requirement |
|---|---|---|
| FR-SHELL-040 | P0 | The status bar shall display at the bottom of the application at all times. |
| FR-SHELL-041 | P0 | The status bar shall show: word count, character count, current section, cursor line and column, file encoding. |
| FR-SHELL-042 | P0 | The status bar shall show the current git branch name. |
| FR-SHELL-043 | P1 | The status bar shall show the current applied style name for the block at cursor position. |
| FR-SHELL-044 | P1 | The status bar shall show a live word count that updates as the user types. |
| FR-SHELL-045 | P2 | The status bar shall show an AI extension sync status indicator when an AI extension is active. |

### 1.6 Tab bar

| ID | Priority | Requirement |
|---|---|---|
| FR-SHELL-050 | P0 | The tab bar shall display one tab per open document. |
| FR-SHELL-051 | P0 | The active tab shall be visually distinct. |
| FR-SHELL-052 | P0 | Each tab shall display the document filename. |
| FR-SHELL-053 | P0 | Each tab shall display a modified indicator (dot or asterisk) when the document has unsaved changes. |
| FR-SHELL-054 | P0 | Clicking a tab shall switch the editor to that document without closing others. |
| FR-SHELL-055 | P0 | Each tab shall have a close button. Closing a tab with unsaved changes shall prompt to save, discard, or cancel. |
| FR-SHELL-056 | P1 | Tabs shall be reorderable by drag and drop. |
| FR-SHELL-057 | P1 | The tab bar shall support keyboard navigation between tabs (Cmd/Ctrl+Tab, Cmd/Ctrl+Shift+Tab). |
| FR-SHELL-058 | P1 | Right-clicking a tab shall show a context menu: Close, Close Others, Close All, Reveal in Explorer. |
| FR-SHELL-059 | P2 | When tabs exceed available width, the tab bar shall scroll or provide a tab overflow menu. |

### 1.7 Keyboard shortcuts

| ID | Priority | Requirement |
|---|---|---|
| FR-SHELL-060 | P0 | The application shall provide keyboard shortcuts for all primary actions. |
| FR-SHELL-061 | P0 | Shortcuts shall follow platform conventions (Cmd on macOS, Ctrl on Windows). |
| FR-SHELL-062 | P1 | Every named style shall be assignable via a keyboard shortcut configurable in settings. |
| FR-SHELL-063 | P1 | Users shall be able to customise keyboard shortcuts through the settings interface. |

---

## 2. Project and Workspace (PROJ)

### 2.1 Project model

| ID | Priority | Requirement |
|---|---|---|
| FR-PROJ-001 | P0 | A project shall be a directory on the local file system containing `.fvm` document files, a project configuration file, an optional `.fvm-theme` file, and an optional content library directory. |
| FR-PROJ-002 | P0 | A project directory shall be a git repository. folivm shall initialise git when creating a new project. |
| FR-PROJ-003 | P0 | The application shall support opening any directory as a project. |
| FR-PROJ-004 | P0 | The application shall maintain a list of recently opened projects accessible from the welcome screen and File menu. |
| FR-PROJ-005 | P1 | A project configuration file (`.folivm/project.yaml`) shall store: project name, default theme path, default page size, default orientation, content library paths. |

### 2.2 Project creation and opening

| ID | Priority | Requirement |
|---|---|---|
| FR-PROJ-010 | P0 | The application shall provide a welcome screen on launch when no project is open, offering: New Project, Open Project, Recent Projects. |
| FR-PROJ-011 | P0 | New Project shall prompt for a project name and location, create the directory, initialise git, and create a default project configuration. |
| FR-PROJ-012 | P0 | Open Project shall present a native OS directory picker. |
| FR-PROJ-013 | P1 | The application shall support opening a project by dragging a project folder onto the application window. |
| FR-PROJ-014 | P2 | The application shall support multiple projects open simultaneously, each in its own window. |

### 2.3 Directory explorer

| ID | Priority | Requirement |
|---|---|---|
| FR-PROJ-020 | P0 | The Explorer sidebar panel shall display the project directory as a navigable file tree. |
| FR-PROJ-021 | P0 | The file tree shall display folders and `.fvm` files. Non-`.fvm` files shall be visible but visually de-emphasised. |
| FR-PROJ-022 | P0 | Folders shall be expandable and collapsible. Expansion state shall persist across sessions. |
| FR-PROJ-023 | P0 | Clicking a `.fvm` file in the tree shall open it in the editor as a new tab (or focus its existing tab). |
| FR-PROJ-024 | P0 | The Explorer panel header shall include actions: New File, New Folder, Refresh. |
| FR-PROJ-025 | P0 | Right-clicking a file or folder shall present a context menu: New File, New Folder, Rename, Delete, Reveal in Finder/Explorer. |
| FR-PROJ-026 | P0 | Renaming a file via the explorer shall update all `cell:include` references to that file within the project and record the change in version history. |
| FR-PROJ-027 | P1 | The file tree shall indicate git status per file (modified, added, untracked, deleted) using colour-coded status badges. |
| FR-PROJ-028 | P1 | Files shall be reorderable within folders by drag and drop. |
| FR-PROJ-029 | P1 | The explorer shall support multi-select for batch operations (delete, move). |
| FR-PROJ-030 | P2 | The explorer shall support nested drag-and-drop to move files between folders. |

---

## 3. The `.fvm` Format (FMT)

### 3.1 Document structure

| ID | Priority | Requirement |
|---|---|---|
| FR-FMT-001 | P0 | A `.fvm` file shall be a valid UTF-8 plain text file readable in any text editor without special software. |
| FR-FMT-002 | P0 | A `.fvm` file shall begin with a YAML frontmatter block delimited by `---`. |
| FR-FMT-003 | P0 | The frontmatter shall support the following document-level properties: `title`, `author`, `date`, `page-size`, `orientation`, `margins` (top/bottom/left/right in mm or pt), `theme`, `header`, `footer`, `tags`. |
| FR-FMT-004 | P0 | All measurement values in the frontmatter shall be expressed in physical units: millimetres (`mm`) or points (`pt`). Pixel values shall not be stored in the format. |
| FR-FMT-005 | P0 | The document body shall be valid extended Markdown following the CommonMark specification, extended with folivm cell block syntax. |
| FR-FMT-006 | P0 | Section divisions shall be expressible using the `:::section` fence block with optional attributes (page-break, orientation, columns). |
| FR-FMT-007 | P1 | The frontmatter shall support per-section header and footer overrides. |
| FR-FMT-008 | P1 | The frontmatter shall support landscape orientation for individual sections (for wide tables or diagrams). |

### 3.2 Cell types

| ID | Priority | Requirement |
|---|---|---|
| FR-FMT-020 | P0 | The format shall support typed cell blocks using the `:::cell:[type]` fence syntax. |
| FR-FMT-021 | P0 | `cell:data` — a deterministic data cell referencing an external source. Attributes: `source`, `fields`. Rendered by the registered data extension; rendered as a labelled placeholder when no extension is installed. |
| FR-FMT-022 | P0 | `cell:ai` — a generative content cell. Attributes: `provider`, `prompt`, `style`, `status` (draft/accepted/rejected). Rendered by the registered AI extension; rendered as a labelled placeholder when no extension is installed. |
| FR-FMT-023 | P0 | `cell:include` — a content library reference. Attributes: `ref` (library item identifier), `version` (semantic version pin). Rendered inline as the library item content. |
| FR-FMT-024 | P1 | `cell:math` — a mathematical equation in LaTeX or AsciiMath syntax. Rendered as a typeset equation in Design mode; rendered as the raw source in Outline mode. |
| FR-FMT-025 | P1 | `cell:diagram` — a diagram definition in Mermaid syntax. Rendered as an SVG diagram in Design mode; rendered as labelled source in Outline mode. |
| FR-FMT-026 | P1 | `cell:citation` — a structured citation reference. Attributes: `key`, `style` (APA, Chicago, MLA). Rendered inline with bibliography management by the registered citation extension. |
| FR-FMT-027 | P1 | `cell:signature` — a signature field. Attributes: `label`, `required`. Rendered as an interactive signature placeholder by the registered signature extension. |
| FR-FMT-028 | P1 | Extensions shall be able to register additional cell types via the extension API. Unrecognised cell types shall render as a labelled placeholder displaying the cell type name and raw attributes. An unrecognised cell type shall never cause an error or prevent the document from opening. |

### 3.3 Images

| ID | Priority | Requirement |
|---|---|---|
| FR-FMT-030 | P0 | The format shall support images via `cell:image` blocks. Raster images (PNG, JPG, WebP, GIF) shall be referenced by relative `src` path. SVG images shall be embeddable inline via an `svg` attribute. Base64 encoding of images shall not be supported. |
| FR-FMT-031 | P0 | `cell:image` shall support the following attributes: `src` (relative path, mutually exclusive with `svg`), `svg` (inline SVG markup), `alt` (required — plain text description for accessibility and AI consumption), `caption` (optional — rendered as a styled caption below the image), `width` (in mm or pt), `align` (left / centre / right), `style` (named theme style for the image frame and caption). |
| FR-FMT-032 | P1 | The `{fig}` token shall be available in `caption` values and shall auto-increment across all `cell:image` blocks with a non-empty caption, in document order. Reordering image cells shall update figure numbers automatically. |
| FR-FMT-033 | P1 | The project shall maintain an `assets/` directory as the conventional location for binary image files. The application shall not enforce this location but shall default to it in the image insertion dialog. |
| FR-FMT-034 | P1 | Adding a binary asset larger than 1MB to the project shall prompt the user to enable git-lfs if not already configured for the repository. |
| FR-FMT-035 | P1 | In Design mode, `cell:image` blocks shall render at the physical dimensions specified by `width`, maintaining aspect ratio. In Outline mode, they shall render as a compact placeholder showing the filename or SVG preview, `alt` text, and caption. |

### 3.4 Template tokens

| ID | Priority | Requirement |
|---|---|---|
| FR-FMT-040 | P0 | The format shall support inline template tokens in the form `{token.path}` in both frontmatter values and document body text. |
| FR-FMT-041 | P0 | Built-in tokens shall include: `{title}`, `{author}`, `{date}`, `{page}`, `{pages}`, `{section}`. |
| FR-FMT-042 | P0 | Data extension tokens (e.g. `{crm.client.name}`, `{csv.row.value}`) shall be resolved by the registered data extension at render and export time. |
| FR-FMT-043 | P1 | Unresolved tokens (no extension registered for the source) shall render visibly as the raw token text in a distinct colour, never silently as empty string. |

---

## 4. Outline Mode (OUTLINE)

Outline mode is the default mode on document open. It is the content-first environment where thinking and drafting happen.

### 4.1 Block rendering

| ID | Priority | Requirement |
|---|---|---|
| FR-OUTLINE-001 | P0 | Outline mode shall render the document as a linear sequence of content blocks — paragraphs, headings, lists, blockquotes, cells — without page chrome, margins, headers, footers, or rulers. |
| FR-OUTLINE-002 | P0 | Every block shall display a style badge showing the name of the applied semantic style (e.g. `ClauseHeading`, `BodyText`). |
| FR-OUTLINE-003 | P0 | Every block shall display a cell type indicator for non-standard blocks: `cell:ai`, `cell:data`, `cell:math`, `cell:include`, etc. |
| FR-OUTLINE-004 | P0 | Heading blocks (H1, H2, H3) shall visually establish document hierarchy through indentation and weight, not through decorative styling. |
| FR-OUTLINE-005 | P0 | The frontmatter YAML block shall be rendered as an editable key-value block at the top of the outline, distinct from content blocks. |

### 4.2 Section folding

| ID | Priority | Requirement |
|---|---|---|
| FR-OUTLINE-010 | P0 | Every heading block shall be foldable, collapsing all content beneath it until the next heading of equal or higher level. |
| FR-OUTLINE-011 | P0 | Fold/unfold shall be triggered by clicking the fold indicator on the heading block or by keyboard shortcut. |
| FR-OUTLINE-012 | P0 | A folded section shall display the heading text, a fold indicator, and a summary showing the number of hidden blocks and word count. |
| FR-OUTLINE-013 | P1 | Fold state shall persist within a session. It shall not persist across sessions (documents open fully expanded on next open). |
| FR-OUTLINE-014 | P1 | Fold all / Unfold all commands shall be available via keyboard shortcut and View menu. |

### 4.3 Section reordering

| ID | Priority | Requirement |
|---|---|---|
| FR-OUTLINE-020 | P0 | Sections (heading blocks and their content) shall be reorderable by drag and drop within Outline mode. |
| FR-OUTLINE-021 | P0 | A drag handle shall appear on hover for each heading block. |
| FR-OUTLINE-022 | P0 | Dragging a heading shall move the heading and all content beneath it until the next heading of equal or higher level. |
| FR-OUTLINE-023 | P1 | Section reorder shall also be available via Up/Down keyboard commands on a selected heading block. |

### 4.4 Word count and targets

| ID | Priority | Requirement |
|---|---|---|
| FR-OUTLINE-030 | P1 | Each section shall display a live word count beside its heading in Outline mode. |
| FR-OUTLINE-031 | P1 | Users shall be able to set a word count target per section. When set, the section shall display progress toward the target. |
| FR-OUTLINE-032 | P2 | Sections that have met or exceeded their target shall display a visual completion indicator. |

### 4.5 Focus mode

| ID | Priority | Requirement |
|---|---|---|
| FR-OUTLINE-040 | P1 | Focus mode shall narrow the editor to display only the current section, dimming or hiding all other sections. |
| FR-OUTLINE-041 | P1 | Focus mode shall be toggled by keyboard shortcut and from the View menu. |
| FR-OUTLINE-042 | P1 | In focus mode, navigation between sections shall be available via keyboard shortcut (next section, previous section). |

### 4.6 Keyboard navigation

| ID | Priority | Requirement |
|---|---|---|
| FR-OUTLINE-050 | P0 | Outline mode shall be fully navigable by keyboard without requiring mouse interaction. |
| FR-OUTLINE-051 | P0 | Standard text editing keyboard shortcuts shall apply within content blocks. |
| FR-OUTLINE-052 | P1 | Arrow key navigation shall move between blocks when the cursor is at the start or end of a block. |

---

## 5. Design Mode (DESIGN)

Design mode is the presentation environment. It renders the document as paginated pages with full physical layout fidelity.

### 5.1 Page canvas

| ID | Priority | Requirement |
|---|---|---|
| FR-DESIGN-001 | P0 | Design mode shall render the document as a sequence of paginated pages on a canvas background. |
| FR-DESIGN-002 | P0 | Each page shall be rendered as a white rectangle with a drop shadow, at the physical dimensions specified in the document frontmatter. |
| FR-DESIGN-003 | P0 | All page dimensions, margins, and content positions shall be computed from physical units (pt) using a single scale factor: `scale = container_width_px / page_width_pt`. |
| FR-DESIGN-004 | P0 | The same scale factor shall be applied uniformly to: the page canvas, the ruler, margin guides, tab stop indicators, and all content positioning. No element shall compute its own independent scale. |
| FR-DESIGN-005 | P0 | The canvas shall be scrollable vertically. Multiple pages shall be visible simultaneously as the user scrolls. |
| FR-DESIGN-006 | P0 | Content shall be fully editable in Design mode. The style selector shall remain the only formatting control available. |
| FR-DESIGN-007 | P1 | The canvas zoom level shall be adjustable by the user (50% – 200%), with a fit-to-width option. |
| FR-DESIGN-008 | P1 | Zoom changes shall update the scale factor and re-render all elements at the new scale without loss of position fidelity. |
| FR-DESIGN-009 | P1 | Tauri's OS API shall be queried for the actual screen DPI at startup and when the window moves between displays. The scale factor shall incorporate DPI for accurate physical rendering. |

### 5.2 Ruler

| ID | Priority | Requirement |
|---|---|---|
| FR-DESIGN-010 | P0 | Design mode shall display a horizontal ruler above the page canvas, spanning the full page width at the current scale. |
| FR-DESIGN-011 | P0 | The ruler shall display tick marks and labels in the document's unit system (mm by default, configurable to inches or pt). |
| FR-DESIGN-012 | P0 | The ruler shall visually indicate the left and right margin positions with distinct shaded zones. |
| FR-DESIGN-013 | P0 | The ruler shall derive its scale from the same single scale factor as the page canvas. The ruler and canvas shall always be pixel-aligned. |
| FR-DESIGN-014 | P1 | The margin indicators on the ruler shall be draggable, updating the document frontmatter margin values in real time as they are dragged. |
| FR-DESIGN-015 | P1 | Tab stop positions shall be displayed on the ruler as draggable indicators. |
| FR-DESIGN-016 | P1 | Tab stops shall be settable by clicking on the ruler and removable by dragging off the ruler. |
| FR-DESIGN-017 | P1 | The ruler shall support multiple tab stop types: left, right, centre, decimal. |

### 5.3 Margin guides

| ID | Priority | Requirement |
|---|---|---|
| FR-DESIGN-020 | P0 | Vertical margin guide lines shall be rendered at the left and right margin positions on each page. |
| FR-DESIGN-021 | P0 | Margin guides shall be rendered at the same scaled pixel position as the ruler's margin indicators. |
| FR-DESIGN-022 | P1 | Margin guides shall be togglable via the View menu. |

### 5.4 Headers and footers

| ID | Priority | Requirement |
|---|---|---|
| FR-DESIGN-030 | P0 | Design mode shall render document headers and footers on each page as defined in the frontmatter `header` and `footer` properties. |
| FR-DESIGN-031 | P0 | Headers and footers shall support three zones: left, centre, right. Each zone accepts a template string with token interpolation (`{title}`, `{page}`, `{pages}`, `{author}`, `{date}`, `{section}`). |
| FR-DESIGN-032 | P0 | The right rail shall provide a template editor for header and footer zones without requiring the user to edit raw frontmatter YAML. |
| FR-DESIGN-033 | P1 | The first page shall support a distinct header/footer or suppression of header/footer (configurable in frontmatter). |
| FR-DESIGN-034 | P1 | Odd and even page headers/footers shall be supported for facing-pages documents (configurable via theme boolean variable). |

### 5.5 Page settings

| ID | Priority | Requirement |
|---|---|---|
| FR-DESIGN-040 | P0 | The right rail shall provide page size selection from standard presets: A4, US Letter, A5, A3. |
| FR-DESIGN-041 | P0 | Custom page dimensions shall be settable via numeric inputs in the right rail (width and height in mm). |
| FR-DESIGN-042 | P0 | Page orientation shall be selectable: portrait or landscape. |
| FR-DESIGN-043 | P0 | All four margins (top, bottom, left, right) shall be individually configurable in the right rail via numeric inputs. |
| FR-DESIGN-044 | P0 | Changes to page settings shall update the document frontmatter immediately and re-render the canvas. |
| FR-DESIGN-045 | P1 | Page size and margin changes shall update the ruler and margin guides in real time without requiring a full re-render. |

---

## 6. Semantic Styling (STYLE)

### 6.1 Style application

| ID | Priority | Requirement |
|---|---|---|
| FR-STYLE-001 | P0 | The application shall not expose raw font, size, colour, or weight controls as primary formatting actions. The style selector is the primary and only formatting interface. |
| FR-STYLE-002 | P0 | Every content block shall have exactly one block-level style applied at all times. A default style (`BodyText`) shall be applied when no explicit style is set. |
| FR-STYLE-003 | P0 | Block styles shall be applied by selecting a style from the style selector in the right rail or toolbar. |
| FR-STYLE-004 | P0 | Inline styles (Emphasis, Strong, Code, and any custom inline styles defined in the theme) shall be applicable to selected text within a block. |
| FR-STYLE-005 | P0 | The style selector shall display the name of the currently applied style for the block at cursor position. |
| FR-STYLE-006 | P0 | The style selector shall list all named styles defined in the active theme, grouped by scope (block styles, inline styles). |
| FR-STYLE-007 | P0 | Applying a style shall not alter the document's content — only the style reference stored in the format. |
| FR-STYLE-008 | P1 | Keyboard shortcuts shall be assignable to named styles in settings. Style application shall be possible without touching the mouse. |

### 6.2 Paste handling

| ID | Priority | Requirement |
|---|---|---|
| FR-STYLE-010 | P0 | Pasting content from an external source (Word, web browser, another application) shall strip all raw formatting (font, size, colour, weight, inline styles). |
| FR-STYLE-011 | P0 | After stripping, the application shall prompt the user to assign a style to the pasted content if the pasted content contains multiple block types. |
| FR-STYLE-012 | P0 | Pasting plain text shall insert it with the style of the block at the cursor position. |
| FR-STYLE-013 | P1 | A "Paste and Match Style" command shall be available and shall be the default paste behaviour. A "Paste with Structure" option shall attempt to map the external document's heading hierarchy to the equivalent folivm styles. |

### 6.3 Character (inline) styles

| ID | Priority | Requirement |
|---|---|---|
| FR-STYLE-030 | P0 | The application shall support named inline (character) styles applied to selected text spans within a block. |
| FR-STYLE-031 | P0 | Inline styles shall be stored in the `.fvm` format using Pandoc-compatible span syntax: `[text]{.StyleName}`. |
| FR-STYLE-032 | P0 | The inline style picker shall display only named inline styles from the active theme. No raw font, colour, size, or weight controls shall be exposed for character-level formatting. |
| FR-STYLE-033 | P0 | The built-in default theme shall include the following inline styles: `Emphasis` (italic), `Strong` (bold), `Code` (monospace), `Superscript`, `Subscript`. |
| FR-STYLE-034 | P0 | The inline style picker shall be accessible via keyboard shortcut, toolbar, and right-click context menu on selected text. |
| FR-STYLE-035 | P0 | Applying an inline style to a selection that already has an inline style applied shall toggle it off if it is the same style, or replace it if it is a different style. |
| FR-STYLE-036 | P1 | The active theme may define additional inline styles (e.g. `DefinedTerm`, `Warning`, `CrossReference`, `ProductName`). All theme-defined inline styles shall appear in the inline style picker automatically. |
| FR-STYLE-037 | P1 | Built-in keyboard shortcuts for inline styles: Emphasis (Cmd/Ctrl+I), Strong (Cmd/Ctrl+B), Code (Cmd/Ctrl+`). Theme-defined inline styles shall be assignable to custom shortcuts in settings. |
| FR-STYLE-038 | P1 | Nested inline styles shall be supported — e.g. Strong applied within a Warning span. The format shall represent this as nested spans: `[[WARNING]{.Strong}]{.Warning}`. |
| FR-STYLE-039 | P1 | Inline styles may declare a `contexts` attribute in the theme limiting which block styles they are valid within. The inline style picker shall filter accordingly — a `LegalCitation` inline style need not appear when editing a `DiagramCaption` block. |

### 6.4 Style definitions

| ID | Priority | Requirement |
|---|---|---|
| FR-STYLE-020 | P0 | Styles are defined exclusively in the active `.fvm-theme` file. The document file does not contain style definitions. |
| FR-STYLE-021 | P0 | Removing a style from the theme while documents reference it shall not corrupt those documents. Blocks with a removed style shall fall back to the default style and display a warning indicator. Spans with a removed inline style shall have the style reference stripped and display a warning. |
| FR-STYLE-022 | P1 | The theme editor (in the right rail or settings) shall allow adding, editing, and removing named styles without requiring manual YAML editing. |

---

## 7. Theme System (THEME)

### 7.1 Collections and variables

| ID | Priority | Requirement |
|---|---|---|
| FR-THEME-001 | P0 | The application shall support theme files in `.fvm-theme` format (YAML). |
| FR-THEME-002 | P0 | A theme file shall support multiple collections. Each collection shall have a name, a set of modes, and a set of variables. |
| FR-THEME-003 | P0 | Variables shall be one of four types: Color, Number, String, Boolean. |
| FR-THEME-004 | P0 | Variables shall support aliases — a variable may reference another variable using the `{collection.variable-name}` syntax. Changes to a source variable shall cascade automatically to all aliases. |
| FR-THEME-005 | P0 | Collections shall support multiple modes. Each mode stores an alternate value for every variable in the collection. |
| FR-THEME-006 | P0 | The active mode per collection shall be selectable at the project level. Switching mode updates all alias resolutions throughout the theme. |
| FR-THEME-007 | P0 | The theme shall support at minimum three collections: Primitives (raw values, single mode), Semantic (purpose-mapped aliases, screen and print modes), and one user-defined Brand collection for multi-client projects. |
| FR-THEME-008 | P1 | Variables shall support scoping to restrict them to specific property contexts (e.g. a color variable scoped to `text` shall not appear in background colour selectors). |

### 7.2 Named styles

| ID | Priority | Requirement |
|---|---|---|
| FR-THEME-020 | P0 | Named styles shall be defined in the theme file as composite bundles of variable references. |
| FR-THEME-021 | P0 | Block-scoped styles shall define: font-family, font-size, font-weight, font-style, color, line-height, space-before, space-after, indent-left, indent-right, text-align. |
| FR-THEME-022 | P0 | Inline-scoped styles shall define: font-family, font-size, font-weight, font-style, color, background, text-decoration. |
| FR-THEME-023 | P0 | Every style property value shall reference a theme variable using `{collection.variable-name}` syntax. Hard-coded values shall be permitted but flagged as a warning in the theme editor. |
| FR-THEME-024 | P1 | Styles shall support inheritance via an `extends` property. A child style inherits all properties of its parent and may override individual properties. |
| FR-THEME-025 | P1 | The theme editor shall provide a live preview of each style using sample text. |

### 7.3 Paragraph-level decorators (callout styles)

| ID | Priority | Requirement |
|---|---|---|
| FR-THEME-050 | P0 | Block styles shall support an optional `icon` property. The icon is a purely decorative presentation element — the semantic meaning is conveyed by the style name, not the icon. |
| FR-THEME-051 | P0 | The `icon` property shall support the following attributes: `source` (font-awesome / lucide / svg / emoji), `name` (icon identifier within the source), `weight` (solid / regular / light for FA), `color` (token reference), `size` (pt), `position` (leading / trailing / background). |
| FR-THEME-052 | P0 | The application shall bundle an offline subset of Font Awesome Free icons covering common callout vocabulary. No network call shall be required to render bundled icons. The bundled set shall include at minimum: fa-lightbulb, fa-triangle-exclamation, fa-circle-info, fa-circle-check, fa-microchip, fa-scale-balanced, fa-paperclip, fa-bookmark, fa-star, fa-flag. |
| FR-THEME-053 | P0 | The default theme shall define a standard callout vocabulary: `Tip`, `Warning`, `Remember`, `Important`, `Technical`. Each shall include an icon, a background colour, and a left border using semantic token references. |
| FR-THEME-054 | P1 | Custom SVG icons shall be definable inline in the theme file under a `custom-icons` key and referenceable by name in any style's `icon` property. |
| FR-THEME-055 | P1 | Block styles with an `icon` property and `position: leading` shall render with the icon vertically centred beside the block's first line, with subsequent lines indented to align with the text — not beside the icon. This layout shall be handled entirely by the renderer from the style definition, with no table or image insertion required. |
| FR-FMT-036 | P0 | The format shall support multi-paragraph callout blocks using the fence syntax `:::StyleName` where `StyleName` is any block-scoped style defined in the active theme. The fence block shall render as a single callout container with the specified style applied. |
| FR-EXPORT-030 | P0 | PDF export shall render block style icons as embedded SVG path data derived from the icon source. No raster image shall be generated for icon rendering in PDF. |
| FR-EXPORT-031 | P1 | DOCX export shall render block style icons as embedded SVG images using DOCX drawing anchors positioned to the left of the paragraph text. A Unicode character fallback shall be used when the target application does not support SVG embedding. |

### 7.4 Accessibility — structural requirements from the format

These requirements flow directly from the semantic-first format. They are not bolt-on accessibility features — they describe accessibility properties that emerge from correct use of the format.

| ID | Priority | Requirement |
|---|---|---|
| FR-THEME-060 | P0 | The theme editor shall display the WCAG AA contrast ratio for every colour token pair used together (text colour on background colour). Pairs that fail WCAG AA (4.5:1 for normal text, 3:1 for large text) shall be flagged with a visible warning. |
| FR-THEME-061 | P1 | The theme editor shall flag any style where `color` is the sole differentiator between two styles that would otherwise appear identical — meaning is not conveyed by colour alone. |
| FR-FMT-037 | P0 | The frontmatter shall support a `lang` property declaring the document language (BCP-47 code, e.g. `en-AU`, `fr-FR`). This value shall propagate to the PDF `Lang` entry and the DOCX `w:lang` attribute. |
| FR-FMT-038 | P1 | The application shall validate heading order on save and warn (not block) when a heading level is skipped (e.g. H1 followed directly by H3 with no H2). Heading order is critical for screen reader navigation. |
| FR-FMT-039 | P1 | `cell:diagram` shall require an `alt` attribute describing the diagram content for screen readers and AI. The editor shall prompt for `alt` text when a `cell:diagram` is inserted without one. |
| FR-EXPORT-032 | P0 | PDF export shall produce a tagged PDF conforming to PDF/UA-1 (ISO 14289-1) when the `output/accessible-pdf` theme boolean is true. Tagged PDF maps document structure (headings, paragraphs, lists, tables) to PDF tags for screen reader navigation. |
| FR-EXPORT-033 | P1 | PDF/UA export shall map folivm style names to PDF role tags: H1–H6 styles to `<H1>`–`<H6>`, body styles to `<P>`, list items to `<LI>`, callout styles to `<Note>`, `cell:image` alt text to `<Alt>`. |
| FR-EXPORT-034 | P1 | DOCX export shall preserve semantic heading structure using DOCX built-in heading styles (Heading 1 through Heading 6), ensuring correct navigation in Word's document map and assistive technology. |

### 7.6 Theme application

| ID | Priority | Requirement |
|---|---|---|
| FR-THEME-030 | P0 | A theme is applied at the project level. All documents in the project share the project theme. |
| FR-THEME-031 | P0 | A document may reference a different theme in its frontmatter to override the project-level theme for that document. |
| FR-THEME-032 | P0 | Switching the project theme shall re-render all open documents immediately using the new theme's style definitions. |
| FR-THEME-033 | P1 | The application shall ship with a set of built-in themes covering common professional contexts: Default, Legal, Academic, Technical. |
| FR-THEME-034 | P1 | The application shall support importing a `.fvm-theme` file into a project. |
| FR-THEME-035 | P2 | The application shall support exporting the current project theme as a shareable `.fvm-theme` file. |

### 7.7 Theme editor UI

| ID | Priority | Requirement |
|---|---|---|
| FR-THEME-040 | P1 | The Variables panel in the right rail shall display all collections, their modes, variable groups, and current values. |
| FR-THEME-041 | P1 | The Variables panel shall allow switching the active mode per collection. |
| FR-THEME-042 | P1 | Color variables shall be editable via a colour picker inline in the Variables panel. |
| FR-THEME-043 | P1 | Number and String variables shall be editable via inline inputs. |
| FR-THEME-044 | P1 | Boolean variables shall be editable via inline toggle switches. |
| FR-THEME-045 | P1 | Alias variables shall display their resolved value alongside the alias path. |
| FR-THEME-046 | P2 | The theme editor shall provide a full-screen theme editing view beyond the right rail panel. |

---

## 8. Project-wide Find and Replace (SEARCH)

### 8.1 Search

| ID | Priority | Requirement |
|---|---|---|
| FR-SEARCH-001 | P0 | The Search sidebar panel shall provide project-wide text search across all `.fvm` files in the project directory. |
| FR-SEARCH-002 | P0 | Search shall operate on the plain text content of `.fvm` files without requiring documents to be open. ASCII format makes this a direct file system text search. |
| FR-SEARCH-003 | P0 | Search results shall be grouped by file, with each result showing the filename, line number, and a line of context with the match highlighted. |
| FR-SEARCH-004 | P0 | The search panel shall support: case-sensitive matching, whole-word matching, regular expression matching. |
| FR-SEARCH-005 | P0 | Clicking a search result shall open the file in a new tab (or focus its existing tab) and scroll to the matching line. |
| FR-SEARCH-006 | P0 | The search panel shall display a total match count and a per-file match count badge. |
| FR-SEARCH-007 | P1 | Search shall support filtering by file glob pattern (e.g. search only in `DELIVERABLES/*.fvm`). |
| FR-SEARCH-008 | P1 | Search shall be debounced — results shall update as the user types with a short delay. |
| FR-SEARCH-009 | P1 | Search shall exclude the `.folivm/` configuration directory, `.git/` directory, and content library directory from results unless explicitly included. |

### 8.2 Replace

| ID | Priority | Requirement |
|---|---|---|
| FR-SEARCH-020 | P0 | The search panel shall provide a replace field supporting replace-one and replace-all operations. |
| FR-SEARCH-021 | P0 | Replace-all shall apply the replacement across all matching files in the project in a single operation. |
| FR-SEARCH-022 | P0 | Each replace operation shall be recorded as a single version history entry (git commit) with a generated message identifying the search and replace terms. |
| FR-SEARCH-023 | P0 | Replace shall operate only on the text content of blocks. It shall not modify cell attributes, YAML frontmatter values, or style references unless the match occurs within those regions and the user has explicitly enabled frontmatter search. |
| FR-SEARCH-024 | P1 | A preview mode shall show all proposed replacements across files before committing the operation. |
| FR-SEARCH-025 | P1 | Replace shall support per-file accept/reject — the user may deselect individual files or individual matches from a replace-all operation. |

### 8.3 Find in document

| ID | Priority | Requirement |
|---|---|---|
| FR-SEARCH-030 | P0 | Find in document shall be available in both Outline and Design modes via keyboard shortcut (Cmd/Ctrl+F). |
| FR-SEARCH-031 | P0 | Find in document shall highlight all matches in the current document and support next/previous navigation. |
| FR-SEARCH-032 | P1 | Find in document shall support the same matching options as project-wide search (case, whole-word, regex). |

---

## 9. Content Library (LIB)

### 9.1 Library model

| ID | Priority | Requirement |
|---|---|---|
| FR-LIB-001 | P0 | The content library shall be a managed collection of reusable `.fvm` fragment files stored within the project (`.folivm/library/`) or at a global application level. |
| FR-LIB-002 | P0 | Library items shall be standard `.fvm` fragment files — plain text, diffable, versionable. |
| FR-LIB-003 | P0 | Each library item shall have a unique identifier, a human-readable name, a description, and a semantic version number. |
| FR-LIB-004 | P0 | Library items shall be referenceable in documents via `cell:include` with a version pin: `ref: library.item-id`, `version: "2.3"`. |
| FR-LIB-005 | P0 | A `cell:include` block shall render the library item content inline in both Outline and Design modes. |
| FR-LIB-006 | P0 | The version pin shall be honoured: a document using `version: "2.3"` of a library item shall continue to render version 2.3 even after the library item is updated to 3.0. |

### 9.2 Library management

| ID | Priority | Requirement |
|---|---|---|
| FR-LIB-010 | P0 | The application shall provide a library management interface accessible from the sidebar or settings. |
| FR-LIB-011 | P0 | Users shall be able to add new library items by: creating from scratch, or promoting an existing document section to a library item. |
| FR-LIB-012 | P0 | Editing a library item shall increment its version number. The previous version shall be retained and accessible. |
| FR-LIB-013 | P0 | Deleting a library item shall warn if any documents in the project reference it. Deletion shall not break existing documents — the `cell:include` block shall fall back to a placeholder showing the item identifier and version. |
| FR-LIB-014 | P1 | The library management interface shall show which documents use each library item and at which version. |

### 9.3 Library update workflow

| ID | Priority | Requirement |
|---|---|---|
| FR-LIB-020 | P1 | When a newer version of a library item exists, documents referencing an older version shall display an update available indicator on the `cell:include` block. |
| FR-LIB-021 | P1 | Accepting a library update shall update the version pin in the document's `cell:include` block and record the change in version history. |
| FR-LIB-022 | P1 | Library updates shall be opt-in per document, per include block. A project-wide "update all" action shall be available. |
| FR-LIB-023 | P2 | The library update UI shall show a diff between the old and new version of the item before the user accepts. |

---

## 10. Versioning (VER)

### 10.1 Version model

| ID | Priority | Requirement |
|---|---|---|
| FR-VER-001 | P0 | All version history shall be stored in the project's git repository. The application manages git operations transparently — users never interact with git directly. |
| FR-VER-002 | P0 | The application shall use document-language terminology throughout the versioning UI. Git terminology shall not be exposed. |
| FR-VER-003 | P0 | Auto-save shall occur at a configurable interval (default: every 30 seconds) and on window blur. Auto-save shall write to disk but shall not create a version history entry. |

### 10.2 Saving versions

| ID | Priority | Requirement |
|---|---|---|
| FR-VER-010 | P0 | "Save Version" (Cmd/Ctrl+Shift+S) shall create a version history entry (git commit) for all modified files in the project with a user-provided or auto-generated message. |
| FR-VER-011 | P0 | The version message input shall be accessible from the Versioning sidebar panel. |
| FR-VER-012 | P0 | The Versioning panel shall display the staged (modified) and unstaged file lists using document-language labels. |
| FR-VER-013 | P1 | Version messages shall be auto-generated based on what changed: "Updated Section 2 in project1.fvm", "Added new document research.fvm". |

### 10.3 Drafts (branches)

| ID | Priority | Requirement |
|---|---|---|
| FR-VER-020 | P0 | "Create Draft" shall create a new git branch from the current state, giving it a user-provided name. |
| FR-VER-021 | P0 | The active draft name shall be displayed in the status bar and Versioning panel. |
| FR-VER-022 | P0 | "Switch Draft" shall switch the working directory to a different branch after prompting to save pending changes. |
| FR-VER-023 | P1 | "Combine Draft" shall merge a named draft back into the main version (git merge), presenting any conflicts as a track-changes view for resolution. |

### 10.4 Track changes and history

| ID | Priority | Requirement |
|---|---|---|
| FR-VER-030 | P0 | The Versioning panel shall display a chronological version history list showing: version message, author attribution, timestamp, and files changed. |
| FR-VER-031 | P0 | Selecting a version history entry shall show the changes from that version as a track-changes view — additions and deletions presented inline in the document, not as a raw diff. |
| FR-VER-032 | P0 | Individual changes in the track-changes view shall be acceptable or rejectable one at a time or all at once. |
| FR-VER-033 | P1 | Track changes shall attribute each change to its author. AI-extension-generated content shall be attributed to the extension, not to the user. |
| FR-VER-034 | P1 | "Who wrote this" shall be available on right-click for any block, showing the author and version that last modified it. |

### 10.5 Comments

| ID | Priority | Requirement |
|---|---|---|
| FR-VER-040 | P1 | Users shall be able to add inline comments to any block in the document. |
| FR-VER-041 | P1 | Comments shall be displayed as margin annotations in Design mode and as inline indicators in Outline mode. |
| FR-VER-042 | P1 | Comments shall be resolvable. Resolved comments shall be hidden by default but accessible via the Versioning panel. |
| FR-VER-043 | P1 | Comments shall be stored in the `.fvm` file in a way that does not affect export output — they shall be stripped from PDF and DOCX exports. |

---

## 11. Export (EXPORT)

### 11.1 PDF export

| ID | Priority | Requirement |
|---|---|---|
| FR-EXPORT-001 | P0 | The application shall export the active document to PDF using the Design mode rendering as the source of truth. |
| FR-EXPORT-002 | P0 | PDF export shall honour all page settings from the frontmatter: page size, orientation, margins. |
| FR-EXPORT-003 | P0 | PDF export shall resolve all template tokens using the active data sources. Unresolved tokens shall export as visible raw token text. |
| FR-EXPORT-004 | P0 | PDF export shall render headers and footers on every page as defined in the frontmatter templates. |
| FR-EXPORT-005 | P0 | `cell:ai` and `cell:include` blocks shall export as their rendered content. `cell:data` blocks shall export as their resolved data values. |
| FR-EXPORT-006 | P0 | `cell:ai` blocks that have not been accepted (status: draft) shall export with a visible watermark or be excluded, controlled by an export setting. |
| FR-EXPORT-007 | P0 | Comments and track-changes markup shall not appear in PDF export. |
| FR-EXPORT-008 | P1 | PDF export shall embed fonts used in the theme when the `output/embed-fonts` theme boolean is true. |
| FR-EXPORT-009 | P1 | PDF export shall support a print mode that uses the `print` semantic mode values from the theme (e.g. adjusted colours and sizes for print). |
| FR-EXPORT-010 | P1 | The export dialog shall provide options: page range selection, include/exclude frontmatter block, draft cell handling, print mode toggle. |
| FR-EXPORT-011 | P2 | PDF export shall support crop marks when the `output/print-marks` theme boolean is true. |

### 11.2 DOCX export

| ID | Priority | Requirement |
|---|---|---|
| FR-EXPORT-020 | P0 | The application shall export the active document to DOCX format. |
| FR-EXPORT-021 | P0 | DOCX export shall map folivm named styles to DOCX paragraph and character styles of the same name. |
| FR-EXPORT-022 | P0 | DOCX export shall preserve document structure: headings, sections, lists, blockquotes, tables. |
| FR-EXPORT-023 | P0 | DOCX export shall resolve template tokens using the same rules as PDF export. |
| FR-EXPORT-024 | P0 | DOCX export shall preserve headers and footers using DOCX header/footer sections. |
| FR-EXPORT-025 | P1 | DOCX export shall map theme typography (font family, size, weight, colour) to DOCX style definitions so that the exported file is re-stylable in Word. |
| FR-EXPORT-026 | P1 | `cell:math` blocks shall export as DOCX equation objects (OMML) where possible. |
| FR-EXPORT-027 | P1 | `cell:diagram` blocks shall export as embedded SVG images. |
| FR-EXPORT-028 | P2 | The export dialog shall provide a DOCX compatibility mode that flattens all styles to inline formatting for maximum compatibility with older Word versions. |

---

## 12. Extension Architecture (EXT)

### 12.1 Extension model

| ID | Priority | Requirement |
|---|---|---|
| FR-EXT-001 | P0 | Extensions shall be the exclusive mechanism for adding AI, cloud, CRM, citation, signature, and vertical-specific capabilities to folivm. The core application shall not contain any such integrations. |
| FR-EXT-002 | P0 | Each extension shall declare: name, version, description, author, required permissions, and network connectivity requirement (none / optional / required). |
| FR-EXT-003 | P0 | Extensions that declare network connectivity shall display a connectivity badge in the Extensions panel. Extensions with no connectivity declaration shall make no network calls — this shall be enforced by the runtime. |
| FR-EXT-004 | P0 | The application shall function completely and correctly with zero extensions installed. |
| FR-EXT-005 | P0 | An unrecognised cell type (from an uninstalled extension) shall render as a labelled placeholder. It shall not cause an error, prevent document opening, or affect export of other content. |

### 12.2 Extension API capabilities

| ID | Priority | Requirement |
|---|---|---|
| FR-EXT-010 | P0 | Extensions shall be able to register as handlers for one or more cell types. When a cell type has a registered handler, the handler renders the cell's content. |
| FR-EXT-011 | P0 | Extensions shall be able to register sidebar panels, which appear in the activity bar as additional icons. |
| FR-EXT-012 | P0 | Extensions shall be able to register right rail panels, which appear as additional sections in the right rail. |
| FR-EXT-013 | P0 | Extensions shall be able to register toolbar commands and menu items. |
| FR-EXT-014 | P0 | Extensions shall be able to register export hooks. Export hook extensions receive the full document object model — a structured tree of blocks, cells, and metadata — not a rendered output stream. |
| FR-EXT-015 | P1 | Extensions shall be able to register data source handlers for `cell:data` blocks, resolving `source: [extension-id]` references. |
| FR-EXT-016 | P1 | Extensions shall be able to register document lifecycle hooks: on-open, on-save-version, on-export-start, on-export-complete. |
| FR-EXT-017 | P1 | Extensions shall be able to contribute content library items to the project library. |
| FR-EXT-018 | P1 | Extensions shall be able to contribute named styles and complete `.fvm-theme` files. |
| FR-EXT-019 | P2 | Extensions shall be able to register additional cell types beyond the built-in set, with their own renderer and export handler. |

### 12.3 Extension management UI

| ID | Priority | Requirement |
|---|---|---|
| FR-EXT-030 | P0 | The Extensions sidebar panel shall display installed extensions with enable/disable toggles and version information. |
| FR-EXT-031 | P0 | The Extensions panel shall display available extensions from the extension marketplace (when network is available). |
| FR-EXT-032 | P0 | Extensions shall be installable from the marketplace and from a local file (`.fvmext` package). |
| FR-EXT-033 | P0 | Disabling an extension shall hide its UI contributions and revert its cell type handlers to placeholder rendering, without removing the extension or modifying documents. |
| FR-EXT-034 | P1 | The Extensions panel shall include a search field for marketplace browsing. |
| FR-EXT-035 | P1 | Extension updates shall be notified in the Extensions panel. Updates shall be opt-in. |

---

## 13. Settings (SETTINGS)

| ID | Priority | Requirement |
|---|---|---|
| FR-SETTINGS-001 | P0 | The application shall provide a settings interface accessible from the menu (File → Settings or Cmd/Ctrl+,). |
| FR-SETTINGS-002 | P0 | Settings shall include: auto-save interval, default page size, default orientation, default theme, editor font size (for Outline mode UI), spell check language. |
| FR-SETTINGS-003 | P0 | Settings shall include keyboard shortcut customisation for all application commands and for named styles. |
| FR-SETTINGS-004 | P0 | Settings shall persist across sessions using the OS-standard application settings location. |
| FR-SETTINGS-005 | P1 | Settings shall include a theme editor providing a GUI over the active `.fvm-theme` file. |
| FR-SETTINGS-006 | P1 | Settings shall include extension management (install, remove, update) as an alternative to the sidebar Extensions panel. |
| FR-SETTINGS-007 | P1 | Settings shall include a reset-to-defaults option per settings section. |
| FR-SETTINGS-008 | P2 | Settings shall be exportable and importable as a `.yaml` file for sharing across machines. |

---

## 14. Document Outline Panel (TOC)

| ID | Priority | Requirement |
|---|---|---|
| FR-TOC-001 | P0 | The Document Outline sidebar panel shall display a navigable tree of all headings in the active document. |
| FR-TOC-002 | P0 | The outline tree shall reflect the document's live heading hierarchy (H1, H2, H3) updated as the document is edited. |
| FR-TOC-003 | P0 | Clicking an outline entry shall scroll the editor to that heading in both Outline and Design modes. |
| FR-TOC-004 | P0 | The currently visible section shall be highlighted in the outline panel as the user scrolls. |
| FR-TOC-005 | P1 | The Document Outline panel shall display document statistics: section count, page count (Design mode), total word count, character count. |
| FR-TOC-006 | P1 | Per-section word counts shall be displayed beside each outline entry. |

---

## 15. Out of scope for v1.0

The following are explicitly deferred. They are captured here to prevent scope creep during v1.0 development.

| Feature | Target |
|---|---|
| Real-time multi-user collaboration | v2.0 |
| Cloud sync / hosted document storage | v2.0 |
| Mobile application | v2.0 |
| Web application | v2.0 |
| Formula/spreadsheet tables | v1.1 |
| Full extension marketplace (curated) | v1.1 |
| DITA / XML export (via extension) | v1.1 |
| HTML / ePub export (via extension) | v1.1 |
| Facing pages / book layout | v1.1 |
| Handwriting equation input (MyScript) | v1.1 |
| Plagiarism checking integration | v1.1 |
| Digital signature integration | v1.1 |

---

*Document history tracked in git. Next: Non-Functional Requirements (NFR.md)*
