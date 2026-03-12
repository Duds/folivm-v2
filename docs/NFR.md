# folivm — Non-Functional Requirements
**Version:** 0.1 (draft)
**Date:** 2026-03-12
**Status:** In review
**Depends on:** CONCEPT.md v0.3, FR.md v0.1

---

## Conventions

**Requirement IDs** follow the pattern `NFR-[CATEGORY]-[NNN]`.

**Priority:**
- `P0` — Must meet at launch. Non-negotiable.
- `P1` — Strong target. Degradation is acceptable only with documented justification.
- `P2` — Aspirational. Targeted for v1.1.

**Categories:**
| Code | Category |
|---|---|
| PERF | Performance |
| FIDELITY | Physical measurement and rendering fidelity |
| ACCESS | Accessibility |
| SEC | Security |
| PRIVACY | Privacy and data handling |
| RELI | Reliability and data integrity |
| PLATFORM | Platform and compatibility |
| USABILITY | Usability and learnability |
| MAINTAIN | Maintainability and extensibility |
| INTEROP | Interoperability and import/export |

---

## 1. Performance (PERF)

Performance is critical for a word processor. Typing latency and scroll jank are visceral — users feel them immediately and associate them with poor quality. Every performance budget below is a hard target, not a guideline.

### 1.1 Application startup

| ID | Priority | Requirement |
|---|---|---|
| NFR-PERF-001 | P0 | Cold start to interactive (last project open, editor ready) shall complete in under 3 seconds on the minimum supported hardware. |
| NFR-PERF-002 | P0 | Warm start (application already in memory, brought to foreground) shall complete in under 500ms. |
| NFR-PERF-003 | P1 | The welcome screen shall be interactive within 1 second of cold launch, before the last project has finished loading. |

### 1.2 Document operations

| ID | Priority | Requirement |
|---|---|---|
| NFR-PERF-010 | P0 | Opening a document of up to 50,000 words shall complete in under 500ms. |
| NFR-PERF-011 | P0 | Opening a document of up to 200,000 words shall complete in under 2 seconds. |
| NFR-PERF-012 | P0 | Switching between open tabs shall complete in under 100ms. |
| NFR-PERF-013 | P0 | Switching between Outline and Design modes shall complete in under 300ms. |
| NFR-PERF-014 | P1 | Mode switching shall not produce a visible flash or layout reflow. |

### 1.3 Editor responsiveness

| ID | Priority | Requirement |
|---|---|---|
| NFR-PERF-020 | P0 | Keystroke-to-screen latency shall not exceed 16ms (60fps frame budget) under normal editing conditions. |
| NFR-PERF-021 | P0 | Keystroke-to-screen latency shall not exceed 32ms in documents of up to 200,000 words. |
| NFR-PERF-022 | P0 | Undo and redo operations shall complete in under 50ms for all supported document sizes. |
| NFR-PERF-023 | P0 | Style application (changing the style of a block or inline selection) shall complete in under 32ms. |
| NFR-PERF-024 | P1 | Paste operations (including format stripping) shall complete in under 100ms for pasted content up to 10,000 words. |

### 1.4 Scroll and canvas

| ID | Priority | Requirement |
|---|---|---|
| NFR-PERF-030 | P0 | Scrolling in both Outline and Design modes shall maintain 60fps on the minimum supported hardware. |
| NFR-PERF-031 | P0 | Design mode canvas scroll shall not stutter when multiple pages are visible simultaneously. |
| NFR-PERF-032 | P1 | Design mode shall use virtualised page rendering — pages outside the visible viewport shall not be fully rendered in the DOM. |
| NFR-PERF-033 | P1 | Zoom level changes in Design mode shall re-render the canvas within 100ms. |

### 1.5 Search

| ID | Priority | Requirement |
|---|---|---|
| NFR-PERF-040 | P0 | Project-wide search shall return initial results within 500ms for projects containing up to 100 `.fvm` files. |
| NFR-PERF-041 | P0 | Project-wide search shall complete within 2 seconds for projects containing up to 500 `.fvm` files. |
| NFR-PERF-042 | P0 | Search results shall be streamed — initial results shall appear before the full search completes, updating as remaining files are processed. |
| NFR-PERF-043 | P1 | Find in document shall highlight all matches within 50ms for documents of up to 200,000 words. |

### 1.6 Export

| ID | Priority | Requirement |
|---|---|---|
| NFR-PERF-050 | P0 | PDF export of a 50-page document shall complete in under 5 seconds. |
| NFR-PERF-051 | P0 | DOCX export of a 50-page document shall complete in under 5 seconds. |
| NFR-PERF-052 | P1 | Export shall not block the editor UI. A progress indicator shall be shown and the user shall be able to continue editing during export. |
| NFR-PERF-053 | P1 | PDF export of a 200-page document shall complete in under 20 seconds. |

### 1.7 Theme and styling

| ID | Priority | Requirement |
|---|---|---|
| NFR-PERF-060 | P0 | Switching the active theme at project level shall re-render all open documents within 500ms. |
| NFR-PERF-061 | P0 | Editing a theme variable value shall update the live preview in the theme editor within 32ms. |
| NFR-PERF-062 | P1 | Switching a collection mode (e.g. screen → print, client-a → client-b) shall re-render within 300ms. |

### 1.8 Memory

| ID | Priority | Requirement |
|---|---|---|
| NFR-PERF-070 | P0 | The application shall not exceed 512MB RAM with a single project open and up to 5 documents open in tabs. |
| NFR-PERF-071 | P1 | The application shall not exceed 1GB RAM with up to 20 documents open in tabs. |
| NFR-PERF-072 | P1 | Memory usage shall not grow unboundedly during a long editing session. A maximum session memory growth of 20% over baseline is acceptable. |

---

## 2. Physical Measurement and Rendering Fidelity (FIDELITY)

This category addresses the v1 failure directly. These requirements are non-negotiable — they define the correctness of the product's core WYSIWYG promise.

| ID | Priority | Requirement |
|---|---|---|
| NFR-FIDELITY-001 | P0 | All document measurements (margins, page dimensions, tab stops, indents) shall be stored exclusively in physical units: points (pt) or millimetres (mm). Pixel values shall never be stored in the document or theme format. |
| NFR-FIDELITY-002 | P0 | A single scale factor shall be computed as `scale = container_width_px / page_width_pt` and applied uniformly to all elements in the Design mode canvas: page rectangle, margin guides, ruler, tab stop indicators, and all content positioning. No element shall derive an independent scale. |
| NFR-FIDELITY-003 | P0 | The application shall query the actual screen DPI from the OS via the Tauri API at startup and whenever the window moves to a different display. The scale factor computation shall incorporate the actual DPI. |
| NFR-FIDELITY-004 | P0 | The visual deviation between a dimension as rendered on screen at 100% zoom and the same dimension in a PDF export shall not exceed 0.5mm. |
| NFR-FIDELITY-005 | P0 | The ruler and the page canvas shall always be pixel-aligned — the ruler's margin indicator position shall correspond exactly to the page canvas's margin guide position, at any zoom level. |
| NFR-FIDELITY-006 | P0 | Zoom level changes shall update the scale factor and recompute all element positions. No element shall retain a cached pixel position from a previous zoom level. |
| NFR-FIDELITY-007 | P0 | Changing a margin value in the right rail shall update the ruler, margin guides, and content reflow simultaneously in a single render pass. They shall never be out of sync by even one frame. |
| NFR-FIDELITY-008 | P1 | Tab stop positions set via the ruler shall correspond to the same positions in PDF and DOCX export, within 0.5mm. |
| NFR-FIDELITY-009 | P1 | The application shall provide a calibration utility allowing the user to verify on-screen measurements against a physical ruler, and apply a per-display correction factor if needed. |

---

## 3. Accessibility (ACCESS)

### 3.0 The structural accessibility guarantee

folivm's semantic-first format means that a correctly authored document is WCAG-accessible by construction. This is not a coincidence — it is a direct consequence of the format's design constraints:

| WCAG criterion | Word (ad-hoc) | folivm (semantic) |
|---|---|---|
| 1.1.1 Non-text content | Alt text optional, often omitted | `alt` required on `cell:image` and `cell:diagram` by format |
| 1.3.1 Info and relationships | Fake headings via big bold text are invisible to AT | Headings are H1/H2/H3 by style — real semantic structure |
| 1.3.2 Meaningful sequence | Table layout tricks break reading order | Document order IS reading order — no tables, floats or z-index |
| 1.4.1 Use of colour | Colour-only indicators common | Style name is the semantic signal; icons and colour are decorative |
| 1.4.3 Contrast | Manual, unchecked | Theme editor validates WCAG AA contrast ratios per token pair |
| 2.4.6 Headings and labels | Often missing or misleading | Every heading is a real heading; every callout has a semantic name |
| 3.1.1 Language of page | Often missing | `lang` declared in frontmatter, propagates to PDF and DOCX |
| 4.1.2 Name, role, value | Icons in tables carry no role | Callout style name IS the role; icons are presentation-only |

This means that for organisations with WCAG compliance obligations — government, legal, academic publishers, corporate communications — folivm produces compliant documents by default. The remediation step that Word requires is eliminated.

### 3.1 Application chrome accessibility

| ID | Priority | Requirement |
|---|---|---|
| NFR-ACCESS-001 | P0 | The application chrome (activity bar, sidebar panels, right rail, status bar, dialogs, menus) shall conform to WCAG 2.1 Level AA. |
| NFR-ACCESS-002 | P0 | All interactive elements shall be reachable and operable via keyboard alone without requiring a mouse or trackpad. |
| NFR-ACCESS-003 | P0 | All interactive elements shall have accessible labels readable by screen readers. Custom SVG icons shall include `aria-label` attributes. Icon-only buttons shall include visually hidden text labels. |
| NFR-ACCESS-004 | P0 | Focus indicators shall be visible at all times on all focusable elements. The focus ring shall not be suppressed by any application style. |
| NFR-ACCESS-005 | P0 | The application shall support the OS-level high contrast mode on macOS and Windows without breaking layout or hiding content. |
| NFR-ACCESS-006 | P0 | The application UI text shall respect the OS-level font size accessibility preference. |
| NFR-ACCESS-007 | P1 | The application shall be fully operable with VoiceOver (macOS) and NVDA/JAWS (Windows) for all primary workflows: opening a project, creating a document, writing content, applying a style, saving a version, exporting to PDF. |
| NFR-ACCESS-008 | P1 | Colour shall not be the sole means of conveying information in the application UI. All status indicators (git status badges, modified document indicator, warning flags) shall include a text, icon, or pattern component alongside colour. |
| NFR-ACCESS-009 | P1 | The minimum interactive touch/click target size shall be 44×44pt (Apple HIG) on all platforms. |
| NFR-ACCESS-010 | P1 | All animations and transitions shall respect the OS-level Reduce Motion preference. When Reduce Motion is enabled, all transitions shall be instantaneous. |
| NFR-ACCESS-011 | P2 | The application shall provide a high-contrast editor theme variant for users with low vision, increasing border contrast and inverting the canvas. |

### 3.2 Document output accessibility

| ID | Priority | Requirement |
|---|---|---|
| NFR-ACCESS-020 | P0 | PDF export shall produce a tagged PDF (PDF/UA-1, ISO 14289-1) when the `output/accessible-pdf` theme boolean variable is true. This shall be the default for all built-in themes. |
| NFR-ACCESS-021 | P0 | Tagged PDF output shall map folivm semantic structures to PDF tags: heading styles to `<H1>`–`<H6>`, body styles to `<P>`, lists to `<L>/<LI>`, tables to `<Table>`, `cell:image` alt text to `<Alt>`, callout styles to `<Note>`. |
| NFR-ACCESS-022 | P0 | DOCX export shall use DOCX built-in heading styles (Heading 1–6) for folivm heading styles, ensuring correct navigation in Word's document map and in assistive technology. |
| NFR-ACCESS-023 | P0 | The reading order of exported PDF and DOCX documents shall match the document order in the `.fvm` source. No reordering shall occur during export. |
| NFR-ACCESS-024 | P1 | The theme editor shall display WCAG AA contrast ratio for every colour token pair used as text-on-background in any named style. Pairs that fail WCAG AA (4.5:1 normal text, 3:1 large text) shall display a labelled warning. |
| NFR-ACCESS-025 | P1 | The application shall warn (but not block) when a `cell:image` or `cell:diagram` block is inserted or saved without an `alt` attribute. |
| NFR-ACCESS-026 | P1 | The document language declared in frontmatter (`lang`) shall propagate to the PDF `Lang` dictionary entry and the DOCX `w:lang` element, enabling correct screen reader pronunciation. |
| NFR-ACCESS-027 | P2 | The application shall provide an accessibility report on demand (pre-export check) listing: missing alt text, skipped heading levels, colour contrast failures, and missing document language. |

---

## 4. Security (SEC)

### 4.1 Extension sandboxing

| ID | Priority | Requirement |
|---|---|---|
| NFR-SEC-001 | P0 | Extensions shall execute in an isolated sandbox. They shall not have arbitrary access to the file system, system APIs, or other extensions. |
| NFR-SEC-002 | P0 | Extensions shall declare all required permissions at install time. The application shall present the declared permissions to the user before installation is confirmed. |
| NFR-SEC-003 | P0 | Extensions that declare no network connectivity shall be prevented at the runtime level from making outbound network calls. This shall be enforced by the sandbox, not by trust in the extension code. |
| NFR-SEC-004 | P0 | Extensions shall only be able to read and write files within the current project directory and the extension's own data directory. Cross-project file access shall not be permitted. |
| NFR-SEC-005 | P1 | Extension permissions shall be reviewable and revocable from the Extensions panel after installation. |
| NFR-SEC-006 | P1 | Extension packages shall be verified against a checksum before installation. Tampered packages shall be rejected. |

### 4.2 File system and data

| ID | Priority | Requirement |
|---|---|---|
| NFR-SEC-010 | P0 | The application shall only write to the current project directory, the OS-standard application data directory, and explicitly user-selected export locations. It shall not write to arbitrary file system locations. |
| NFR-SEC-011 | P0 | The application shall validate all `.fvm` and `.fvm-theme` files before parsing. Malformed YAML or unexpected cell types shall be handled gracefully without executing arbitrary content. |
| NFR-SEC-012 | P0 | SVG content inlined in `cell:image` blocks shall be sanitised before rendering to prevent XSS via SVG script execution. |
| NFR-SEC-013 | P1 | The application shall not auto-execute any content from imported documents, pasted content, or extension-provided data without an explicit user action. |

### 4.3 Network

| ID | Priority | Requirement |
|---|---|---|
| NFR-SEC-020 | P0 | The core application shall make zero outbound network calls. All network activity is the responsibility of explicitly installed and user-enabled extensions. |
| NFR-SEC-021 | P0 | Any network activity by an extension shall be declarable and auditable. A network activity log shall be available in the Extensions panel for extensions with network permissions. |
| NFR-SEC-022 | P1 | The extension marketplace connection (for browsing available extensions) shall use HTTPS exclusively. Extension packages shall be downloaded over HTTPS with certificate validation. |

---

## 5. Privacy and Data Handling (PRIVACY)

| ID | Priority | Requirement |
|---|---|---|
| NFR-PRIVACY-001 | P0 | The application shall collect no telemetry, usage data, or analytics by default. |
| NFR-PRIVACY-002 | P0 | No document content, project file, theme file, or user file shall be transmitted to any remote server by the core application at any time. |
| NFR-PRIVACY-003 | P0 | Optional anonymous usage analytics shall be opt-in only, presented during first-run setup, with a clear description of what is collected and a persistent opt-out in settings. |
| NFR-PRIVACY-004 | P0 | Crash reports shall not be transmitted without explicit user consent at the time of the crash. The crash report shall display its contents to the user before submission. |
| NFR-PRIVACY-005 | P0 | All application data (project files, settings, extension data) shall be stored in user-controlled, user-accessible locations on the local file system. No proprietary data store shall be used. |
| NFR-PRIVACY-006 | P1 | Extensions that transmit data remotely shall disclose this in their declared permissions. The user shall be able to see exactly which extensions have network access and revoke it. |
| NFR-PRIVACY-007 | P1 | The application shall comply with applicable data protection regulations (GDPR, Australian Privacy Act) to the extent applicable to a local-first desktop application with optional analytics. |

---

## 6. Reliability and Data Integrity (RELI)

### 6.1 Auto-save and data loss prevention

| ID | Priority | Requirement |
|---|---|---|
| NFR-RELI-001 | P0 | The application shall auto-save all open documents to disk at a configurable interval (default: 30 seconds). |
| NFR-RELI-002 | P0 | Auto-save shall trigger on window blur (application loses focus) and before any destructive operation. |
| NFR-RELI-003 | P0 | In the event of an application crash, the user shall lose no more than 30 seconds of work, assuming the default auto-save interval. |
| NFR-RELI-004 | P0 | A crash recovery mechanism shall detect uncommitted auto-saved changes on next launch and offer to restore them. |
| NFR-RELI-005 | P0 | Auto-save shall write atomically — using a write-to-temp-then-rename pattern — to prevent partial writes from corrupting documents. |

### 6.2 Version history integrity

| ID | Priority | Requirement |
|---|---|---|
| NFR-RELI-010 | P0 | The git repository backing the project's version history shall not be corrupted by any application operation. All git operations shall be validated before and after execution. |
| NFR-RELI-011 | P0 | A failed git operation (e.g. merge conflict, locked index) shall surface a clear, actionable error message in document language. The application shall not silently fail or leave the repository in a broken state. |
| NFR-RELI-012 | P1 | The application shall detect a corrupted git repository at project open and offer a recovery path (reinitialise git while preserving file content). |

### 6.3 Document integrity

| ID | Priority | Requirement |
|---|---|---|
| NFR-RELI-020 | P0 | Opening a malformed `.fvm` file (invalid YAML frontmatter, unclosed cell blocks) shall not crash the application. The application shall open the file in a degraded state, highlight the malformed sections, and allow the user to correct them. |
| NFR-RELI-021 | P0 | A document with an unrecognised cell type shall open and render correctly for all recognised content. Only the unrecognised cells shall be affected, rendered as placeholders. |
| NFR-RELI-022 | P1 | The application shall validate the document format on every save and warn (but not block) if the saved file contains structural issues. |

### 6.4 Undo / redo

| ID | Priority | Requirement |
|---|---|---|
| NFR-RELI-030 | P0 | The application shall support unlimited undo and redo within a session. |
| NFR-RELI-031 | P0 | Undo history shall persist across auto-saves within a session. Auto-save shall not clear the undo stack. |
| NFR-RELI-032 | P1 | Undo history shall be per-document, not global. Undoing in one tab shall not affect the undo stack of another tab. |

---

## 7. Platform and Compatibility (PLATFORM)

### 7.1 Supported platforms

| ID | Priority | Requirement |
|---|---|---|
| NFR-PLATFORM-001 | P0 | The application shall run on macOS 13 (Ventura) and later. |
| NFR-PLATFORM-002 | P0 | The application shall run on Windows 10 (22H2) and Windows 11. |
| NFR-PLATFORM-003 | P1 | The application shall run on Ubuntu 22.04 LTS and later. Arch and Fedora are secondary targets. |
| NFR-PLATFORM-004 | P0 | The application shall support Apple Silicon (ARM64) natively. It shall not run under Rosetta emulation on Apple Silicon. |
| NFR-PLATFORM-005 | P0 | The application shall support x86-64 on macOS, Windows, and Linux. |
| NFR-PLATFORM-006 | P1 | The application shall be distributed as: a `.dmg` installer on macOS, an `.msi` / `.exe` installer on Windows, and an AppImage or `.deb` on Linux. |

### 7.2 Minimum hardware

| ID | Priority | Requirement |
|---|---|---|
| NFR-PLATFORM-010 | P0 | The application shall meet all performance requirements (Section 1) on hardware with: 8GB RAM, a dual-core CPU at 2GHz or equivalent, and a display at 1920×1080 resolution. |
| NFR-PLATFORM-011 | P1 | The application shall be usable (not necessarily at full performance targets) on hardware with 4GB RAM. |

### 7.3 Display

| ID | Priority | Requirement |
|---|---|---|
| NFR-PLATFORM-020 | P0 | The application shall render correctly at 1x, 1.5x, and 2x display scaling factors (standard, HiDPI, Retina). |
| NFR-PLATFORM-021 | P0 | The application shall respond correctly when the window is moved between displays with different DPI settings, recomputing the scale factor without requiring a restart. |
| NFR-PLATFORM-022 | P1 | The minimum supported display resolution shall be 1280×800. The layout shall not break or overlap at this resolution. |

---

## 8. Usability and Learnability (USABILITY)

| ID | Priority | Requirement |
|---|---|---|
| NFR-USABILITY-001 | P0 | A new user with no prior folivm experience shall be able to create a project, write a document, apply styles, and export to PDF within 10 minutes, using only the in-app onboarding. |
| NFR-USABILITY-002 | P0 | All error messages shall be written in plain language describing what went wrong and what the user can do next. Technical identifiers (git error codes, Rust panics) shall not be shown to the user. |
| NFR-USABILITY-003 | P0 | Every destructive action (delete file, delete version, replace-all, remove style from theme) shall require explicit confirmation and shall be undoable where technically possible. |
| NFR-USABILITY-004 | P0 | The application shall provide an interactive first-run onboarding that introduces: Outline mode → writing → style application → Design mode → export. The onboarding shall be skippable and re-accessible from the Help menu. |
| NFR-USABILITY-005 | P1 | The application shall surface contextual help inline — hovering over UI elements shall show a tooltip describing the element's purpose. |
| NFR-USABILITY-006 | P1 | The style selector shall make applying a style faster than typing the style name — it shall support fuzzy search so that typing "cl" surfaces "ClauseHeading" and "ClauseBody" immediately. |
| NFR-USABILITY-007 | P1 | The application shall nudge users toward Outline mode through a better experience — section word count targets, focus mode, keyboard-first navigation — without preventing Design mode use. |
| NFR-USABILITY-008 | P1 | First-time use of the content library, versioning, and extension installation shall each be accompanied by a brief contextual explanation of the feature and its purpose. |
| NFR-USABILITY-009 | P2 | The application shall provide an optional "habit coach" mode that quietly logs which mode the user writes in and surfaces a weekly summary encouraging Outline mode use. This shall be opt-in. |

---

## 9. Maintainability and Extensibility (MAINTAIN)

| ID | Priority | Requirement |
|---|---|---|
| NFR-MAINTAIN-001 | P0 | The extension API shall be versioned using semantic versioning. Breaking changes to the extension API shall require a major version bump and a documented migration guide. |
| NFR-MAINTAIN-002 | P0 | The `.fvm` format specification shall be independently versioned and documented. The format version shall be stored in the document frontmatter (`fvm-version`). The application shall support reading all prior format versions. |
| NFR-MAINTAIN-003 | P0 | The `.fvm-theme` format specification shall be independently versioned. The application shall support reading all prior theme format versions. |
| NFR-MAINTAIN-004 | P0 | The Rust backend and React/TypeScript frontend shall maintain strict type safety. TypeScript `strict` mode shall be enabled. Unsafe Rust blocks shall be documented with a safety justification comment. |
| NFR-MAINTAIN-005 | P1 | Unit test coverage shall be maintained at a minimum of 80% for the core document model (parser, renderer, theme resolution, scale factor computation). |
| NFR-MAINTAIN-006 | P1 | The pt→px scale factor pipeline shall have dedicated unit tests verifying correct computation across: standard DPI, HiDPI, and multi-display scenarios. |
| NFR-MAINTAIN-007 | P1 | The extension API surface shall be covered by integration tests. Any change that breaks an existing integration test shall be treated as a breaking change. |
| NFR-MAINTAIN-008 | P1 | The application shall emit structured logs at debug level for all git operations, scale factor computations, and extension API calls. These logs shall be accessible from a developer console (Help → Developer Logs). |
| NFR-MAINTAIN-009 | P2 | The application architecture shall enforce a hard boundary between the document model layer (Rust), the rendering layer (React), and the extension host. Cross-layer dependencies shall be explicit and minimal. |

---

## 10. Interoperability (INTEROP)

### 10.1 Import

| ID | Priority | Requirement |
|---|---|---|
| NFR-INTEROP-001 | P1 | The application shall support best-effort import of `.docx` files, converting them to `.fvm` format. |
| NFR-INTEROP-002 | P1 | DOCX import shall map Word paragraph styles to folivm named styles where a matching name exists in the active theme. Unmapped styles shall be flagged for user resolution. |
| NFR-INTEROP-003 | P1 | DOCX import shall convert inline formatting (bold, italic) to the equivalent folivm built-in inline styles (Strong, Emphasis). |
| NFR-INTEROP-004 | P1 | DOCX import shall extract images and place them in the project `assets/` directory, replacing inline binary content with `cell:image` references. |
| NFR-INTEROP-005 | P1 | DOCX import shall preserve document structure: headings (mapped by level to H1/H2/H3), lists, tables, and blockquotes. |
| NFR-INTEROP-006 | P2 | The application shall support import of plain Markdown (`.md`) files, treating them as `.fvm` documents with default styles applied. |

### 10.2 Export fidelity

| ID | Priority | Requirement |
|---|---|---|
| NFR-INTEROP-010 | P0 | PDF export shall produce output visually identical to the Design mode canvas within the 0.5mm physical fidelity tolerance defined in NFR-FIDELITY-004. |
| NFR-INTEROP-011 | P0 | DOCX export shall produce a document that opens without errors in Microsoft Word 2019+, Word for Microsoft 365, and LibreOffice Writer 7+. |
| NFR-INTEROP-012 | P1 | DOCX export shall produce a document in which the exported styles are editable in Word's Styles panel — the DOCX shall not be a flattened representation. |
| NFR-INTEROP-013 | P1 | PDF output shall be PDF/A-2b compliant for documents intended for archival (configurable export option), meeting the requirements of legal and government document archiving standards. |
| NFR-INTEROP-014 | P1 | The `.fvm` format shall be processable by standard text processing tools (grep, sed, awk, Python, Node.js) without requiring folivm to be installed. The format specification shall be sufficient to write a conforming parser. |

---

## 11. Localisation (L10N)

| ID | Priority | Requirement |
|---|---|---|
| NFR-L10N-001 | P1 | The application UI shall be fully internationalised. All user-facing strings shall be externalised and translatable. |
| NFR-L10N-002 | P1 | The application shall ship with English (en-AU, en-GB, en-US) at launch. |
| NFR-L10N-003 | P1 | The application shall support right-to-left (RTL) document layout for Arabic and Hebrew content. Page margins and text alignment shall mirror correctly in RTL mode. |
| NFR-L10N-004 | P2 | Additional language UI translations (French, German, Spanish, Japanese) shall be supported via community contribution to a public translation repository. |
| NFR-L10N-005 | P2 | Measurement unit display shall respect the user's locale — metric (mm) for most markets, imperial (inches) for the US market. The underlying format storage in pt shall be unaffected. |

---

## Appendix A — Performance test conditions

All performance requirements shall be verified under the following conditions unless otherwise specified:

| Parameter | Condition |
|---|---|
| Hardware | 8GB RAM, dual-core 2GHz CPU, SSD storage |
| Display | 1920×1080 at 1x scaling |
| OS | macOS 13 (primary), Windows 11 (secondary) |
| Document size (standard) | 10,000 words, 20 pages, A4, default theme |
| Document size (large) | 50,000 words, 100 pages, A4, default theme |
| Project size (standard) | 20 `.fvm` files, 5 assets |
| Project size (large) | 200 `.fvm` files, 50 assets |
| Extensions installed | None (core only) |
| Concurrent tabs open | 3 documents |

---

## Appendix B — Fidelity verification method

Physical measurement fidelity (NFR-FIDELITY-004) shall be verified by:

1. Creating a document with a known element at an exact dimension (e.g. a horizontal rule at exactly 100mm width)
2. Exporting to PDF
3. Measuring the element in the PDF using a PDF measurement tool
4. The measured value shall be 100mm ± 0.5mm

Screen fidelity shall be verified by:
1. Rendering the same element in Design mode at 100% zoom on a calibrated display
2. Measuring the element on screen using the on-screen ruler
3. The ruler reading shall match the defined dimension within ± 0.5mm

---

*Document history tracked in git. Next: High Level Design (HLD.md)*
