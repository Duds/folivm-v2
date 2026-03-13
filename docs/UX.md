# folivm — UX / Interaction Design
**Version:** 0.1 (draft)
**Date:** 2026-03-13
**Status:** In review
**Depends on:** CONCEPT.md v0.4, FR.md v0.1, SCOPE.md v1.0

---

## 1. Interaction Principles

**Keyboard first, mouse optional.** Every action in the editor is reachable by keyboard. Mouse interactions are additive, not required. A user who never touches the mouse can produce a complete professional document.

**Semantic intent, not visual instruction.** The user says "this is a Heading 2" not "make this 18pt bold." Every interface affordance reinforces this: style names appear everywhere, font size and colour never appear in the editing surface.

**The document is always consistent.** There is no unsaved-but-applied state for structural operations. Block style changes, cell insertions, and section moves take effect immediately and are undoable. Auto-save runs continuously. The user never loses work by closing a window.

**Modes are not restrictions.** Outline mode and Design mode present the same document differently. Switching between them is instant and reversible. The user is never blocked from editing in either mode.

**Interruptions are minimal.** Dialogs are used sparingly. Confirmations are asked only when an action is difficult to reverse (delete a library item, force-discard changes). Everything else is undoable.

---

## 2. Application Shell

### 2.1 Window layout

```
┌─────────────────────────────────────────────────────────────────────┐
│ TitleBar: [window controls] [menu bar] [panel toggles]              │
├───┬─────────────────┬───────────────────────────────┬───────────────┤
│   │                 │ TabBar                        │               │
│ A │   Sidebar       │─────────────────────────────  │  Right Rail   │
│ c │   Panel         │ Toolbar                       │               │
│ t │                 │─────────────────────────────  │  Styles Panel │
│ i │  Explorer       │ Ruler (Design mode only)      │  Page Settings│
│ v │  Search         │─────────────────────────────  │  Header/Footer│
│ i │  Versioning     │                               │  Variables    │
│ t │  Extensions     │   Editor Canvas               │               │
│ y │  TOC            │                               │               │
│   │                 │                               │               │
│   │                 │                               │               │
├───┴─────────────────┴───────────────────────────────┴───────────────┤
│ StatusBar                                                           │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 Panel controls

**Activity bar** — the narrow icon strip on the far left. Clicking an activity icon toggles its sidebar panel. Clicking the active icon collapses the sidebar. The active icon is highlighted.

**Sidebar panel** — displays one panel at a time, selected by the activity bar. Resizable by dragging the right edge. Minimum width: 200px. Collapses to zero by clicking the active activity icon or pressing `Cmd+B` (toggle sidebar).

**Right rail** — contains the style/page/variable panels. Toggle with `Cmd+Shift+B` or the panel toggle button in the title bar. Four tabs: Styles, Page, Header/Footer, Variables. Defaults to Styles tab.

**Toolbar** — always visible above the editor canvas. Contents adapt to context. Never hidden.

**Ruler** — visible in Design mode only. Hidden in Outline mode. No toggle; it is part of the Design mode interface.

### 2.3 Tab bar

Each open document has a tab. Tabs show: filename, modified indicator (dot), close button.

| Interaction | Result |
|---|---|
| Click tab | Switch to document |
| Middle-click tab | Close document |
| Click × on tab | Close document (prompts if unsaved version) |
| `Cmd+W` | Close active tab |
| `Cmd+Shift+]` / `Cmd+Shift+[` | Next / previous tab |
| `Cmd+1` … `Cmd+9` | Switch to tab by position |
| Drag tab | Reorder tabs |
| Right-click tab | Context menu: Close, Close Others, Close All, Copy Path, Reveal in Explorer |

**Unsaved state:** A tab with a modified indicator has auto-saved content that differs from the last git-committed version. Closing such a tab prompts: "Save a version before closing? [Save Version] [Close Without Saving] [Cancel]"

### 2.4 Status bar

Left to right:

```
[Mode: Outline ▾] [1,842 words] [Page 3 of 12] [Ln 24, Ch 156] [Branch: main ●3] [⟳ Saved 2m ago] [100% −  +]
```

| Element | Behaviour |
|---|---|
| Mode indicator | Click to switch mode. Shows Outline or Design |
| Word count | Total document words. Shows selection count when text is selected |
| Page indicator | Visible in Design mode only |
| Cursor position | Block number and grapheme offset within the block |
| Branch indicator | Git branch name. Dot + count indicates unsaved changes since last version |
| Save indicator | "Saved N ago" / "Saving…" / "Unsaved changes" |
| Zoom | Click − / + to change by 10%. Click percentage to type exact value. Scroll on the percentage to change continuously |

---

## 3. Project Workspace

### 3.1 Opening a project

On launch with no recent project: a welcome screen with [Open Project Folder] and [Recent Projects] list.

`File → Open Project` → native folder picker. The selected directory becomes the project root. All `.fvm` and `.fvm-theme` files in the directory tree appear in the Explorer panel.

The project retains its last state between sessions: which files were open, which tab was active, panel sizes, mode.

### 3.2 Explorer panel

A tree view of the project directory. `.fvm-theme` files are shown with a theme icon. `.fvm` files show a modified indicator if they have unsaved changes.

| Interaction | Result |
|---|---|
| Click file | Open in a new tab (or focus existing tab) |
| Double-click file | Open and make permanent (single-click opens in preview tab) |
| Right-click file | Rename, Delete, Duplicate, Copy Path, Reveal in OS, Save to Library |
| Right-click folder | New Document, New Folder, Rename, Delete |
| `N` with folder focused | New document in that folder |
| Drag file | Reorder or move into folder |

### 3.3 New document

`File → New Document` or `Cmd+N`.

If a project is open: a small inline dialog in the Explorer panel prompts for a filename. The document opens immediately as an empty Outline mode document with one empty Body block.

If no project is open: a native file picker prompts for a save location first.

**Empty document state:**
- Single empty Body block with cursor positioned at offset 0
- Block shows ghost text in the style of the current theme's placeholder colour: the style name ("Body")
- Ghost text disappears on first keypress
- Outline mode by default
- Toolbar shows "Body" as the active block style

---

## 4. Text Editing — Core Behaviours

### 4.1 Cursor model

The cursor is a model-level coordinate: `{ block_id, offset }` where offset is a grapheme cluster index. One grapheme cluster = one cursor position, regardless of how many Unicode codepoints it contains.

The cursor blink is rendered by the Rust layout engine. The blink rate follows the OS accessibility setting.

### 4.2 Cursor movement

| Key | Movement |
|---|---|
| `←` / `→` | One grapheme cluster |
| `Cmd+←` / `Cmd+→` | Start / end of line |
| `Alt+←` / `Alt+→` | Previous / next word boundary |
| `↑` / `↓` | One visual line (within or across blocks) |
| `Cmd+↑` / `Cmd+↓` | Start / end of document |
| `Home` | Start of current visual line |
| `End` | End of current visual line |
| `PgUp` / `PgDn` | Scroll one viewport height, cursor follows |

Moving `←` at offset 0 moves cursor to the end of the preceding block. Moving `→` at the end of a block moves to offset 0 of the next block.

### 4.3 Selection

All cursor movement keys extend the selection when `Shift` is held. Click sets the cursor (clears selection). Click and drag creates a selection. Double-click selects a word. Triple-click selects the block.

`Cmd+A` selects the entire document (all blocks).

When a selection is active:
- The selection is highlighted (colour defined by theme's selection token)
- The word count in the status bar switches to the selection word count
- The inline style toolbar appears (see §6.2)
- Typing replaces the selection

### 4.4 Typing

Printable characters insert at the cursor position. The active block's RunBuffer is updated immediately. The Canvas redraws the affected region. No IPC. No perceptible delay.

If the cursor is at the boundary between two inline styles, the new character inherits the style of the character to its left (or no style if at the start of a block).

### 4.5 Enter — block splitting

`Enter` splits the active block at the cursor position.

**Paragraph blocks:** The text before the cursor becomes the current block. The text from the cursor becomes a new block of the same style — unless the current style is in the "heading propagation exclusion" list (Heading1–6), in which case the new block is Body.

```
Before: [Heading1: "Introduction"]  cursor after "Intro"
After:  [Heading1: "Intro"]
        [Body: "duction"]
```

**List items:** `Enter` at the end of a list item creates a new list item at the same level. `Enter` on an empty list item (no text) exits the list and creates a Body block. `Enter` in the middle of a list item splits the item into two items.

**Section boundary:** `Enter` at the end of the last block inside a section creates a new block inside that section. To create a block after the section, the user moves the cursor past the closing fence and presses Enter, or uses `Insert Block Below` from the context menu.

**`Shift+Enter`** inserts a soft line break (single newline within the same block). Renders as a line break in both Outline and Design mode. Exported as a line break in PDF and DOCX.

### 4.6 Backspace — deletion and block merging

`Backspace` at offset > 0 deletes the grapheme cluster before the cursor.

`Backspace` at offset 0 (start of a block):
- If the preceding block is a standard block: merge the current block into the preceding block. The cursor lands at the junction point. If the styles differ, the merged content adopts the preceding block's style.
- If the preceding block is a cell: the cursor moves to the end of the cell's placeholder — cells cannot be merged into.
- If the current block is the first block in a section: the cursor moves to the block before the section's opening fence — the merge does not cross the section boundary without confirmation.

`Delete` at the end of a block merges the next block into the current block (inverse of the above).

### 4.7 Tab

| Context | Behaviour |
|---|---|
| Start of a list item | Indent one level (nested list) |
| Start of any other block | No effect (Tab does not indent paragraphs — use left-indent via style) |
| Inside a table cell | Move to next cell; at last cell of last row, insert a new row |
| In Outline mode, block focused (not text editing) | Indent section nesting level |

`Shift+Tab` is the reverse of all the above.

### 4.8 Undo / redo

`Cmd+Z` — undo. `Cmd+Shift+Z` — redo.

Undo granularity:
- Consecutive characters typed within 300ms are coalesced into one undo entry
- A style change is one undo entry
- A block split or merge is one undo entry
- A block move is one undo entry
- A cell insertion is one undo entry

The undo stack is cleared when a version is saved (`Save Version`). The user cannot undo across a version boundary — versions are the persistence mechanism for that.

### 4.9 Copy, cut, paste

`Cmd+C` — copy selection. `Cmd+X` — cut selection. `Cmd+V` — paste.

**Copy / cut** places a `.fvm` fragment on the clipboard (structured, with style information) plus a plain-text fallback.

**Paste from another folivm document:** the structured blocks are inserted at the cursor position. Styles are preserved. If a style is not present in the current theme, the block is inserted with the unknown style name preserved; the style name is shown in the style badge with a warning indicator.

**Paste from an external source** (Word, browser, email):
1. Content is stripped to plain text. All formatting is discarded.
2. The text is inserted as one or more Body blocks, split at paragraph breaks.
3. A toast notification appears at the bottom of the editor: "Pasted as plain text. [Apply Style]"
4. Clicking "Apply Style" opens the block style picker with the newly pasted blocks pre-selected.
5. The toast dismisses after 8 seconds or on any edit.

`Cmd+Shift+V` — paste as plain text without the toast (skips the notification, inserts directly as Body).

---

## 5. Style Application

### 5.1 Block style picker

The block style picker is the primary formatting interface. It applies a named block style to the active block (or all blocks in a multi-block selection).

**Invocation:**
- `Cmd+/` — opens the style picker as a command palette (keyboard shortcut — primary)
- Click the style name in the toolbar — opens the same picker
- Right-click a block → "Apply Block Style" — opens the picker

**Picker appearance:**
A floating command palette appears centred in the editor, approximately 480px wide. It contains:
- A text input field (auto-focused)
- A filtered list of all block styles from the active theme
- Each list item: style name, a short line of text rendered in that style as preview
- The currently applied style is shown at the top of the list, highlighted
- A "Block style" header and an "Inline style" header separate the two categories if inline styles are also shown (they are shown when the picker is opened with a text selection)

**Filtering:**
Typing filters the list by style name (fuzzy match). Prefix match ranks first.

**Navigation:**
`↑` / `↓` navigate the list. `Enter` applies the focused style and dismisses. `Escape` dismisses without applying.

**Application:**
The style is applied immediately to the active block. If a multi-block selection is active (text spanning more than one block), the style is applied to all blocks that contain selected text.

**Heading shortcut:** `Cmd+1` through `Cmd+6` apply Heading1 through Heading6 directly, without opening the picker.

### 5.2 Inline style picker

The inline style picker applies a named inline style to the selected text.

**Invocation:**
The inline style toolbar appears automatically when a text selection is established (mouseup or keyboard selection complete). It appears 6px above the midpoint of the selection, on the line above.

The toolbar contains:
- All inline-scope styles from the active theme, shown as labelled buttons
- Built-in shortcuts: `B` (Strong), `I` (Emphasis), `</>` (Code)
- A clear button (`×`) to remove all inline styles from the selection

**Keyboard shortcuts:**
| Shortcut | Effect |
|---|---|
| `Cmd+B` | Toggle Strong on selection |
| `Cmd+I` | Toggle Emphasis on selection |
| `Cmd+`` ` `` ` | Toggle Code on selection |
| `Cmd+.` | Toggle Superscript on selection |
| `Cmd+,` | Toggle Subscript on selection |

**No `Cmd+U`:** Underline has no built-in semantic equivalent. Pressing `Cmd+U` shows a brief tooltip: "Use a named inline style for emphasis. Try DefinedTerm or CrossReference." This is not an error — it is a teaching moment.

**Toggle behaviour:** If the entire selection already has the target inline style, applying it again removes it (toggle). If the selection is mixed (some characters have the style, others do not), applying the style applies it to the entire selection.

### 5.3 Style badges (Outline mode)

In Outline mode, every block displays its style name as a small badge in the left margin. The badge uses the same colour as the style's primary text colour at reduced opacity.

Clicking the badge opens the block style picker for that block.

A block with an unknown style (style not in the active theme) shows the badge with a warning colour and a `!` prefix.

### 5.4 Preventing ad-hoc formatting

The application intercepts all operating system font/format shortcuts before they reach the OS text system:

| User action | folivm response |
|---|---|
| Right-click → Font (macOS) | Intercepted. No effect |
| Format menu (if exposed by OS) | Not exposed |
| Paste with formatting | Strips formatting (see §4.9) |
| Drag font from Font Book | Intercepted. No effect |
| `Cmd+U` (underline) | Tooltip explaining semantic styles |

The toolbar never exposes a font picker, colour picker, size picker, or raw bold/italic toggle as primary controls.

---

## 6. Block Operations

### 6.1 Block creation

**Insert Block Below:** `Cmd+Return` — inserts a new Body block below the active block, regardless of cursor position within the block. Cursor moves to the new block. This is the "I want a new paragraph" shortcut.

**Insert Block Above:** `Cmd+Shift+Return` — inserts a new Body block above the active block.

**Insert via slash command:** Typing `/` at the very start of an empty block (offset 0, no text) opens an insert picker inline within the block. The picker lists: all block styles, all cell types. Typing filters. Enter inserts. Escape cancels (the `/` character is then typed into the block normally).

The slash command is the quick-insert mechanism for cells and style-specific blocks without reaching for the toolbar.

### 6.2 Block deletion

`Cmd+Shift+Delete` — deletes the active block entirely. If the block contains text, a brief undo toast appears: "Block deleted. [Undo]".

Cells are deleted the same way. There is no separate "delete cell" confirmation — deletion is always undoable.

### 6.3 Block movement (Outline mode)

`Alt+↑` / `Alt+↓` — move the active block up or down past the adjacent block.

Movement is animated (200ms slide). The moved block is highlighted for 500ms after landing.

Moving into a section: when a block is moved to a position inside a section's fence, it becomes a child of that section.

Moving out of a section: when the last block inside a section is moved above the section's opening fence, the section remains but is now empty (an empty section is valid and can be deleted separately).

**Drag to reorder:** In Outline mode, hovering over a block reveals a drag handle (⠿) in the left margin. Click-hold and drag vertically. A blue insertion indicator line shows the drop target. Release to place.

### 6.4 Section folding (Outline mode)

Every section block shows a chevron `›` in the left margin, to the left of the block ID area.

| Interaction | Result |
|---|---|
| Click `›` | Fold section. Children hidden. Chevron rotates to `›` (pointing right). A count badge shows how many blocks are hidden |
| Click `›` (folded) | Unfold section |
| `Cmd+Alt+[` | Fold all sections |
| `Cmd+Alt+]` | Unfold all sections |
| `Alt+[` | Fold the section containing the cursor |
| `Alt+]` | Unfold the section containing the cursor |

Folded state is UI state — it is not saved to the `.fvm` file. Reopening a document always shows all sections unfolded.

### 6.5 Focus mode (Outline mode)

`Cmd+Shift+F` — enters focus mode. The section containing the cursor expands to fill the editor. All other content is hidden. The title bar shows the section title. A breadcrumb shows position within the document.

`Escape` or `Cmd+Shift+F` again — exits focus mode.

Focus mode is a display filter. The document is not changed. Export operates on the full document.

---

## 7. Cell Interactions

### 7.1 Inserting a cell

Via slash command (§6.1): type `/image`, `/math`, `/include` at the start of an empty block.

Via toolbar Insert menu: click the Insert dropdown → choose cell type.

Via keyboard: `Cmd+Shift+I` opens the Insert Cell picker — same list as the slash command, as a command palette.

When a cell is inserted, it replaces the current empty block (or inserts below the current block if the block is not empty). The cursor moves into the cell's first editable field.

### 7.2 cell:image

**Insert flow:**
1. The cell is created with an empty `alt` field, no `src`.
2. A native file picker opens immediately for raster images, or the user can paste SVG source.
3. After selecting a file, the image is copied into the document's `assets/` directory (created if it does not exist).
4. The `alt` text field is immediately focused with a prompt: "Describe this image for screen readers (required)". The field is highlighted until text is entered.
5. Alt text is required. The cell cannot leave this edit state without alt text — a validation indicator appears if the user tries to move focus away from an empty alt field.

**Caption:** An optional caption field below the image. Clicking it activates text editing. The `{fig}` token is available here. If left empty, no caption line is rendered or exported.

**Width adjustment (Design mode):** A resize handle appears on the right edge of the image. Dragging adjusts the `width` attribute. The width is shown as a tooltip in pt while dragging. Values snap to 10pt increments; hold Shift to snap to 1pt.

**Alignment:** Three alignment buttons appear in the floating cell toolbar when the cell is focused: Left, Centre, Right.

### 7.3 cell:math

**Insert flow:**
1. The cell is created with an empty LaTeX source field.
2. The source field is immediately active. It is a plain text editor (monospace, no syntax highlighting in v1.0).
3. Below the source field, a live preview renders the equation using KaTeX. The preview updates as the user types, with a brief debounce (150ms).
4. `Tab` or `Escape` commits the equation and returns focus to the document.
5. An invalid LaTeX expression shows the KaTeX error message in the preview area.

**Edit an existing equation:** Click the rendered equation or press `Enter` with the cursor on the cell — this opens the source field for editing.

### 7.4 cell:include

**Insert flow:**
1. The Content Library panel opens automatically (if not already open) and focused.
2. The user clicks a library item or uses arrow keys + Enter to select one.
3. The cell is created with the selected `ref` and the current `version`.
4. The library panel returns focus to the editor.

**Library update notification:** When a newer version of a referenced item is available, a pill badge appears on the cell in Outline mode: "Update available: 2.4". In Design mode, the update badge appears in the right rail when the cursor is on the cell.

Clicking the badge (or pressing `U` with the cell focused): "Update to version 2.4? This will create a version commit. [Update] [Cancel]"

Confirming the update: the version attribute changes, the document is saved, a git commit is created with the message "Update [library.item-name] from 2.3 to 2.4".

### 7.5 Cell placeholder interactions

Cells without a v1.0 renderer (cell:data, cell:ai, cell:diagram, cell:citation, cell:signature) render as a placeholder block showing:
- The cell type name and icon
- The key attributes (e.g., for cell:ai: the prompt text, the status)
- "Extension required" if no handler is registered, or the extension name if one is installed but not responding

The placeholder is not editable from within the editor. The raw cell attributes can be viewed and edited by right-clicking the placeholder → "Edit cell attributes" — this opens a YAML attribute editor in a floating panel.

---

## 8. Design Mode — Ruler and Layout

### 8.1 Ruler

The horizontal ruler spans the full width of the editor canvas. It is calibrated in pt (or mm, switchable in settings). It shows:
- Document margins as filled grey regions at left and right
- Content area in white
- Tab stop indicators as small triangles in the content area
- Current cursor position as a faint vertical line on the ruler

**Margin drag:** Hover over a margin boundary — the cursor changes to a horizontal resize cursor. Click and drag to adjust the margin. The margin value tooltip updates live. Releasing the drag updates `frontmatter.margins` and commits as an auto-save (not a version save).

**Tab stop:** Click in the content area of the ruler to place a tab stop at that position. A tab stop indicator appears. Drag an existing tab stop to move it. Drag a tab stop off the ruler (above or below) to remove it.

Tab stops are per-style — they are set in the theme's style definition. In v1.0, tab stops on the ruler are visual-only indicators for the currently active block's style. Direct ruler tab-stop editing sets them on the active style in the theme. A confirmation prompt: "This will update the tab stops for all [StyleName] blocks. [Update Style] [Cancel]"

**Double-click ruler:** Opens the Page Setup dialog (page size, orientation, margins with numeric fields).

### 8.2 Zoom

Zoom range: 50% to 200%. Default: 100%.

| Interaction | Result |
|---|---|
| `Cmd++` / `Cmd+-` | Zoom in / out by 10% |
| `Cmd+0` | Reset to 100% |
| Scroll on zoom indicator in status bar | Continuous zoom |
| Pinch (trackpad) | Continuous zoom |
| Click zoom % in status bar | Text field appears, type exact value |

Zoom is a display property only. It does not affect the document or export.

### 8.3 Page navigation (Design mode)

`Cmd+PgUp` / `Cmd+PgDn` — jump to previous / next page.

The current page number is shown in the status bar. Clicking the page indicator in the status bar opens a "Go to page" input.

---

## 9. Mode Switching

The mode switch control is in the left side of the toolbar: a segmented button `Outline | Design`.

Clicking either segment switches to that mode instantly. No loading, no re-parse. The current scroll position is maintained relative to the active block — if the cursor was on block 24 in Outline mode, Design mode scrolls to show block 24.

`Cmd+Shift+O` — switch to Outline mode.
`Cmd+Shift+D` — switch to Design mode.

On mode switch:
- The ruler appears or disappears
- The toolbar zoom controls appear or disappear
- The editor canvas content re-renders in the new mode
- The cursor remains on the same block

---

## 10. Content Library Panel

The library panel is accessed via the Activity Bar (book icon). It shows two sections: Project Library and Global Library, each as a collapsible tree.

**Library item entry:**
Each item shows: name, current version, a short description if provided.

**Insert into document:** Double-click a library item, or select it and press Enter — inserts a `cell:include` reference at the cursor position in the editor.

**Preview on hover:** Hovering over a library item for 500ms shows a tooltip preview of the first three blocks of the item.

**Save selection as library item:**
1. Select one or more blocks in the editor (selection must not cross a cell boundary)
2. Right-click → "Save to Library…"
3. A dialog prompts: Name, Description, Version (default: 1.0), Destination (Project / Global)
4. Confirming saves the selection as a `.fvm` fragment in the appropriate library directory and adds it to the index

**Edit a library item:** Right-click → "Edit Item" — opens the item's `.fvm` fragment as a tab in the editor. Saving the file increments a minor version and updates the index.

---

## 11. Versioning Panel

The versioning panel (clock icon in Activity Bar) has three sub-panels: History, Drafts, Changes.

### 11.1 History

A list of all version saves (git commits) in reverse chronological order. Each entry: message, author, date, short hash.

**Open a version:** Click an entry → a read-only preview of the document at that version appears in a new tab, labelled "[title] (version: [message])". It is not editable. It can be exported.

**Compare versions:** Select two entries (Shift+click or Cmd+click) → "Compare" button activates. Clicking it opens the Diff view (see §11.3).

### 11.2 Drafts

A list of all git branches. Each entry: branch name, creation date, last change date.

**Create Draft:**
1. `File → Create Draft` or `Cmd+Shift+G`
2. Prompt: "Draft name: [text field]"
3. Name is used as the git branch name (spaces become hyphens)
4. After confirming, the current branch switches to the new draft

**Switch Draft:** Click a draft entry → if the current document has unsaved changes, prompt to save a version first. Then switch branch. The document reloads from the branch's latest commit.

**Combine Draft (merge):** Right-click a draft → "Combine into main draft". If there are no conflicts, a merge commit is created. If there are conflicts, the Diff view opens showing the conflicts as merge conflict blocks (see §11.3).

### 11.3 Diff view (Changes)

The Changes tab shows the diff between the current working state and the last version save.

**Block-level diff:** Each changed block is shown with before/after text, highlighted. Added blocks are green-bordered. Removed blocks are red-bordered. Modified blocks show line-level text changes with word-level highlighting.

**Compare two versions:** When opened from the History panel, shows the same diff view between two selected versions.

**Conflict blocks (merge conflicts):** A conflict block shows both versions with accept/reject buttons:
```
┌─ [Branch: main] ──────────────────── [Accept] ─┐
│ The contractor agrees to provide the services.  │
└─────────────────────────────────────────────────┘
┌─ [Branch: client-review] ─────────── [Accept] ─┐
│ The contractor agrees to provide the listed     │
│ professional services as defined in Schedule A. │
└─────────────────────────────────────────────────┘
```

Clicking "Accept" on a block accepts that version and removes the conflict marker. All conflicts must be resolved before a Save Version can be performed.

---

## 12. Save Version Flow

**Trigger:** `Cmd+Shift+S` or `File → Save Version`.

1. If the document has a dirty RunBuffer (unsaved text in the active block), it is committed first (transparent to the user).
2. A compact input field appears in the toolbar area (inline, not a modal): "Version message: [_________________________] [Save] [Cancel]"
3. The field is auto-focused. The user types a message and presses Enter or clicks Save.
4. The version is saved (git commit). The status bar's save indicator updates to "Just now" and the change count resets to 0.
5. The undo stack is cleared.

If no changes have been made since the last version save, `Cmd+Shift+S` shows a brief toast: "No changes since last version."

---

## 13. Search and Replace

### 13.1 Find within document

`Cmd+F` — opens the Find bar at the bottom of the editor canvas (not a floating dialog).

The Find bar contains: text field, match count ("3 of 12"), Previous (`Shift+Enter`), Next (`Enter`), Case sensitive toggle (`Aa`), Regex toggle (`.*`), Close (`Escape`).

Matches are highlighted on the canvas. The active match is highlighted in a different colour. The view scrolls to keep the active match visible.

`Escape` or clicking outside the Find bar — closes it and restores focus to the editor.

### 13.2 Project-wide search

`Cmd+Shift+F` — opens the Search panel in the sidebar (switches the Activity Bar to the Search panel).

The search panel contains: query field, Replace field (collapsible), file filter field, Case/Regex toggles.

Results stream in as they arrive (ripgrep, streaming). Grouped by file. Each result shows the file name, line number, and the matching line with the match highlighted.

Clicking a result: opens the file in a tab and scrolls to the matching line, with the match highlighted.

**Replace:**
1. Enter a replacement string in the Replace field
2. Click the Replace button next to a single result to replace one occurrence
3. Click "Replace All" to replace across all results
4. A preview dialog shows a summary: "Replace 23 occurrences across 7 files? [Preview] [Replace All] [Cancel]"
5. Preview shows a diff for each affected file, with per-file checkboxes to exclude files
6. On confirming: replacements are applied atomically, a single git commit is created: "Replace '[query]' with '[replacement]' in 7 files"

---

## 14. Theme Interaction

### 14.1 Variables panel

The Variables panel (right rail, Variables tab) shows all theme collections and their current mode selections.

Each collection is a row: "Brand: default ▾" where the mode name is a dropdown. Clicking the dropdown shows all available modes for that collection.

Selecting a mode: the theme re-resolves immediately. All blocks in the editor re-render. The transition takes less than 300ms. No confirmation is needed — mode switching is not a document change, it is a display preference stored in the project settings.

### 14.2 Styles panel

The Styles panel (right rail, Styles tab) lists all named styles in the active theme, grouped by scope (Block, Inline).

Each style entry: style name, a one-line preview rendered in that style.

Clicking a style entry with a block selected → applies the style. Same as using the style picker.

Double-clicking a style entry → opens the style definition in the `.fvm-theme` file for editing (in a new tab).

---

## 15. Export Flow

`File → Export → PDF` or `File → Export → DOCX`, or `Cmd+Shift+E` (opens export dialog).

**Export dialog** (compact, not a full screen):
```
Export document
Format:   ● PDF   ○ DOCX
Pages:    ● All   ○ Range [1] to [12]
PDF only: ☑ Accessible PDF (PDF/UA)
          ☑ Embed fonts
[Export…] [Cancel]
```

Clicking "Export…" opens a native save dialog. After the user confirms the filename and location, export begins.

**Progress:** The toolbar shows a slim progress bar for the duration. Large documents may take 2–5 seconds. An "Export complete" toast appears on completion with a "Show in Finder/Explorer" button.

**Export error:** If export fails (font missing, write permission denied), the error is shown in a toast with a "Show details" link that opens the error log in the Developer panel.

---

## 16. Extension Panels

Extensions may register sidebar panels. Registered panels appear in the Activity Bar below the built-in icons, separated by a divider.

Extension panel iframes are sandboxed. They can send messages to the shell (to trigger IPC calls to the extension's Deno runtime) but cannot access the editor canvas or document state directly.

**Extension toolbar commands:** Extensions may register commands that appear in the toolbar Insert menu or in the right-click context menu. These are labelled with the extension's name.

**Extension errors:** If an extension panel fails to load or an extension command throws an error, a warning badge appears on the extension's Activity Bar icon. The error is logged in the Developer panel (`Help → Developer Panel` or `Cmd+Shift+I`).

---

## 17. Keyboard Reference

### Navigation

| Shortcut | Action |
|---|---|
| `↑` `↓` `←` `→` | Move cursor |
| `Cmd+←` `Cmd+→` | Start / end of line |
| `Alt+←` `Alt+→` | Previous / next word |
| `Cmd+↑` `Cmd+↓` | Start / end of document |
| `Home` `End` | Start / end of visual line |
| `PgUp` `PgDn` | Scroll viewport |
| `Cmd+PgUp` `Cmd+PgDn` | Previous / next page (Design mode) |
| `Cmd+G` | Go to line / block number |

### Selection

| Shortcut | Action |
|---|---|
| `Shift` + any navigation | Extend selection |
| `Cmd+A` | Select all |
| Double-click | Select word |
| Triple-click | Select block |

### Editing

| Shortcut | Action |
|---|---|
| `Enter` | Split block / new list item |
| `Shift+Enter` | Soft line break |
| `Cmd+Return` | Insert Body block below |
| `Cmd+Shift+Return` | Insert Body block above |
| `Backspace` | Delete left / merge blocks |
| `Delete` | Delete right / merge blocks |
| `Cmd+Shift+Delete` | Delete active block |
| `Cmd+Z` | Undo |
| `Cmd+Shift+Z` | Redo |
| `Cmd+X` `Cmd+C` `Cmd+V` | Cut / copy / paste |
| `Cmd+Shift+V` | Paste as plain text |
| `/` (at block start) | Insert picker |

### Styles

| Shortcut | Action |
|---|---|
| `Cmd+/` | Open block style picker |
| `Cmd+1` … `Cmd+6` | Apply Heading1 … Heading6 |
| `Cmd+B` | Toggle Strong (inline) |
| `Cmd+I` | Toggle Emphasis (inline) |
| `` Cmd+` `` | Toggle Code (inline) |
| `Cmd+.` | Toggle Superscript |
| `Cmd+,` | Toggle Subscript |

### Block operations

| Shortcut | Action |
|---|---|
| `Alt+↑` `Alt+↓` | Move block up / down |
| `Alt+[` | Fold active section |
| `Alt+]` | Unfold active section |
| `Cmd+Alt+[` | Fold all sections |
| `Cmd+Alt+]` | Unfold all sections |
| `Cmd+Shift+F` | Toggle focus mode |

### Mode and view

| Shortcut | Action |
|---|---|
| `Cmd+Shift+O` | Outline mode |
| `Cmd+Shift+D` | Design mode |
| `Cmd++` `Cmd+-` | Zoom in / out |
| `Cmd+0` | Reset zoom to 100% |
| `Cmd+B` | Toggle sidebar |
| `Cmd+Shift+B` | Toggle right rail |

### File and versioning

| Shortcut | Action |
|---|---|
| `Cmd+N` | New document |
| `Cmd+O` | Open project |
| `Cmd+S` | (No-op — auto-save is continuous) |
| `Cmd+Shift+S` | Save version |
| `Cmd+Shift+G` | Create draft |
| `Cmd+Shift+E` | Export |
| `Cmd+W` | Close tab |
| `Cmd+Shift+W` | Close project |
| `Cmd+Q` | Quit |

### Panels and search

| Shortcut | Action |
|---|---|
| `Cmd+F` | Find in document |
| `Cmd+Shift+F` | Project-wide search |
| `Cmd+Shift+H` | Project-wide replace |
| `Cmd+Shift+I` | Developer panel |
| `Cmd+1` … `Cmd+9` | Switch to tab by position (when not in style shortcut context) |
| `Cmd+Shift+]` `Cmd+Shift+[` | Next / previous tab |

---

## 18. Empty States

### Empty block
A block with no text shows the style name as ghost text (muted, styled like placeholder text). Ghost text is not content. It disappears on the first keypress. It is not exported or searched.

### Empty document
A new document shows one empty Body block. Ghost text reads "Start writing, or type / to insert…". The style picker in the toolbar is highlighted to draw attention.

### Empty project
An Explorer panel with no `.fvm` files shows: "No documents yet. [New Document]"

### No search results
The search panel shows: "No results for '[query]'" with a suggestion to try a simpler query or check the file filter.

### No library items
The library panel shows: "Library is empty. Select text in the editor and right-click to save your first item."

### No version history
The versioning panel History tab shows: "No saved versions yet. Use Cmd+Shift+S to save your first version."

---

## 19. Error States

### File cannot be saved
Toast: "Could not save '[filename]'. Check disk space and permissions. [Retry] [Save As…]"
The document remains open and editable. Auto-save continues attempting to save every 30 seconds.

### Extension fails to load
Activity Bar icon for the extension shows a warning badge.
The extension's panel shows: "This extension could not be loaded. [View error]"
Clicking "View error" opens the Developer panel with the extension's error log.

### Library item not found
A `cell:include` whose reference cannot be resolved shows: "⚠ Library item not found: [ref] v[version]". The block renders at the correct height with a warning border.

### Theme file not found or invalid
A banner at the top of the editor: "Theme file could not be loaded. Using default theme. [Locate file] [Use default]"

### Git operation fails
The versioning panel shows the error inline: "Could not save version: [git error message]. [Retry]"

### Export fails
Toast: "Export failed: [error summary]. [Show details] [Retry]"
Clicking "Show details" opens the Developer panel.

---

*This document specifies interaction behaviour. Visual design (colours, typography, spacing of the application chrome) is specified in the default `.fvm-theme` file bundled with the application.*
