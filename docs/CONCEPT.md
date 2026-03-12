# folivm — Concept Document
**Version:** 0.4 (draft)
**Date:** 2026-03-12
**Status:** In review

---

## 1. The Problem

Professional document production is broken in four distinct ways.

### 1.1 The format problem
Word processors produce binary files. `.docx` is a zipped XML bundle that requires specialised parsers to read or write programmatically. This creates compounding failures:

- **AI cannot natively read or write it.** Every AI integration requires a conversion step (python-docx, pandoc, LibreOffice headless), each introducing fidelity loss.
- **Round-tripping is lossy.** Convert to AI-readable format → AI edits → convert back → formatting has shifted, styles have collapsed, tracked changes are gone.
- **Version control is unusable.** Git diffs of binary files are meaningless. Branching, merging, and change history are impractical.
- **Project-wide search is impossible.** Find and replace across a folder of `.docx` files requires opening every document individually. There is no way to search across a project — a matter, an engagement, a manual — as a whole. This is a basic professional need that the binary format structurally prevents.
- **Collaboration is file-based.** Documents travel by email as attachments. Review cycles are manual. There is no canonical source of truth.

### 1.2 The formatting problem
Every major word processor exposes raw formatting controls as the primary interface. Bold. Font size. Colour picker. These controls are seductive and destructive. Users apply character-level formatting ad-hoc, document by document, paragraph by paragraph. The result:

- Documents that look different from each other despite representing the same brand
- Rebrand exercises that require reformatting every document manually
- Documents with no semantic structure — an AI sees a blob of styled text, not a heading, a clause, a definition
- Export fidelity that depends on manual effort rather than system design

Word has had a Styles panel since 1989. Nobody uses it because bold is always one click closer.

### 1.3 The workflow problem
Current tools force a false choice: structured tools (outliners, note apps) that don't produce publishable output, or layout tools (Word, Pages) that punish structured thinking. Writers are forced to jump between tools — outlining in one app, writing in another, laying out in a third — losing structure and context at every handoff.

AI has begun to enter this space but as a bolt-on: a sidebar, a plugin, a copy-paste workflow. The document format itself remains unchanged and AI-hostile.

### 1.4 The vertical gap problem
Every major professional vertical — law, academia, technical writing, bid management, consulting — compensates for Word's inadequacies by layering third-party add-ins on top of it. Citation managers, legal drafting tools, proposal automation, data integration, XML authoring. The seams between these add-ins show. Data does not flow between them. The binary format breaks every attempt at systematic integration. The ecosystem is fragile because the foundation is wrong.

---

## 2. The Insight

The problem is the file format.

If the source document were ASCII-first — human-readable, machine-readable, versionable line by line, structurally explicit — then:

- AI can read, write, and reason about it natively, without conversion
- Git can diff, branch, and merge it meaningfully
- Semantic structure is preserved and queryable
- Export to PDF, DOCX, or any format is deterministic and lossless
- Round-tripping is safe — what AI writes is what the editor renders
- Project-wide find and replace works as a simple text operation across all files — no parsing, no conversion, no per-file opening required

And if the editor enforced semantic styling — named styles backed by a design token system, with no ad-hoc formatting escape hatch — then:

- Every document produced by a firm or team is structurally consistent
- Rebranding is a theme change, not a reformatting exercise
- Style is separated from content — structure can be reasoned about independently of presentation
- **Documents are WCAG-accessible by construction.** Semantic headings cannot be faked with big bold text. Image alt text is required by the format. Icons are decorative properties of named styles — meaning is never conveyed by colour or icon alone. Paragraph-level callouts (`Tip`, `Warning`, `Remember`) carry their semantic meaning in their style name, not in a visual treatment. Accessibility is structural, not a post-hoc audit.

And if the extension architecture were principled — with a rich, documented API that vertical tools can build against — then:

- Zotero, LegalMind, ClearBrief, DocuSign, and DITA exporters can integrate deeply rather than superficially
- The ecosystem builds on a sound foundation rather than compensating for a broken one
- Each vertical extension makes folivm more valuable for that vertical without bloating the core

This is the foundation of folivm.

---

## 3. The Solution

**folivm** is a professional word processor built on an ASCII-first document format, a semantic styling system, a content library, and a principled extension architecture.

It is a word processor first. Not an AI editor. Not a note-taking app. Not a publishing platform. A word processor — the tool professionals use to produce documents — rebuilt from first principles for an era where documents must be readable by both humans and machines.

### 3.1 The folivm format (`.fvm`)

An open, ASCII-first document format extending Markdown and YAML with typed cells and layout primitives.

```
---
title: Agreement for Services
client: "{crm.client.name}"
page-size: A4
orientation: portrait
margins: {top: 25mm, bottom: 25mm, left: 30mm, right: 20mm}
theme: acme-legal
header: {left: "{title}", right: "Page {page} of {pages}"}
footer: {left: "{client} — {matter.reference}", right: "Confidential"}
---

# Agreement for Services

This agreement is entered into between {crm.client.name} of
{crm.client.address} ("the Client") and...

:::section
## 1. Scope of Works

The Contractor agrees to provide the following services:
:::

:::cell:data
source: crm
fields: [matter.scope, matter.deliverables]
:::

:::cell:ai
provider: legalcounsel-ext
prompt: Draft the indemnity clause for {matter.type} jurisdiction {matter.jurisdiction}
style: ClauseBody
:::

:::cell:include
ref: library.standard-signature-block
version: "3.1"
:::
```

Every measurement is in physical units (mm or pt). Every style reference is a named semantic style. Every dynamic field is an explicit typed cell. The file is readable in any text editor. Git can diff it line by line. An LLM can read and write it without conversion.

**Native cell types registered by the format:**

| Cell type | Purpose |
|---|---|
| `cell:data` | Deterministic data from an external source (CSV, CRM) |
| `cell:ai` | Generative content from an AI extension |
| `cell:math` | LaTeX/AsciiMath equation, rendered as typeset formula |
| `cell:diagram` | Mermaid or draw.io syntax, rendered as diagram |
| `cell:citation` | Structured citation reference, renders inline + bibliography |
| `cell:signature` | Signature field, handled by signature extension |
| `cell:include` | Content library reference with version pin |

Extensions may register additional cell types through the extension API. Unknown cell types render as labelled placeholders — unrecognised cells never cause errors.

### 3.2 The theme system (`.fvm-theme`)

A Figma-inspired design token system built in three layers, with collections, modes, aliases, and named styles.

**Three token layers:**

- **Primitives** — raw foundational values. Never applied directly to content. The single source of truth for every value in the system.
- **Semantic tokens** — named aliases that reference primitives and communicate purpose and context. Applied to styles.
- **Named styles** — composite bundles of semantic tokens that define a complete text or paragraph style.

**Collections** group variables by purpose. Each collection has its own independent set of modes:

| Collection | Modes | Purpose |
|---|---|---|
| Primitives | `default` | Raw values — colours, sizes, fonts, booleans |
| Semantic | `screen`, `print` | Purpose-mapped references; values shift between screen and print output |
| Brand | `client-a`, `client-b` | A single theme file serves multiple client brands |
| Spacing | `normal`, `compact` | Wide margins for reading; tight for dense technical documents |

**Aliases** allow variables to reference other variables. A change to a primitive cascades automatically through every semantic token and named style that references it — rebrand by editing one value.

**Modes** store alternate values within the same variable. A colour token can have a different value for screen rendering versus print output, or for one client brand versus another. Switching mode at the project level repoints all aliases simultaneously.

**Variable types:**

| Type | Document uses |
|---|---|
| Color | Text colour, background, border, highlight |
| Number | Font size (pt), line height, spacing, margin, border width, opacity |
| String | Font family, font weight |
| Boolean | Widow control, hyphenation, ligatures, facing pages, section numbering, embed fonts, print marks |

**Scoping** restricts which variables appear in which UI contexts. Spacing variables do not appear in font-size selectors. Colour variables scoped to `text` do not appear in background colour contexts. As theme complexity grows, scoping keeps the interface manageable.

**Named styles** bundle semantic tokens into complete block or inline styles. Styles can extend other styles, reducing duplication:

```yaml
Definition:
  scope: block
  extends: ClauseBody
  font-style: italic
  indent-left: "{primitives.spacing-md}"
```

A corporate theme is a single `.fvm-theme` file. Apply it to a project and every document adopts it instantly. Rebrand by changing one primitive — everything cascades.

### 3.3 Semantic styling — the non-negotiable

folivm does not expose a font picker, a colour picker, or a raw bold button as primary controls. The only way to style content is to apply a named style.

This is not a restriction that can be worked around. It is a design decision enforced at the editor level.

The editor makes semantic styling fast and visible:
- Style selector is always prominent in both modes
- Keyboard shortcut for every named style
- Style badge on every block in outline mode
- Paste from external sources strips raw formatting and prompts for style assignment
- Inline styles (Emphasis, Strong, Code) are semantic names, not raw attributes

Users who want a different visual result change the style definition — they do not override individual characters.

**Accessibility is a structural consequence, not a feature.**

In Word, producing an accessible document requires running an accessibility checker after the fact and manually fixing issues: adding alt text to images, replacing fake headings with real ones, fixing colour-only indicators, correcting reading order broken by table layout tricks. It is remedial work performed on a document that was authored without accessibility in mind.

In folivm, the semantic-first format makes accessible authoring the only available path:

- Headings are H1/H2/H3 by style — a screen reader navigates real heading structure, not guessed-at big text
- `cell:image` requires `alt` text — the format enforces it, the editor prompts for it
- Paragraph-level callouts (`Tip`, `Warning`, `Remember`) carry semantic meaning in their style name — a screen reader announces "Warning: ..." not "yellow box with triangle icon"
- Icons are decorative style properties — meaning is never conveyed by icon or colour alone; the style name is always the primary semantic signal
- Inline styles (`DefinedTerm`, `CrossReference`) name their purpose — assistive technology can expose this context
- Reading order follows document order — no table layout tricks, no floating elements, no z-index hacks
- Document language is declared in frontmatter (`lang: en-AU`) — screen readers use the correct pronunciation rules
- Theme colour contrast ratios are auditable — the theme editor can validate WCAG AA contrast for every colour token pair

A folivm document exported to PDF/UA-compliant PDF is accessible by default. No remediation step required. For law firms, government agencies, academic publishers, and corporate communications teams with accessibility obligations, this is not a nice-to-have — it is a compliance requirement that folivm meets structurally.

### 3.4 Two modes, one document

**Outline mode** (default on open)
The content-first environment. Structure is visible. Pages are invisible. Every block shows its applied style, word count, and cell type indicator. Sections fold and unfold. Keyboard-driven. Fast. Focus mode narrows the view to a single section.

The environment where thinking and drafting happen.

**Design mode**
The presentation environment. Full WYSIWYG with physically accurate page layout, ruler, margin guides, tab stops, headers and footers, and pagination. Content is fully editable. Style selector remains the only formatting control.

The environment where layout is reviewed and output is verified.

Switching between modes is instant and lossless. Both modes enforce semantic styling. Neither mode is read-only. Users who prefer to write in Design mode are not blocked — they are nudged toward Outline mode through a better Outline experience, not through restriction.

### 3.5 Content Library — core feature

Professional document production depends on reusable content. Every vertical has this need:

| Vertical | Content reuse need |
|---|---|
| Legal | Standard clause library, approved boilerplate |
| Bid/Proposal | Approved content bank, standard sections |
| Technical writing | Notice blocks, API table templates, standard disclaimers |
| Academic | Abstract templates, methodology boilerplate |
| Consulting | Section templates, standard recommendations |

folivm's Content Library is a core feature — not an extension. It provides a managed library of reusable `.fvm` fragments (sections, blocks, clause sets, table structures) stored at project or global level.

Library items are referenced in documents with a version pin:

```
:::cell:include
ref: library.indemnity-standard
version: "2.3"
:::
```

The version pin means a document always knows exactly which version of a library item it used. Updating the library does not silently change existing documents. Accepting a library update is an explicit action, tracked in version history.

Library items are themselves `.fvm` fragments — ASCII, diffable, versionable. A firm's clause library lives in git alongside their documents.

### 3.6 The project workspace — IDE metaphor applied to documents

Professional document work is project work, not single-file work. Word treats every document as an island. folivm treats documents as members of a project — a folder of related files that share a theme, a version history, a content library, and a workspace.

The IDE metaphor — directory explorer, tabbed open documents, project-wide search — is the correct mental model for this work. Developers solved the "working on a project" problem decades ago. Document professionals have the same problem and have never had the right tool.

**The project as the unit of work:**

| Profession | Their project |
|---|---|
| Legal | A matter — contract, exhibits, precedents, correspondence |
| Consulting | An engagement — discovery, research, multiple deliverables |
| Academic | A paper — main document, appendices, data, supplementary material |
| Technical writing | A manual — dozens of topic files, shared notices, diagrams |
| Bid management | A tender — cover letter, technical response, pricing, case studies |

**Directory explorer**
The explorer makes the whole project visible at all times. It enables navigation between related documents without losing context, reveals what exists before creating something new, and makes project-level operations — versioning, theming, search — natural rather than exceptional. The project folder is the git repository. Files are versioned together because they belong together.

**Tabbed open documents**
Professionals work across multiple documents simultaneously. A lawyer drafts a contract while referencing a precedent in another tab. A consultant writes an executive summary while referencing the research document. Tabs keep this contained without window management overhead. Each tab shows unsaved change state. Switching context is one keystroke.

**Project-wide find and replace**
This is where ASCII-first format delivers an immediate, concrete benefit that has nothing to do with AI.

Finding and replacing across a folder of `.docx` files requires opening every document individually — the binary format cannot be searched as plain text. There is no practical way to perform a coordinated terminology change, client name update, or regulatory reference correction across a legal matter or technical manual as a whole.

Because `.fvm` files are plain text, project-wide find and replace is a straightforward text operation. No parsing. No conversion. No per-file opening. Results are grouped by file with line context. Regex, case sensitivity, and whole-word matching work exactly as in an IDE.

| Scenario | Word | folivm |
|---|---|---|
| Client name change across a 12-document matter | Open each file, Find & Replace, save, repeat | One operation across the project |
| Regulatory reference updated across a compliance manual | Manual, error-prone, often missed | Search reveals every instance, replace is coordinated |
| Defined term used inconsistently across a proposal | Invisible problem | Search surfaces it immediately |
| Terminology audit before submission | Impossible across files | Project-wide search with full context |

**When project context compounds**
The IDE workspace is not just a convenience — it changes what is possible:

- Version control commits capture all related documents together in a coherent state
- Theme application is project-wide — one change, all documents update
- Content library items are available to every document in the project
- Find and replace operates across the whole project as a single operation
- Extensions with project awareness (AI extensions, when installed) can reason about the full project — what has been written, what styles are in use, what documents exist

### 3.8 Versioning — document language, git backbone

folivm's version history is backed by git, but users never see git. The interface speaks document language:

| What the user sees | What happens underneath |
|---|---|
| Save version | git commit |
| Create draft | git branch |
| See changes | git diff rendered as track changes |
| Accept / reject changes | git merge / revert hunk |
| Who wrote this | git blame |
| Share for comment | export patch or share branch |

Inline comments and track-changes markup sit on top of git's line-level history. Non-technical users manage document history through a familiar review interface. The underlying git repository is always accessible to technical users who need it.

### 3.9 Extension architecture — the secondary moat

folivm's core ships with no AI dependency, no cloud dependency, and no third-party integrations. It works fully offline, permanently. Extensions that require network connectivity declare it. Core folivm never makes a network call.

The extension architecture is folivm's secondary moat — second only to the format itself. A rich, well-documented, stable extension API means that every vertical tool has a strong reason to build a folivm extension. Each extension that ships makes folivm more valuable for that vertical and harder to displace.

Extensions register capabilities through a typed API:

| Capability | Examples |
|---|---|
| Cell type handlers | AI extension handles `cell:ai`; citation extension handles `cell:citation` |
| Sidebar panels | Zotero registers a citation browser panel |
| Right rail panels | LegalMind registers a clause analysis panel |
| Toolbar commands | ClearBrief adds a "Check brief" command |
| Export hooks | DOCX extension hooks into the export pipeline with full document-model access |
| Data sources | CSV extension registers `source: csv`; Salesforce registers `source: sfdc` |
| Style contributions | A theme extension ships with named styles and a `.fvm-theme` |
| Content library items | A legal extension ships with a standard clause library |
| Document lifecycle hooks | An integrity extension hooks into on-save to run plagiarism checks |

**Key extension verticals:**

| Vertical | Extensions |
|---|---|
| AI providers | OpenAI, Anthropic, local LLMs (Ollama), specialised legal/academic AI |
| Reference management | Zotero, Mendeley, EndNote |
| Legal & compliance | LegalMind, ClearBrief, contract analysis tools |
| Proposal & bid | Qorus-equivalent content bank, approval workflow |
| Technical writing | Mermaid diagrams, Lucidchart, DITA/XML export |
| Academic | Plagiarism checking (Turnitin, Copyleaks), style checkers (Paperpal) |
| Data & CRM | CSV, Salesforce, HubSpot, bespoke CRM integrations |
| Signatures | DocuSign, Adobe Acrobat Sign |
| Writing quality | ProWritingAid, Outwrite, grammar and style analysis |

Extensions that require document model access for export (DITA, TeX, HTML) receive a structured document tree — not a rendered stream. This enables faithful transformation to arbitrary output formats.

---

## 4. Target Users

### 4.1 The consultant
Produces client-facing deliverables: proposals, reports, strategy documents, due diligence packs. Works across multiple clients, each with their own brand requirements. Increasingly expected to use AI to accelerate drafting while maintaining quality and structure. Frustrated by Word's formatting chaos and by AI tools that produce unstructured output that needs reformatting before it can be delivered.

**folivm gives them:** Client theme files that produce on-brand documents instantly. AI extensions that draft into the document's structure. Version history that tracks every draft without managing numbered file copies. A content library of approved section templates and standard recommendations.

### 4.2 The academic / researcher
Produces long-form structured documents: papers, literature reviews, grant proposals, theses. Needs robust outlining, citation management, equation support, and version control. Writes in phases — structure first, content second, formatting last. Frustrated by word processors that conflate writing and layout, and by the brittleness of long `.docx` files. Requires `cell:math` for equations, citation extensions for bibliography management, and integrity checking before submission.

**folivm gives them:** A genuine outliner as the primary writing environment. Git-backed version history that meaningfully tracks structural changes. Native `cell:math` for LaTeX equations. Extension integration with Zotero, Mendeley, and integrity checkers. A format that academic toolchains can process without conversion.

### 4.3 The professional writer
Produces articles, editorial content, reports, and long-form pieces. Works to style guides and brand guidelines. Uses AI to assist with drafting, research, and editing but needs to maintain voice and control over structure. Frustrated by tools that either lack AI capability or are entirely AI-driven with no structural discipline.

**folivm gives them:** A writing environment that supports structured drafting without imposing layout concerns. Word count targets and focus mode in Outline mode. AI extensions that assist within the document's semantic structure. Export to the formats publishers and clients require.

### 4.4 The specialist professional (law, conveyancing, compliance)
Produces regulated documents: contracts, agreements, briefs, compliance reports. Every document must be structurally consistent, tracked, and auditable. AI assistance is increasingly expected but the documents are high-stakes — hallucinations in a contract clause are a liability. Current tools produce binary files that break every attempt at AI integration.

**folivm gives them:** An ASCII-first format where AI-generated cells are explicitly typed and attributable — the editor distinguishes what a human wrote from what an AI generated. Deterministic `cell:data` fields for critical values that must never be AI-generated. A standard clause library with version-pinned `cell:include` references. Version history with full audit trail. A format that is diffable, archivable, and processable by legal toolchains.

### 4.5 The technical writer
Produces documentation, manuals, API references, and technical specifications. Works across structured content formats — often needing to output to multiple targets (PDF, HTML, DITA XML). Requires consistent terminology, reusable content blocks (notices, warnings, standard API table structures), and diagram support.

**folivm gives them:** A content library for standard blocks and notice templates with version control. Native `cell:diagram` for Mermaid and draw.io diagrams. Export extensions that provide DITA XML and HTML output with full document-model access. A semantic structure that maps cleanly to DITA topic types.

### 4.6 The bid / proposal manager
Assembles complex proposal and tender documents under time pressure, drawing from banks of approved content. Requires rapid document assembly from reusable sections, digital signature integration, and consistent brand presentation. Frustrated by the fragility of Word templates and the manual effort of assembling content from multiple approved sources.

**folivm gives them:** A content library that serves as an approved content bank — sections, pricing tables, case studies, and standard terms — with version pinning to ensure only approved versions are used. Theme files that guarantee brand consistency across all proposals. Signature extension integration. Assembly of a complex proposal from library items as a first-class workflow.

---

## 5. Unique Value Proposition

**folivm is the only word processor built for an era where documents are read by both humans and machines.**

| Dimension | folivm | Word / Pages | Notion / Obsidian | Google Docs |
|---|---|---|---|---|
| AI-native format | Yes | No | Partial | No |
| Semantic styling enforced | Yes | No | No | No |
| Design token theme system | Yes | No | No | No |
| Version control (git) | Yes | No | No | No |
| Offline, local-first | Yes | Partial | No | No |
| Professional page layout | Yes | Yes | No | Partial |
| Physical measurement fidelity | Yes | Yes | No | No |
| WCAG-accessible by construction | Yes | No | No | No |
| Content library (versioned) | Yes | No | No | No |
| Extension ecosystem | Yes | Limited | Limited | Limited |
| Open file format | Yes | No | Partial | No |

No existing tool scores across all of these. folivm's moat is the intersection of professional layout fidelity, ASCII-first format, semantic style discipline, and a principled extension ecosystem.

---

## 6. Why now

The right question is not why folivm should exist — it is why nobody built it before, and why now is the right time.

### Why it wasn't built before

**The format predates the problems.** `.docx` was designed in the late 1980s to solve the problems of the late 1980s: desktop printing, WYSIWYG fidelity on CRT screens, sharing files between PCs. At that time, none of the following existed: git (2005), Markdown (2004), LLMs (2022+), WCAG as an enforced standard, YAML as a universal data format. The binary format was the right answer for 1989. It is the wrong answer for 2025.

**WYSIWYG was the right call — then.** Before word processors, professionals used typewriters or markup languages like troff. "What You See Is What You Get" was genuinely revolutionary. It drove mass PC adoption in offices. But WYSIWYG trains users to think about appearance while writing, and once that habit is established at a civilisational scale, it is very hard to change. Every word processor since Word has optimised for the same mental model.

**LaTeX solved the format problem — and nobody used it.** LaTeX is ASCII-first, semantic, version-controllable, and produces beautiful output. It has been the gold standard for academic publishing since the 1980s. It failed to cross into mainstream professional use because the authoring experience is hostile. It looks like programming. There is no WYSIWYG. There is no corporate theme system. folivm is essentially solving the LaTeX UX problem — keeping the format's structural integrity while making the authoring experience as good as a modern word processor.

**Markdown got halfway there and stopped.** Markdown (2004) is ASCII-first and beloved. Obsidian, Bear, iA Writer — excellent tools. But standard Markdown has no layout primitives, no semantic styling layer, and the step from "Markdown note" to "professional deliverable" was never completed cleanly. Nobody merged Markdown's simplicity with Word's layout capability and Figma's design token system.

**Notion proved professionals would switch — but chose the wrong trade-offs.** Notion demonstrated that professionals would adopt a non-Word tool for a better experience. But Notion chose cloud-first, proprietary format, no offline, no page layout, and no semantic styling. It solved the collaboration problem but not the professional document production problem.

**The professional document market is unglamorous.** Startups and investors chase consumer apps, developer tools, and AI wrappers. "Better Word for lawyers and consultants" does not generate venture excitement. The people who feel the pain most acutely — law firms, compliance teams, bid managers — are not typically the people who found software startups.

### Why now is different

Six things converged simultaneously for the first time:

| Convergence | Detail |
|---|---|
| **LLMs made the binary format actively painful** | Post-2022, every professional is trying to use AI with documents. The `.docx` round-trip problem went from theoretical to daily frustration overnight. |
| **WCAG enforcement is maturing** | Accessibility compliance is now a legal obligation in government, financial services, healthcare, and increasingly corporate contexts. Word requires remediation after the fact. folivm produces compliant output by default. |
| **Developer tools crossed into the professional mainstream** | A growing cohort of consultants, academics, and writers are comfortable with git, Markdown, and YAML. The IDE metaphor is no longer intimidating to this audience. |
| **Tauri made native desktop viable without Electron's weight** | Before Tauri (2022), building a performant local-first desktop app meant Electron — 150MB, 300MB RAM baseline. Tauri changes the calculus: small binary, native performance, offline-first. |
| **The tooling stack matured simultaneously** | TipTap (2021), mature Rust ecosystem (~2019), Deno Core embeddable (2020), docx-rs. The combination of tools needed to build folivm correctly simply did not exist five years ago. |
| **AI created a generation that thinks about documents as data** | The professionals who feel the pain of the binary format most acutely are increasingly the ones who have tried to build AI workflows around their documents and hit the wall. They are already primed for the solution. |

### The window

The market need is newly urgent. The tooling is newly viable. The target users are newly receptive. The nearest competitors each have a fundamental gap: Notion (no layout), LaTeX (no UX), Word (no semantics, no AI-native format), Quarto (academic/data science only, no general professional use).

The window for a well-executed ASCII-first professional word processor is open now in a way it has never been before.

---

## 7. Core Principles

These principles are not aspirational. They govern every product decision.

**1. The format is the product.**
The `.fvm` format is folivm's primary contribution. The editor serves the format. If a feature cannot be expressed in the format, it should not exist.

**2. Semantic over decorative.**
Structure is always more important than appearance. Style is always defined, never improvised. No escape hatch to raw formatting.

**3. Content before presentation.**
Outline mode is the default. Design mode is for review. The workflow flows from thinking to writing to presentation — not the reverse. Users who prefer Design mode are not blocked; they are nudged through a better Outline experience.

**4. Offline first, always.**
The core product makes no network calls. Every core feature works without connectivity, permanently. Extensions that need the network declare it explicitly.

**5. Extensions, not bloat.**
AI, citations, CRM integration, legal tools — these are extensions. The core is a word processor. Features that serve a specific vertical belong in extensions, not in core. The v1 lesson: scope discipline prevents bloat.

**6. The extension API is a product surface.**
The extension API is folivm's secondary moat. It must be designed with the same rigour as the format. A stable, rich, well-documented extension API is what turns folivm from a word processor into a platform.

**7. Accessibility is structural, not remedial.**
The semantic format makes WCAG compliance the path of least resistance. Headings are real headings. Alt text is required. Meaning is never conveyed by colour or icon alone. No accessibility audit step should be needed for a correctly authored folivm document.

**8. Nudge, never enforce (except styling).**
Better habits are built through a better experience, not through prohibition. The one exception is ad-hoc formatting — this is the non-negotiable.

**9. Physical fidelity is non-negotiable.**
What the user sees in Design mode must be what they get in export. All measurements are stored in physical units (pt/mm). The pt→px pipeline uses a single scale factor applied uniformly to ruler, canvas, margins, and tab stops. The Tauri layer provides actual screen DPI for accurate physical rendering.

---

## 8. What folivm is not

- **Not a note-taking app.** There is no infinite canvas, no backlinks graph, no daily notes.
- **Not an AI editor.** AI is an extension. The core product has no AI dependency.
- **Not a collaboration platform.** v1.0 is single-user. Multi-user is a future milestone.
- **Not a publishing platform.** Core export targets are PDF and DOCX. Web publishing is an extension concern.
- **Not a replacement for InDesign.** Page layout is professional but not DTP-grade. The target is Word-equivalent fidelity, not print-production fidelity.
- **Not a spreadsheet.** Formula tables are a format feature; full spreadsheet capability is out of scope for v1.0.

---

## 9. Success — what good looks like

**Legal:** A law firm manages their standard contract library as `.fvm` files in a git repository. A paralegal opens a matter template. The CRM extension populates `cell:data` fields with client and matter data. A LegalMind extension drafts bespoke clauses into `cell:ai` blocks. Standard boilerplate is inserted via `cell:include` references pointing to the firm's versioned clause library. The partner reviews in Design mode using track changes, accepts changes, and exports to PDF and DOCX. The workflow is auditable, versionable, and the output is pixel-perfect on brand.

**Consulting:** A consultant produces a strategy report. They begin in Outline mode, structuring sections and assigning styles before writing a word. An AI extension assists with a market analysis section — the draft appears as an attributed `cell:ai` block. Approved section templates from the content library populate the standard recommendations section. The consultant switches to Design mode to verify layout, adjusts a margin, and exports to PDF. The client receives a document that perfectly reflects their brand theme. The same document is rebased for a second client by switching the Brand collection mode — not by reformatting.

**Academic:** A researcher writes a grant proposal. They outline structure in Outline mode, write each section, insert LaTeX equations via `cell:math`, add citations via the Zotero extension, and track changes between drafts. The `.fvm` source is archived in git as the definitive record. The final document is exported to PDF for submission.

**Technical writing:** A technical writer produces an API reference manual. They assemble the document from content library items — standard warning notices, API table templates, code example blocks. A Mermaid extension renders architecture diagrams from `cell:diagram` blocks. A DITA export extension produces the XML output for the documentation CMS alongside the PDF for the print manual. Both outputs derive from the same `.fvm` source.

**Bid management:** A bid manager assembles a 60-page tender response under a 48-hour deadline. They assemble approved content sections from the content library — case studies, standard terms, pricing table templates. The corporate theme ensures every page is on brand. A DocuSign extension adds signature fields. The final document is exported to PDF and submitted. Every content item used is version-pinned, ensuring only approved content was included.

---

## 10. Open questions (as of v0.2)

- **Monetisation model** — TBD. Architecture is agnostic. Candidate models: one-time purchase, freemium core + paid extension marketplace, team licensing.
- **Extension distribution** — First-party marketplace, third-party hosting, or local install only? Runtime model (WASM sandbox, Deno, native plugin)?
- **Format governance** — Is `.fvm` an open published specification or proprietary? Open spec enables ecosystem; proprietary protects moat.
- **Formula tables** — Lightweight formula support (SUM, basic arithmetic) in v1.0, or deferred entirely?
- **Multi-user timeline** — What triggers the multi-user milestone?

---

*Document history tracked in git. Next: Functional Requirements (FR.md)*
