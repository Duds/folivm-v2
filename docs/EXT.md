# folivm — Extension API Specification
**Version:** 0.1 (draft)
**Date:** 2026-03-13
**Status:** In review
**Depends on:** FORMAT.md v1.0, SCOPE.md v1.0, SAD.md v0.2

---

## 1. Purpose

This document specifies the public API that folivm exposes to third-party extensions. It defines everything an extension author needs to know to build a working extension: the package format, the permission model, the runtime environment, and every API surface.

The target reader is an extension developer. folivm internals are referenced only where they clarify a contract.

---

## 2. Extension Model

### 2.1 Isolation

Each extension runs in a dedicated **Deno Core** runtime embedded in `folivm-native`. Deno Core is a Rust-embedded JavaScript/TypeScript engine. It provides:

- A V8 isolate per extension — extensions cannot read each other's memory
- No default access to the file system, network, or OS APIs
- All capabilities provided exclusively through `folivm` global APIs declared in the manifest

The extension runtime has no access to the WASM module (`folivm-core`), the document model, or the renderer. All interactions happen through the structured `folivm.*` API surface described in this document.

### 2.2 Communication model

```
Extension Runtime (Deno Core)
        ↕  Deno Ops (Rust FFI)
folivm-native (Tauri backend)
        ↕  Tauri IPC
Shell (TypeScript, WebView)
        ↕  postMessage (sandboxed)
Panel iframe (HTML, if registered)
```

The extension runtime is the single point of contact. Panel iframes (HTML UIs) communicate with the extension runtime through a message-passing bridge exposed by the shell. Panels cannot call `folivm.*` APIs directly.

### 2.3 Runtime environment

The extension runtime is TypeScript-compatible via Deno Core's built-in transpiler. Extensions are written in TypeScript or plain JavaScript.

Available globals:
- `folivm` — the complete API surface (see sections 5–12)
- `console` — output to the extension developer console
- `fetch` — available only if `permissions.network` is declared
- `Deno` — **not** exposed. Extensions do not have access to the Deno standard library or Deno APIs

TypeScript type definitions are distributed as `@folivm/extension-types` and are bundled in the Folivm developer package. Extensions should import types from this package.

---

## 3. Extension Package Format

An extension is distributed as a `.fvmext` file. A `.fvmext` file is a ZIP archive with the following layout:

```
my-extension.fvmext
├── manifest.json          # required — extension metadata and permissions
├── index.ts               # required — entry point
├── panels/                # optional — HTML files for sidebar panels
│   └── main.html
├── assets/                # optional — bundled static files
│   └── logo.svg
└── lib/                   # optional — additional modules
    └── helpers.ts
```

### 3.1 Manifest schema

`manifest.json` must be valid JSON conforming to the following schema:

```json
{
  "id": "com.example.mermaid",
  "name": "Mermaid Diagrams",
  "version": "1.0.0",
  "description": "Renders cell:diagram blocks using Mermaid syntax",
  "author": "Example Corp <hello@example.com>",
  "homepage": "https://example.com/folivm-mermaid",
  "license": "MIT",
  "min_folivm_version": "1.0.0",

  "entry": "index.ts",

  "permissions": {
    "cell.render": ["diagram"],
    "cell.export": ["diagram"],
    "panel": true,
    "network": ["https://mermaid.ink"]
  },

  "panels": [
    {
      "id": "mermaid-preview",
      "title": "Mermaid Preview",
      "icon": "diagram-project",
      "file": "panels/main.html"
    }
  ],

  "library": {
    "path": "library/"
  }
}
```

#### 3.1.1 Required fields

| Field | Type | Description |
|---|---|---|
| `id` | string | Reverse-domain identifier. Must be globally unique. Use your domain. |
| `name` | string | Display name shown in the Extensions panel |
| `version` | string | SemVer string |
| `description` | string | One-sentence description |
| `min_folivm_version` | string | Minimum folivm version required (SemVer) |
| `entry` | string | Relative path to the entry point file |
| `permissions` | object | Declared permissions (see section 4) |

#### 3.1.2 Optional fields

| Field | Type | Description |
|---|---|---|
| `author` | string | Author name and optional email |
| `homepage` | string | URL to extension documentation |
| `license` | string | SPDX license identifier |
| `panels` | array | Panel registrations (required if `permissions.panel` is true) |
| `library` | object | Path to bundled library items (`library.path`) |

### 3.2 Entry point

The entry point is executed once when the extension loads. It must call all registration functions synchronously during load. Any registration call made after the entry point returns is silently dropped.

```typescript
import type { FolvimAPI } from "@folivm/extension-types";
declare const folivm: FolvimAPI;

// All registrations happen here, at the top level
folivm.cells.register("diagram", {
  render: renderDiagram,
  exportPdf: exportDiagramPdf,
  exportDocx: exportDiagramDocx,
});

folivm.panels.register("mermaid-preview", {
  onMessage: handlePanelMessage,
});

folivm.on("document:opened", onDocumentOpened);
```

---

## 4. Permission System

Permissions are declared in `manifest.json` and presented to the user at install time. The user must accept all declared permissions before the extension is activated. An extension that attempts to use an API it has not declared permission for will receive a `PermissionDeniedError` at runtime.

Permissions cannot be granted or expanded after install. If an extension update adds new permissions, the user is prompted to re-approve.

### 4.1 Permission reference

| Permission key | Value | Grants |
|---|---|---|
| `document.read` | `true` | Read document blocks, metadata, and selection |
| `document.write` | `true` | Insert, update, and delete blocks |
| `cell.render` | `string[]` | Render handler for named cell types (e.g. `["diagram"]`) |
| `cell.export` | `string[]` | Export handler for named cell types |
| `data.source` | `true` | Register a data source for `cell:data` resolution |
| `panel` | `true` | Register sidebar panels |
| `export.hook` | `true` | Register pre-export transform hooks |
| `library.contribute` | `true` | Contribute items to the Content Library |
| `network` | `string[]` | Make HTTP/HTTPS requests to declared host patterns |
| `filesystem.read` | `string[]` | Read files from declared paths (glob patterns, relative to document) |

`filesystem.write` is not a v1.0 permission. Extensions cannot write to the file system in v1.0.

`document.read` is implicitly granted to extensions that declare `cell.render`, `cell.export`, or `data.source`.

### 4.2 Network permission

Network patterns are URL prefixes. Only HTTPS is permitted. HTTP is rejected.

```json
"network": ["https://api.zotero.org", "https://mermaid.ink"]
```

Requests to any origin not listed will throw `NetworkPermissionDeniedError`.

---

## 5. Core API (`folivm`)

The `folivm` global is available in all extension runtimes with no permission requirement.

### 5.1 Extension metadata

```typescript
folivm.extension.id: string          // the extension's manifest id
folivm.extension.version: string     // the extension's manifest version
folivm.extension.name: string        // the extension's display name
```

### 5.2 Lifecycle events

```typescript
folivm.on(event: LifecycleEvent, handler: () => void): void
folivm.off(event: LifecycleEvent, handler: () => void): void
```

| Event | Fired when |
|---|---|
| `application:ready` | The extension host is fully initialised. Fired once at startup. |
| `application:quit` | The application is about to quit. Clean up resources. |
| `document:opened` | A document is opened or becomes the active document |
| `document:closed` | A document is closed |
| `document:saved` | A document is saved to disk (auto-save or Save Version) |
| `theme:changed` | The active theme or any theme mode is changed |

For `document:opened` and `document:closed`, the handler receives a `DocumentHandle`:

```typescript
folivm.on("document:opened", (doc: DocumentHandle) => {
  console.log(`Opened: ${doc.title}`);
});
```

### 5.3 Logging

```typescript
console.log(...args)    // info level
console.warn(...args)   // warning level
console.error(...args)  // error level
```

All output appears in the Extension Developer Console (View → Developer Console, or `Cmd+Shift+J`). Output is prefixed with the extension name and timestamp.

---

## 6. Document API (`folivm.document`)

Requires `document.read` or `document.write` permission.

### 6.1 Reading the document

```typescript
folivm.document.getMetadata(): Promise<DocumentMetadata>
```

Returns the document frontmatter:

```typescript
interface DocumentMetadata {
  title: string;
  author?: string;
  date?: string;
  lang?: string;
  page_size?: string;
  version?: string;
  [key: string]: unknown;  // custom frontmatter keys
}
```

```typescript
folivm.document.getBlocks(): Promise<Block[]>
```

Returns all top-level blocks in document order. See section 6.3 for the `Block` type.

```typescript
folivm.document.getBlock(id: string): Promise<Block | null>
```

Returns the block with the given UUID, or `null` if not found.

```typescript
folivm.document.getSelection(): Promise<Selection | null>
```

Returns the current cursor selection, or `null` if no document is focused.

```typescript
interface Selection {
  anchorBlockId: string;
  anchorOffset: number;
  focusBlockId: string;
  focusOffset: number;
  isCollapsed: boolean;
}
```

### 6.2 Writing the document

Requires `document.write` permission.

```typescript
folivm.document.insertBlock(
  block: BlockInsert,
  position: InsertPosition
): Promise<string>   // returns the new block's UUID
```

```typescript
interface InsertPosition {
  after?: string;    // UUID of block to insert after
  before?: string;   // UUID of block to insert before
  // If neither is specified, inserts at end of document
}
```

```typescript
folivm.document.updateBlock(
  id: string,
  update: Partial<BlockInsert>
): Promise<void>
```

```typescript
folivm.document.deleteBlock(id: string): Promise<void>
```

All write operations are undoable. They appear in the undo stack as a single operation attributed to the extension name.

### 6.3 Block types

```typescript
type Block =
  | ParagraphBlock
  | HeadingBlock
  | ListBlock
  | BlockquoteBlock
  | TableBlock
  | SectionBlock
  | CalloutBlock
  | CellBlock;

interface BaseBlock {
  id: string;           // UUID
  style?: string;       // applied style name
}

interface ParagraphBlock extends BaseBlock {
  type: "paragraph";
  content: Inline[];
}

interface HeadingBlock extends BaseBlock {
  type: "heading";
  level: 1 | 2 | 3 | 4 | 5 | 6;
  content: Inline[];
}

interface ListBlock extends BaseBlock {
  type: "list";
  ordered: boolean;
  items: ListItem[];
}

interface ListItem {
  content: Inline[];
  children: ListItem[];
}

interface BlockquoteBlock extends BaseBlock {
  type: "blockquote";
  content: Inline[];
}

interface TableBlock extends BaseBlock {
  type: "table";
  header: TableCell[][];
  rows: TableCell[][];
}

interface TableCell {
  content: Inline[];
  align?: "left" | "center" | "right";
}

interface SectionBlock extends BaseBlock {
  type: "section";
  id_attr?: string;
  title?: string;
  numbered?: boolean;
  page_break_before?: boolean;
  children: Block[];
}

interface CalloutBlock extends BaseBlock {
  type: "callout";
  style_name: string;
  content: Block[];
}

interface CellBlock extends BaseBlock {
  type: "cell";
  cell_type: string;       // "image" | "math" | "include" | "data" | "ai" | "diagram" | "citation" | "signature" | custom
  attributes: Record<string, string | number | boolean>;
  source?: string;         // fenced source body, if any
}
```

### 6.4 Inline types

```typescript
type Inline =
  | TextRun
  | EmphasisSpan
  | StrongSpan
  | CodeSpan
  | SuperscriptSpan
  | SubscriptSpan
  | StyledSpan
  | TokenSpan;

interface TextRun { type: "text"; text: string; }
interface EmphasisSpan { type: "em"; content: Inline[]; }
interface StrongSpan { type: "strong"; content: Inline[]; }
interface CodeSpan { type: "code"; text: string; }
interface SuperscriptSpan { type: "sup"; content: Inline[]; }
interface SubscriptSpan { type: "sub"; content: Inline[]; }
interface StyledSpan { type: "span"; style_name: string; content: Inline[]; }
interface TokenSpan { type: "token"; path: string; }
```

---

## 7. Cell Type Handler API (`folivm.cells`)

Requires `cell.render` and/or `cell.export` permission for the relevant cell types.

Cell type handlers provide folivm with render output and export output for cell types that the core does not render natively. An extension registers a handler for one or more cell types at load time.

### 7.1 Registration

```typescript
folivm.cells.register(cellType: string, handler: CellHandler): void
```

`cellType` must match a value declared in `permissions.cell.render` or `permissions.cell.export`. Only one handler per cell type per application session is allowed. If two extensions declare handlers for the same cell type, the user is prompted to choose one.

```typescript
interface CellHandler {
  render?: (cell: CellBlock) => Promise<CellRenderResult>;
  exportPdf?: (cell: CellBlock) => Promise<CellExportResult>;
  exportDocx?: (cell: CellBlock) => Promise<CellExportResult>;
}
```

All three functions are optional. An extension may provide only `render` (no export), only `exportPdf`, or any combination.

### 7.2 Render result

The `render` function returns content for the editor canvas:

```typescript
type CellRenderResult =
  | { type: "svg"; content: string }
  | { type: "html"; content: string; height: number }
  | { type: "text"; content: Inline[] }
  | { type: "error"; message: string };
```

| Result type | Use for | Notes |
|---|---|---|
| `svg` | Diagrams, charts, any vector output | SVG is embedded directly into the canvas at the block position. folivm handles sizing. |
| `html` | Interactive content (preview panes, live editors) | Rendered in a sandboxed iframe. `height` is the requested iframe height in pt. |
| `text` | Citation text, resolved data labels, inline content | Rendered with normal text layout using the block's active style. |
| `error` | When rendering fails | Displayed as a styled error placeholder. Does not block export. |

SVG and HTML renders are cached until the cell source changes or the extension signals invalidation.

### 7.3 Export result

The `exportPdf` and `exportDocx` functions return content for export:

```typescript
type CellExportResult =
  | { type: "svg"; content: string }
  | { type: "image"; data: Uint8Array; mime_type: "image/png" | "image/jpeg" }
  | { type: "text"; content: string }
  | { type: "runs"; content: DocxRun[] }
  | { type: "skip" }
  | { type: "error"; message: string };
```

| Result type | PDF | DOCX | Notes |
|---|---|---|---|
| `svg` | Embedded as vector | Embedded as image | Preferred for diagrams |
| `image` | Embedded as raster | Embedded as raster | Use when vector output is not possible |
| `text` | Rendered as paragraph | Rendered as paragraph | Use for citations, resolved data |
| `runs` | Not applicable | Character runs | DOCX only — rich inline content |
| `skip` | Cell omitted | Cell omitted | Use for decorative cells that add no exported value |
| `error` | Rendered as placeholder note | Rendered as placeholder note | Logged to export report |

```typescript
interface DocxRun {
  text: string;
  bold?: boolean;
  italic?: boolean;
  style_name?: string;    // maps to a DOCX character style
}
```

### 7.4 Invalidation

If a cell's rendered output changes without the source changing (e.g. data was refreshed from a network source), the extension signals re-render:

```typescript
folivm.cells.invalidate(cellType: string, blockId?: string): void
```

If `blockId` is omitted, all cells of that type are re-rendered.

### 7.5 Reference implementation: Mermaid diagrams

```typescript
import type { FolvimAPI, CellBlock, CellRenderResult, CellExportResult } from "@folivm/extension-types";
declare const folivm: FolvimAPI;

folivm.cells.register("diagram", {
  async render(cell: CellBlock): Promise<CellRenderResult> {
    const syntax = cell.attributes.syntax as string;
    if (syntax !== "mermaid") {
      return { type: "error", message: `Unknown diagram syntax: ${syntax}` };
    }
    if (!cell.source) {
      return { type: "error", message: "No diagram source" };
    }
    try {
      const svg = await renderMermaid(cell.source);
      return { type: "svg", content: svg };
    } catch (err) {
      return { type: "error", message: String(err) };
    }
  },

  async exportPdf(cell: CellBlock): Promise<CellExportResult> {
    // same render path — SVG embeds directly in PDF
    const result = await this.render!(cell);
    if (result.type === "svg") return result;
    return { type: "error", message: "Mermaid render failed for export" };
  },

  async exportDocx(cell: CellBlock): Promise<CellExportResult> {
    // DOCX cannot embed SVG vectors — rasterise instead
    const result = await this.render!(cell);
    if (result.type !== "svg") return { type: "error", message: "Mermaid render failed" };
    const png = await svgToPng(result.content);
    return { type: "image", data: png, mime_type: "image/png" };
  }
});

async function renderMermaid(source: string): Promise<string> {
  const resp = await fetch(`https://mermaid.ink/svg/${btoa(source)}`);
  if (!resp.ok) throw new Error(`Mermaid service error: ${resp.status}`);
  return resp.text();
}
```

---

## 8. Data Source API (`folivm.data`)

Requires `data.source` permission.

A data source resolves field references in `cell:data` blocks. The `source` and `field` attributes of a `cell:data` block are routed to the registered handler for that source ID.

### 8.1 Registration

```typescript
folivm.data.register(sourceId: string, handler: DataSourceHandler): void
```

`sourceId` is a string identifier that matches the `source` attribute in `cell:data` blocks. Example: a `cell:data` with `source: crm` routes to the handler registered with `sourceId: "crm"`.

```typescript
interface DataSourceHandler {
  // List all available fields (used for autocomplete in the editor)
  fields(): Promise<DataField[]>;

  // Resolve one field reference to a display string
  resolve(field: string, context: DocumentContext): Promise<string | null>;
}

interface DataField {
  name: string;         // the field identifier used in {source.field}
  label: string;        // human-readable label for the picker
  description?: string;
  type: "text" | "number" | "date" | "currency";
}

interface DocumentContext {
  documentId: string;
  metadata: DocumentMetadata;
}
```

The `resolve` function is called once per `cell:data` block when the document renders. If it returns `null`, the block renders as an unresolved placeholder (`[source.field]`).

### 8.2 Reference implementation: CSV data source

```typescript
import type { FolvimAPI, DataSourceHandler, DataField } from "@folivm/extension-types";
declare const folivm: FolvimAPI;

let csvData: Record<string, string> = {};

folivm.on("document:opened", async () => {
  // Load a CSV file from the assets/ folder bundled with the extension
  const raw = await folivm.assets.read("data/contacts.csv");
  csvData = parseCsv(raw);
});

folivm.data.register("crm", {
  async fields(): Promise<DataField[]> {
    return Object.keys(csvData).map(key => ({
      name: key,
      label: key.replace(/_/g, " "),
      type: "text",
    }));
  },

  async resolve(field: string): Promise<string | null> {
    return csvData[field] ?? null;
  },
});
```

---

## 9. UI Panel API (`folivm.panels`)

Requires `panel` permission. Panel registrations must also be declared in `manifest.json` under `panels`.

### 9.1 Registration

Panel HTML files are loaded in sandboxed iframes in the folivm sidebar. The panel manifest entry declares the file, title, and icon.

```typescript
folivm.panels.register(panelId: string, handler: PanelHandler): void
```

`panelId` must match an `id` declared in `manifest.json` under `panels`.

```typescript
interface PanelHandler {
  onMessage?: (message: unknown) => Promise<unknown> | void;
  onPanelVisible?: () => void;
  onPanelHidden?: () => void;
}
```

### 9.2 Panel–extension messaging

The panel iframe cannot call `folivm.*` APIs. All communication passes through the extension runtime.

**From panel HTML to extension runtime:**

```javascript
// In the panel HTML file
window.folivm.send({ type: "get-fields" });

window.folivm.onMessage((message) => {
  // receive responses from the extension runtime
  renderFields(message.fields);
});
```

**From extension runtime to panel:**

```typescript
// In the extension entry point
folivm.panels.register("my-panel", {
  async onMessage(message) {
    if (message.type === "get-fields") {
      const fields = await folivm.data.getFields("crm");
      return { fields };
    }
  }
});
```

`window.folivm` is injected by the shell into every registered panel iframe. It is not available to arbitrary web pages — only to HTML files from the `.fvmext` package.

### 9.3 Panel iframe sandbox

Panel iframes run with the following sandbox attributes:

```
allow-scripts allow-same-origin
```

Panels may not:
- Navigate or open new windows
- Access `localStorage`, `sessionStorage`, or `IndexedDB` directly (use the extension runtime for persistent state via `folivm.storage`)
- Make direct network requests (all network access routes through the extension runtime)

### 9.4 Extension storage

```typescript
folivm.storage.get(key: string): Promise<string | null>
folivm.storage.set(key: string, value: string): Promise<void>
folivm.storage.delete(key: string): Promise<void>
```

Storage is scoped to the extension. Keys are strings. Values must be serialisable to JSON strings (extension is responsible for serialisation). Storage is persisted in the OS application support directory, not in the project directory.

---

## 10. Export Hook API (`folivm.export`)

Requires `export.hook` permission.

Export hooks run before the export pipeline serialises the document. They receive the full document model and may transform it — adding, removing, or rewriting blocks. Hooks run in order of registration and cannot be reordered between extensions.

### 10.1 Registration

```typescript
folivm.export.onBefore(
  formats: ExportFormat[],
  hook: ExportHook
): void

type ExportFormat = "pdf" | "docx" | "all";

type ExportHook = (
  document: ExportDocument,
  context: ExportContext
) => Promise<ExportDocument> | ExportDocument;
```

### 10.2 ExportDocument

```typescript
interface ExportDocument {
  metadata: DocumentMetadata;
  blocks: Block[];    // mutable copy — changes do not affect the open document
}

interface ExportContext {
  format: "pdf" | "docx";
  pageRange?: { from: number; to: number };
}
```

The `ExportDocument` passed to the hook is a deep copy. Mutations do not affect the live document. The hook returns the transformed document. If the hook throws, the export fails with an error displayed to the user.

### 10.3 Example: field resolution before export

```typescript
folivm.export.onBefore(["pdf", "docx"], async (doc, ctx) => {
  // Walk all cell:data blocks and inline resolved values
  doc.blocks = await resolveDataCells(doc.blocks);
  return doc;
});
```

---

## 11. Library Contribution API (`folivm.library`)

Requires `library.contribute` permission.

Extensions may contribute items to the Content Library. Contributed items appear in the Library panel under an extension-labelled section and can be inserted into documents via `cell:include`.

### 11.1 Bundled library items

Library items bundled with the extension are declared in `manifest.json`:

```json
"library": {
  "path": "library/"
}
```

All `.fvm` fragment files in the declared path are registered as library items on extension load. The item name is the filename without the extension. The initial version is `1.0`.

Items contributed this way are static. They cannot be edited by the user.

### 11.2 Dynamic library contributions

```typescript
folivm.library.contribute(item: LibraryItem): Promise<void>

interface LibraryItem {
  id: string;           // stable identifier, scoped to the extension
  name: string;         // display name in the Library panel
  description?: string;
  version: string;      // SemVer
  content: Block[];     // the blocks to insert on use
}
```

Dynamic contributions are re-registered on each load. If a version number is higher than the previously registered version, existing `cell:include` blocks referencing this item show a version update indicator.

---

## 12. Assets API (`folivm.assets`)

Available to all extensions without a specific permission.

Extensions may bundle static files in the `assets/` directory of the `.fvmext` package. These files are accessible through:

```typescript
folivm.assets.read(path: string): Promise<string>     // text files
folivm.assets.readBytes(path: string): Promise<Uint8Array>  // binary files
folivm.assets.url(path: string): string               // blob URL (panels only)
```

Paths are relative to the root of the package archive. Attempting to read a path outside the archive throws `AssetNotFoundError`.

`folivm.assets.url` returns a blob URL suitable for use as an `<img src>` or `<link href>` in a panel HTML file. The URL is valid only for the lifetime of the application session.

---

## 13. Error Types

All extension API errors are instances of `FolvimError` or a subclass:

| Class | When thrown |
|---|---|
| `PermissionDeniedError` | API call not covered by declared permissions |
| `NetworkPermissionDeniedError` | `fetch` call to an undeclared host |
| `AssetNotFoundError` | `folivm.assets.read` path not found in package |
| `BlockNotFoundError` | `folivm.document.getBlock` with unknown ID |
| `RegistrationError` | `folivm.cells.register` for an undeclared cell type |
| `ConflictError` | Two extensions attempt to register the same cell type |
| `ExportAbortedError` | Export pipeline was cancelled before the hook completed |

---

## 14. Extension Lifecycle

```
Install (.fvmext)
    │
    ▼
Permission approval dialog
    │ user approves
    ▼
Extension stored in OS app support dir
Extension listed as disabled
    │ user enables
    ▼
Deno Core isolate created
Entry point executed (registrations happen here)
folivm.on("application:ready") fired
    │
    ▼  (during session)
folivm.on("document:opened") fired per document
folivm.on("document:closed") fired per document
folivm.on("theme:changed") fired on theme changes
    │
    ▼  (application quit)
folivm.on("application:quit") fired
Isolate torn down
```

### 14.1 Enable / disable

Disabling an extension tears down its Deno Core isolate. All registrations (cell handlers, data sources, panels) are removed. Documents with cells rendered by the disabled extension show the built-in placeholder.

Re-enabling an extension is equivalent to a fresh load: the entry point re-runs and all registrations happen again.

### 14.2 Update

Updating an extension installs the new `.fvmext` file over the old one. The old isolate is torn down and the new isolate starts. No application restart is required.

---

## 15. Developer Tooling

### 15.1 Developer console

View → Developer Console (`Cmd+Shift+J`) opens a split panel showing:

- Extension log output (from `console.log`, `console.warn`, `console.error`)
- Structured API call trace (enabled in Settings → Extensions → Log API calls)
- Error stack traces on uncaught exceptions
- Cell render timing per block

Output is filterable by extension name and log level.

### 15.2 Live reload

During development, extensions can be loaded from a local directory instead of a `.fvmext` file:

1. Extensions panel → Install from folder → select the extension directory
2. Enable "Watch for changes" — folivm monitors the directory for file changes and reloads the extension automatically

Live reload re-runs the entry point. Existing cell renders are invalidated and re-requested.

### 15.3 Packaging

The `folivm` CLI (distributed separately as `folivm-cli`) packages an extension directory into a `.fvmext` file:

```
folivm ext pack ./my-extension --out my-extension-1.0.0.fvmext
```

Packaging runs the TypeScript compiler, validates the manifest schema, checks that all declared panel HTML files exist, and produces a signed ZIP archive. Signing is optional for local installs. A marketplace submission would require a publisher key (v1.0 has no marketplace).

---

## 16. v1.0 Validation Requirements

Per SCOPE.md section 5, the extension API is not v1.0 ready until at least one published extension is tested against it. The minimum bar:

### 16.1 Cell type handler validation

A working Mermaid diagram extension **or** Zotero citation extension must:

- [ ] Register a cell type handler on load without error
- [ ] Return a `CellRenderResult` within 2 seconds for a typical input
- [ ] Return a `CellExportResult` for PDF export
- [ ] Return a `CellExportResult` for DOCX export
- [ ] Gracefully return `{ type: "error" }` on invalid input (no crash, no hang)
- [ ] Pass through enable/disable/re-enable without requiring application restart
- [ ] Pass through document open/close lifecycle without memory leak (verified by heap snapshot)

### 16.2 Data source validation

A working CSV data source extension must:

- [ ] Register a data source handler on load
- [ ] Return a non-empty field list from `fields()`
- [ ] Resolve known fields to strings within 100ms
- [ ] Return `null` for unknown fields (no crash)
- [ ] Correctly resolve in both PDF and DOCX export paths

### 16.3 API contract stability

Before v1.0 ships, every API in this document is tested against both reference implementations. Any API function not covered by at least one reference implementation is either removed from this document or deferred to v1.1.

---

## 17. Versioning and Compatibility

The extension API is versioned by the `min_folivm_version` field in the extension manifest. folivm will refuse to load an extension that declares a `min_folivm_version` higher than the running application version.

Backwards compatibility guarantee (from v1.0 onward):
- API functions will not be removed within a major version (1.x)
- New optional parameters may be added to existing functions
- New API namespaces may be added
- Breaking changes require a major version bump (2.0)

API additions within 1.x are communicated through the `min_folivm_version` field. An extension that uses `folivm.cells.invalidate` (introduced in 1.1, hypothetically) must declare `"min_folivm_version": "1.1.0"` to prevent loading on older application versions.

---

*This document is the contract between folivm and its extension ecosystem. Changes to any API described here that affect existing extensions require a deprecation notice of at least one minor version.*
