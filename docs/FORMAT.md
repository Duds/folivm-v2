# folivm — Format Specification
**Version:** 1.0 (draft)
**Date:** 2026-03-13
**Status:** In review

---

## 1. Design Principles

The `.fvm` format is the authoritative source of truth for every folivm document. The parser, the renderer, the export pipeline, and all extension APIs operate on data derived from this format. Every architectural decision in the format reflects these principles:

1. **ASCII-first.** The format is UTF-8 encoded plain text. Every structural element uses printable ASCII. No binary sections. No encoded blobs. A `.fvm` file can be read, edited, and diffed with any text tool.

2. **Human-readable.** A `.fvm` file opened in any text editor should communicate its structure clearly. Markup is minimal and visually unobtrusive.

3. **Semantics over presentation.** The format stores what content *is*, not what it looks like. `Heading1` is stored; `font-size: 22pt bold` is not.

4. **Round-trip stable.** `parse(serialise(model)) == model`. The serialiser produces canonical output. Whitespace and ordering are deterministic. Every save of an unchanged document produces an identical file.

5. **Git-friendly.** Each block is a logically independent unit separated by a blank line. Editing one paragraph does not affect adjacent lines. Diffs are meaningful at the block level.

6. **Extension-compatible.** Unknown cell types, unknown frontmatter keys, and unknown style names are preserved verbatim during parse. The parser never discards data it does not understand.

---

## 2. File Conventions

| Concern | Rule |
|---|---|
| Extension | `.fvm` |
| Encoding | UTF-8, no BOM |
| Line endings | LF (`\n`). CRLF is accepted on parse, normalised to LF on serialise |
| Trailing newline | File ends with exactly one `\n` |
| Indentation | Spaces only. Two spaces per indent level inside cell metadata |
| Measurements | All numeric measurements are in **pt** unless the property name specifies otherwise. Never px |
| Null values | Omit optional keys rather than writing `null` or `~` |

---

## 3. Document Structure

A `.fvm` file has two sections, separated by the YAML frontmatter fence:

```
---
[frontmatter: YAML]
---
[body: blocks]
```

The opening `---` must be the first three characters of the file, on their own line. The closing `---` ends the frontmatter and begins the body. The body is a sequence of blocks separated by exactly one blank line.

---

## 4. Frontmatter

The frontmatter is a YAML document. All keys are optional unless marked **required**.

### 4.1 Schema

```yaml
---
# Document identity
title: "Contract for Services"        # string — required
author: "Jane Smith"                  # string
date: "2026-03-13"                    # string, ISO 8601 date
lang: "en-AU"                         # string, BCP-47 language tag

# Page geometry — all measurements in mm
page_size: A4                         # A4 | USLetter | A5 | Legal | Custom
page_width: 210                       # number (mm) — required if page_size: Custom
page_height: 297                      # number (mm) — required if page_size: Custom
orientation: portrait                 # portrait | landscape

# Margins — all measurements in pt
margins:
  top: 72                             # number (pt) — default: 72 (25.4mm)
  bottom: 72
  left: 72
  right: 72

# Theme
theme: ./themes/corporate.fvm-theme   # string — relative path from document

# Header and footer (three-zone layout)
header:
  left: "{title}"                     # string — may contain tokens
  center: ""
  right: "{page} of {pages}"
footer:
  left: "{author}"
  center: ""
  right: "{date}"

# Metadata
tags: [legal, contract, nda]          # array of strings
version: "2.3"                        # string — document version label

# Extension-defined keys (preserved verbatim by parser)
crm:
  client_id: "C-12345"
  matter_id: "M-98765"
zotero:
  library: "shared"
---
```

### 4.2 Page size reference

| Value | Width × Height (mm) |
|---|---|
| `A4` | 210 × 297 |
| `USLetter` | 215.9 × 279.4 |
| `A5` | 148 × 210 |
| `Legal` | 215.9 × 355.6 |
| `Custom` | requires `page_width` and `page_height` |

When `orientation: landscape`, width and height are transposed by the renderer. The stored values always reflect portrait orientation.

### 4.3 Built-in tokens

Tokens used in header, footer, and body content are resolved at render and export time:

| Token | Resolves to |
|---|---|
| `{title}` | `frontmatter.title` |
| `{author}` | `frontmatter.author` |
| `{date}` | `frontmatter.date` |
| `{page}` | Current page number (Design mode and export only) |
| `{pages}` | Total page count (Design mode and export only) |
| `{fig}` | Auto-incrementing figure number across all `cell:image` captions |
| `{version}` | `frontmatter.version` |

Extension-provided tokens use dotted namespace paths: `{crm.client.name}`, `{crm.matter.reference}`. The namespace prefix matches the extension's registered data source identifier.

---

## 5. Block Identifiers

Every block has a stable UUID assigned at creation. The identifier is stored as an HTML comment on the line immediately preceding the block's content or opening fence.

```
<!-- block:a3f2b1c4-e5d6-47a8-b9c0-d1e2f3a4b5c6 -->
```

**Format:** `<!-- block:` followed by a lowercase UUID v4, followed by ` -->`.

**Rules:**
- The comment must be on its own line with no trailing content
- One blank line separates the comment from the preceding block
- The comment is immediately followed (no blank line) by the block it identifies
- New blocks created by the editor receive a freshly generated UUID v4
- Block IDs are never reused, even after a block is deleted
- The serialiser writes block IDs for every block. The parser treats a block without an ID as a new block and assigns one.

**Example:**

```
<!-- block:a3f2b1c4-e5d6-47a8-b9c0-d1e2f3a4b5c6 -->
This is a paragraph.

<!-- block:b4c3d2e1-f5a6-48b7-c8d9-e0f1a2b3c4d5 -->
# Section One
```

---

## 6. Block Types

### 6.1 Paragraph

A paragraph is one or more lines of inline content with no special prefix. The default style is `Body`. An explicit style is declared in the block identifier comment.

**Default style (Body):**
```
<!-- block:uuid -->
This is body text. It may span multiple lines and contain
inline styles and token substitutions.
```

**Explicit style:**
```
<!-- block:uuid style="Lead" -->
This paragraph uses the Lead style.
```

**Rules:**
- A paragraph ends at the next blank line
- A soft line break within a paragraph is `\` followed by a newline (renders as a line break without starting a new paragraph)
- Hard wrapping is normalised on serialise: paragraphs are serialised as a single line. The parser accepts multi-line paragraphs.

### 6.2 Heading

Headings use standard Markdown ATX syntax. The heading level maps to a built-in heading style.

```
<!-- block:uuid -->
# Document Title

<!-- block:uuid -->
## Section Heading

<!-- block:uuid -->
### Subsection Heading
```

| Prefix | Default style |
|---|---|
| `#` | `Heading1` |
| `##` | `Heading2` |
| `###` | `Heading3` |
| `####` | `Heading4` |
| `#####` | `Heading5` |
| `######` | `Heading6` |

A heading may use an explicit style override in the block identifier:

```
<!-- block:uuid style="ChapterTitle" -->
# Part One: Foundations
```

Headings may contain inline content (styled spans, tokens) but not nested blocks.

### 6.3 List

Lists follow standard Markdown syntax. The entire list (including all items) is one block with one block identifier.

**Unordered:**
```
<!-- block:uuid style="ListBullet" -->
- First item
- Second item
- Third item with [inline style]{.Emphasis}
```

**Ordered:**
```
<!-- block:uuid style="ListNumber" -->
1. First item
2. Second item
3. Third item
```

**Nested (up to three levels):**
```
<!-- block:uuid style="ListBullet" -->
- Top-level item
  - Nested item (two-space indent)
    - Doubly nested item (four-space indent)
- Another top-level item
```

**Rules:**
- Unordered list markers: `-` (preferred), `*`, `+` — all accepted on parse, serialised as `-`
- Ordered list markers: `N.` where N is any positive integer — serialised with sequential numbers starting at 1
- Nested items use two-space indentation per level
- Mixed ordered/unordered nesting is permitted
- A list block ends at the next blank line

### 6.4 Table

Tables follow GitHub Flavoured Markdown table syntax.

```
<!-- block:uuid style="Table" -->
| Party | Role | Signature Required |
|---|---|---|
| {crm.client.name} | Client | Yes |
| {crm.firm.name} | Service Provider | Yes |
```

**Rules:**
- The separator row (second row, containing `---`) is required
- Column alignment is specified in the separator row: `---` (left), `:---:` (centre), `---:` (right)
- Cells may contain inline content but not block content
- Tables are serialised with columns padded to equal width for readability

### 6.5 Blockquote

```
<!-- block:uuid style="BlockQuote" -->
> The quick brown fox jumps over the lazy dog.
> This continues the same blockquote.
```

**Rules:**
- Standard Markdown `>` prefix
- Multi-line blockquotes use `>` on every line
- Blockquote style may be overridden in the block identifier

### 6.6 Callout block

A callout is a fenced container that applies a named block style to all content inside it. The style name follows `:::`.

```
<!-- block:uuid -->
:::Tip
<!-- block:uuid -->
This is a tip. It can contain multiple paragraphs.

<!-- block:uuid -->
Each paragraph inside is its own block with its own ID.
:::
```

```
<!-- block:uuid -->
:::Warning
<!-- block:uuid -->
This action cannot be undone.
:::
```

**Rules:**
- Opening fence: `:::StyleName` on its own line
- Closing fence: `:::` on its own line
- The callout block identifier comment goes before the opening fence
- Each nested block inside a callout has its own block identifier
- Callouts may not be nested (a callout inside a callout is a parse error)
- The style name must be a valid identifier: letters, digits, hyphens (no spaces)
- If the named style does not exist in the theme, the callout renders with a default border and no icon

### 6.7 Section block

A section is a fenced container for grouping related blocks. It supports folding in Outline mode and maps to document sections in export.

```
<!-- block:uuid -->
:::section id="executive-summary" title="Executive Summary" numbered="true"
<!-- block:uuid -->
## Executive Summary

<!-- block:uuid -->
This section provides an overview of our proposal.
:::
```

**Section attributes:**

| Attribute | Type | Description |
|---|---|---|
| `id` | string | Stable identifier for cross-reference and TOC anchor |
| `title` | string | Display name in TOC and Outline mode. Falls back to first heading text if omitted |
| `numbered` | boolean | Whether section participates in automatic section numbering. Default: `false` |
| `page-break-before` | boolean | Force a page break before this section in Design/Export. Default: `false` |

**Rules:**
- Sections may be nested to any depth
- Each section has its own block identifier
- The `id` must be unique within the document

---

## 7. Cell Types

Cells are fenced blocks with structured metadata. The opening fence is `:::cell:type`. A YAML metadata block follows the fence, terminated by `---`. If the cell type has body content (text, source code), it follows the `---` separator.

**General cell structure:**

```
<!-- block:uuid -->
:::cell:type
key: value
key: value
---
optional body content
:::
```

If the cell has no body content, the `---` separator is omitted:

```
<!-- block:uuid -->
:::cell:type
key: value
key: value
:::
```

### 7.1 cell:image

Embeds an image. Raster images reference an external file. SVG may be inlined.

```
<!-- block:uuid -->
:::cell:image
src: assets/architecture-diagram.png
alt: System architecture showing three tiers
caption: "Figure {fig}: Three-tier system architecture"
width: 80
align: center
style: Figure
:::
```

```
<!-- block:uuid -->
:::cell:image
alt: Company logo
align: right
width: 120
---
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 60">
  <!-- SVG content inlined for vector assets -->
</svg>
:::
```

**Attributes:**

| Key | Type | Required | Description |
|---|---|---|---|
| `src` | string | if no inline SVG | Relative path from document to image file |
| `alt` | string | **required** | Alt text for accessibility. Empty string is a parse error |
| `caption` | string | | Caption text. May contain `{fig}` token |
| `width` | number | | Width in pt. If omitted, image renders at its natural size up to the content column width |
| `align` | string | | `left` \| `center` \| `right`. Default: `left` |
| `style` | string | | Block style name. Default: `Figure` |

**Rules:**
- `src` and inline SVG are mutually exclusive
- Raster image files are stored in an `assets/` directory relative to the document
- SVG content is written after the `---` separator
- The `{fig}` token in `caption` auto-increments across all `cell:image` captions in document order
- Images larger than 1MB should be tracked with git-lfs

### 7.2 cell:math

Embeds a mathematical expression.

```
<!-- block:uuid -->
:::cell:math
syntax: latex
display: block
---
E = mc^2
:::
```

```
<!-- block:uuid -->
:::cell:math
syntax: latex
display: block
---
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
:::
```

**Attributes:**

| Key | Type | Required | Description |
|---|---|---|---|
| `syntax` | string | **required** | `latex` (only supported value in v1.0) |
| `display` | string | | `block` (centred, own line) \| `inline` (within text). Default: `block` |

**Rules:**
- The body contains the raw LaTeX source, no delimiters
- Inline math (`display: inline`) is treated as a cell block rather than an inline element
- Rendered in Outline mode as the raw LaTeX source in monospace
- Rendered in Design mode via the math renderer extension (built-in: KaTeX)

### 7.3 cell:diagram

Embeds a diagram from a text-based source format.

```
<!-- block:uuid -->
:::cell:diagram
renderer: mermaid
alt: Sequence diagram showing authentication flow
---
sequenceDiagram
  Client->>Server: POST /auth
  Server-->>Client: 200 OK + token
:::
```

**Attributes:**

| Key | Type | Required | Description |
|---|---|---|---|
| `renderer` | string | **required** | Identifier of the renderer extension (`mermaid`, `plantuml`, etc.) |
| `alt` | string | **required** | Alt text for accessibility |
| `caption` | string | | Caption text |
| `width` | number | | Display width in pt |
| `align` | string | | `left` \| `center` \| `right`. Default: `center` |

**Rules:**
- The body contains the raw diagram source
- Rendered in Outline mode as the first line of the source with a cell type indicator
- Rendered in Design mode by the named renderer extension
- If the renderer extension is not installed, a placeholder is shown with the source text

### 7.4 cell:data

Declares a data binding. The resolved values are substituted at render and export time.

```
<!-- block:uuid -->
:::cell:data
source: crm
fields:
  - client.name
  - client.address
  - client.abn
:::
```

**Attributes:**

| Key | Type | Required | Description |
|---|---|---|---|
| `source` | string | **required** | Registered data source identifier |
| `fields` | list of strings | **required** | Field paths to resolve from the data source |

**Rules:**
- The cell itself has no rendered body; the resolved values are available as tokens in the document body: `{crm.client.name}`, `{crm.client.address}`, etc.
- Token path prefix is the `source` value
- If the data source is unavailable, tokens render as `[client.name]` placeholders
- One `cell:data` per source per document is sufficient

### 7.5 cell:ai

Contains AI-generated content. The cell preserves both the prompt and the accepted output.

```
<!-- block:uuid -->
:::cell:ai
provider: legalai
model: legal-drafter-v2
style: Body
status: generated
prompt: "Draft a standard limitation of liability clause limiting liability to the total fees paid in the preceding 12 months."
generated_at: "2026-03-13T09:42:11Z"
accepted_at: "2026-03-13T10:15:33Z"
accepted_by: "Jane Smith"
---
Limitation of Liability. To the maximum extent permitted by applicable law,
the Service Provider's total liability to the Client for any claims arising
under or in connection with this Agreement shall not exceed the total fees
paid by the Client to the Service Provider in the twelve (12) months
immediately preceding the event giving rise to the claim.
:::
```

**Attributes:**

| Key | Type | Required | Description |
|---|---|---|---|
| `provider` | string | **required** | Extension identifier that generated this content |
| `model` | string | | Model or configuration identifier, logged for audit |
| `style` | string | | Block style applied to the generated content. Default: `Body` |
| `status` | string | **required** | `prompt` \| `generating` \| `generated` \| `accepted` \| `rejected` |
| `prompt` | string | **required** | The prompt that produced this output |
| `generated_at` | string | | ISO 8601 timestamp of generation |
| `accepted_at` | string | | ISO 8601 timestamp of acceptance |
| `accepted_by` | string | | Name of user who accepted the content |

**Status values:**

| Status | Meaning | Body present |
|---|---|---|
| `prompt` | Prompt recorded, generation not yet started | No |
| `generating` | Generation in progress (transient, not written to disk) | No |
| `generated` | Content generated, pending review | Yes |
| `accepted` | Content accepted by user, treated as document content | Yes |
| `rejected` | Content rejected; cell is a stub | No |

**Rules:**
- The body (after `---`) contains the generated Markdown-like content as plain text
- Accepted content participates in search, word count, and export identically to normal blocks
- The git commit that records acceptance uses `{provider} via folivm` as the commit author
- A `cell:ai` with `status: accepted` may be edited directly; edits update the body and clear `accepted_at` / `accepted_by` (the content is now human-authored)

### 7.6 cell:include

References a versioned Content Library item.

```
<!-- block:uuid -->
:::cell:include
ref: library.indemnity-standard
version: "2.3"
:::
```

**Attributes:**

| Key | Type | Required | Description |
|---|---|---|---|
| `ref` | string | **required** | Library item reference in the form `library.item-id` |
| `version` | string | **required** | Semantic version of the library item to include |

**Rules:**
- The cell renders by inlining the resolved library item's blocks at the cell position
- The version pin is strict: `"2.3"` resolves only to version 2.3.x
- If the referenced version is not found, a placeholder is shown with the reference and version
- When a newer version of the library item is available, an update indicator is shown; the user must explicitly accept the update
- The update action changes the `version` value and creates a new git commit

### 7.7 cell:citation

Inserts a formatted citation.

```
<!-- block:uuid -->
:::cell:citation
key: smith2023contract
style: APA
:::
```

**Attributes:**

| Key | Type | Required | Description |
|---|---|---|---|
| `key` | string | **required** | Citation key from the connected reference manager |
| `style` | string | | Citation style identifier. Default: `APA` |

**Rules:**
- Citation rendering requires a citation extension (e.g., Zotero)
- In the absence of a citation extension, the cell renders as `[citation: key]`

### 7.8 cell:signature

Declares a signature field for digital signature workflows.

```
<!-- block:uuid -->
:::cell:signature
label: "Client Signature"
required: true
signer_role: client
:::
```

**Attributes:**

| Key | Type | Required | Description |
|---|---|---|---|
| `label` | string | **required** | Human-readable label for the signature field |
| `required` | boolean | | Whether the signature is required for document completion. Default: `true` |
| `signer_role` | string | | Role identifier for multi-party signature workflows |
| `signed_at` | string | | ISO 8601 timestamp, written by the signature extension on signing |
| `signed_by` | string | | Name of signer, written by the signature extension |

**Rules:**
- Signature collection requires a signature extension (e.g., DocuSign)
- An unsigned required signature field causes the export pipeline to emit a warning

---

## 8. Inline Syntax

Inline content appears within paragraphs, headings, list items, table cells, and blockquotes. Inline elements may not contain block elements.

### 8.1 Built-in inline styles

These map to Markdown conventions and are always available without theme definition:

| Syntax | Semantic style | Notes |
|---|---|---|
| `*text*` or `_text_` | `Emphasis` | Italic rendering by default |
| `**text**` or `__text__` | `Strong` | Bold rendering by default |
| `` `text` `` | `Code` | Monospace rendering by default |
| `^text^` | `Superscript` | |
| `~text~` | `Subscript` | |

Literal `*`, `_`, `` ` ``, `^`, `~` are escaped with `\`: `\*`, `\_`, `` \` ``, `\^`, `\~`.

### 8.2 Named inline styles

Named inline styles use Pandoc span syntax:

```
The [force majeure]{.DefinedTerm} clause applies in the event of...

See [Section 4.2]{.CrossReference} for the applicable rates.

The [GDPR]{.Acronym} requires data processors to...
```

**Syntax:** `[content]{.StyleName}`

**Rules:**
- `StyleName` must be a valid identifier: letters, digits, hyphens (no spaces)
- The style must be defined as an inline-scope style in the active theme
- If the style is not found in the theme, the span renders without styling but is preserved in the model
- Multiple styles are not supported in v1.0: `[text]{.A .B}` is a parse error
- Named inline styles may be nested: `[*important term*]{.DefinedTerm}` applies Emphasis inside DefinedTerm

### 8.3 Token substitution

Tokens are substituted at render and export time. They are not evaluated at parse time.

```
This agreement is entered into on {date} between {crm.client.name}
of {crm.client.address} ("the Client") and {crm.firm.name}.
```

**Rules:**
- Token syntax: `{path}` where `path` is a dotted identifier sequence
- Tokens may appear anywhere inline content is permitted, including header and footer strings
- An unknown token is rendered as `[path]` as a visible placeholder
- Tokens may not span line breaks
- The `{fig}` token is valid only in `cell:image` caption attributes, not in body text

---

## 9. Formal Grammar (Simplified)

```
document      = frontmatter body

frontmatter   = "---\n" yaml-content "---\n"

body          = (blank-line* block blank-line*)*

block         = block-id? block-content
block-id      = "<!-- block:" uuid " -->\n"

block-content = paragraph
              | heading
              | list
              | table
              | blockquote
              | callout
              | section
              | cell

paragraph     = paragraph-line+
heading       = "#"{1,6} " " inline-content "\n"
list          = list-item+
list-item     = ("- " | N". ") inline-content "\n"
              | list-item ("  " list-item)*   (* nested *)
table         = table-header "\n" table-separator "\n" table-row*
blockquote    = ("> " inline-content "\n")+

callout       = ":::" style-name "\n"
                (blank-line* block blank-line*)+
                ":::\n"

section       = ":::section" section-attrs "\n"
                (blank-line* block blank-line*)+
                ":::\n"
section-attrs = (" " attr-key "=" quoted-string)*

cell          = ":::cell:" cell-type "\n"
                cell-meta
                ("---\n" cell-body)?
                ":::\n"
cell-meta     = (key ": " value "\n")*

inline-content = (plain-text | em | strong | code | super | sub
               | named-span | token)*

em            = ("*" | "_") inline-content ("*" | "_")
strong        = ("**" | "__") inline-content ("**" | "__")
code          = "`" text "`"
super         = "^" inline-content "^"
sub           = "~" inline-content "~"
named-span    = "[" inline-content "]{." style-name "}"
token         = "{" token-path "}"
token-path    = identifier ("." identifier)*
style-name    = identifier
identifier    = letter (letter | digit | "-")*
uuid          = hex{8} "-" hex{4} "-" hex{4} "-" hex{4} "-" hex{12}
```

---

## 10. Complete Document Example

```
---
title: "Service Agreement"
author: "Acme Legal Services"
date: "2026-03-13"
lang: "en-AU"
page_size: A4
orientation: portrait
margins:
  top: 72
  bottom: 72
  left: 85
  right: 72
theme: ./themes/acme-corporate.fvm-theme
header:
  left: "CONFIDENTIAL"
  center: ""
  right: "{page} of {pages}"
footer:
  left: "Acme Legal Services"
  center: ""
  right: "{date}"
tags: [services, agreement, v2]
crm:
  client_id: "C-12345"
  matter_id: "M-98765"
---

<!-- block:a1b2c3d4-e5f6-47a8-b9c0-d1e2f3a4b5c6 -->
:::cell:data
source: crm
fields:
  - client.name
  - client.address
  - client.abn
  - matter.description
:::

<!-- block:b2c3d4e5-f6a7-48b9-c0d1-e2f3a4b5c6d7 -->
:::section id="parties" title="Parties" page-break-before="false"

<!-- block:c3d4e5f6-a7b8-49c0-d1e2-f3a4b5c6d7e8 -->
# Service Agreement

<!-- block:d4e5f6a7-b8c9-40d1-e2f3-a4b5c6d7e8f9 -->
This agreement is made on {date} between **{crm.client.name}** ABN {crm.client.abn}
of {crm.client.address} ("the Client") and **Acme Legal Services Pty Ltd** ("the Firm").

:::

<!-- block:e5f6a7b8-c9d0-41e2-f3a4-b5c6d7e8f9a0 -->
:::section id="services" title="Services"

<!-- block:f6a7b8c9-d0e1-42f3-a4b5-c6d7e8f9a0b1 -->
## 1. Scope of Services

<!-- block:a7b8c9d0-e1f2-43a4-b5c6-d7e8f9a0b1c2 -->
The Firm agrees to provide the following services to the Client:

<!-- block:b8c9d0e1-f2a3-44b5-c6d7-e8f9a0b1c2d3 -->
- {crm.matter.description}
- All ancillary services reasonably necessary to complete the above
- Progress reporting as specified in [Schedule A]{.CrossReference}

<!-- block:c9d0e1f2-a3b4-45c6-d7e8-f9a0b1c2d3e4 -->
:::include
ref: library.standard-services-terms
version: "3.1"
:::

:::

<!-- block:d0e1f2a3-b4c5-46d7-e8f9-a0b1c2d3e4f5 -->
:::section id="fees" title="Fees and Payment"

<!-- block:e1f2a3b4-c5d6-47e8-f9a0-b1c2d3e4f5a6 -->
## 2. Fees and Payment

<!-- block:f2a3b4c5-d6e7-48f9-a0b1-c2d3e4f5a6b7 -->
:::Tip
<!-- block:a3b4c5d6-e7f8-49a0-b1c2-d3e4f5a6b7c8 -->
All fees are quoted in Australian dollars (AUD) and are exclusive of GST.
:::

<!-- block:b4c5d6e7-f8a9-40b1-c2d3-e4f5a6b7c8d9 -->
| Service | Fee (ex GST) | Frequency |
|---|---:|---|
| Retainer | 5,000.00 | Monthly |
| Drafting — standard | 450.00 | Per hour |
| Drafting — complex | 650.00 | Per hour |
| Disbursements | At cost | As incurred |

<!-- block:c5d6e7f8-a9b0-41c2-d3e4-f5a6b7c8d9e0 -->
:::cell:ai
provider: legalai
style: Body
status: accepted
prompt: "Draft a limitation of liability clause capping liability at 12 months fees."
generated_at: "2026-03-13T09:42:11Z"
accepted_at: "2026-03-13T10:15:33Z"
accepted_by: "Jane Smith"
---
[Limitation of Liability]{.DefinedTerm}. To the maximum extent permitted by applicable
law, the Firm's total liability to the Client for any claims arising under or in
connection with this Agreement shall not exceed the total fees paid by the Client
to the Firm in the twelve (12) months immediately preceding the event giving rise
to the claim.
:::

:::

<!-- block:d6e7f8a9-b0c1-42d3-e4f5-a6b7c8d9e0f1 -->
:::section id="signatures" title="Signatures" page-break-before="true"

<!-- block:e7f8a9b0-c1d2-43e4-f5a6-b7c8d9e0f1a2 -->
## Signatures

<!-- block:f8a9b0c1-d2e3-44f5-a6b7-c8d9e0f1a2b3 -->
Signed as an agreement on {date}.

<!-- block:a9b0c1d2-e3f4-45a6-b7c8-d9e0f1a2b3c4 -->
:::cell:signature
label: "Client Signature"
required: true
signer_role: client
:::

<!-- block:b0c1d2e3-f4a5-46b7-c8d9-e0f1a2b3c4d5 -->
:::cell:signature
label: "Authorised Signatory — Acme Legal Services"
required: true
signer_role: firm
:::

:::
```

---

## 11. The `.fvm-theme` Format

Theme files use a `.fvm-theme` extension. They are YAML documents.

### 11.1 Overall structure

```yaml
fvm_theme_version: "1.0"       # required

collections:
  Primitives: { ... }
  Semantic:   { ... }
  Brand:      { ... }
  Spacing:    { ... }

styles:
  Body:       { ... }
  Heading1:   { ... }
  # ...
```

### 11.2 Collections

A collection groups related variables. Each collection declares its modes. Variables within the collection have one value per mode.

```yaml
collections:
  CollectionName:
    modes: [mode-one, mode-two]   # list of mode names; first is the default
    variables:
      variable-name:
        type: color | number | string | boolean
        mode-one: value
        mode-two: value
```

**Primitives collection** — single mode only, raw values, no aliases:

```yaml
collections:
  Primitives:
    modes: [default]
    variables:
      color-blue-500:
        type: color
        default: "#2563EB"
      font-size-body:
        type: number        # pt
        default: 11
      font-family-body:
        type: string
        default: "Georgia, serif"
      show-grid:
        type: boolean
        default: false
```

**Semantic collection** — two modes (`screen`, `print`), values are typically aliases to Primitives:

```yaml
  Semantic:
    modes: [screen, print]
    variables:
      text-primary:
        type: color
        screen: "{Primitives.color-gray-900}"
        print: "#000000"
      body-font-size:
        type: number
        screen: "{Primitives.font-size-body}"
        print: "{Primitives.font-size-body}"
```

**Brand collection** — modes are client or brand identities. The first mode is the default:

```yaml
  Brand:
    modes: [default, client-a, client-b]
    variables:
      brand-primary:
        type: color
        default: "{Semantic.accent-primary}"
        client-a: "#D97706"
        client-b: "#7C3AED"
      brand-heading-font:
        type: string
        default: "{Primitives.font-family-heading}"
        client-a: "Playfair Display, serif"
        client-b: "IBM Plex Sans, sans-serif"
```

**Spacing collection** — modes are density settings:

```yaml
  Spacing:
    modes: [normal, compact]
    variables:
      paragraph-space-after:
        type: number
        normal: 12
        compact: 4
      heading-space-before:
        type: number
        normal: 18
        compact: 8
```

### 11.3 Alias syntax

An alias references another variable using `{Collection.variable-name}` syntax:

```yaml
text-primary:
  type: color
  screen: "{Primitives.color-gray-900}"   # alias
  print: "#000000"                         # literal value
```

**Rules:**
- Aliases resolve at theme load time into a flat resolved map
- Circular aliases are a theme validation error
- An alias must reference a variable in the same or a lower-precedence collection (Primitives → Semantic → Brand → Spacing)
- A variable may alias a variable in a different collection but not a different mode within the same collection

### 11.4 Named Styles

Styles bundle variables into named, reusable formatting sets. Every style has a `scope` of either `block` or `inline`.

```yaml
styles:
  Body:
    scope: block
    font-family: "{Semantic.body-font}"
    font-size: "{Semantic.body-size}"          # pt
    line-height: 1.5                            # multiplier
    color: "{Semantic.text-primary}"
    space-after: "{Spacing.paragraph-space-after}"
    space-before: 0
    text-align: left                            # left | center | right | justify

  Heading1:
    scope: block
    font-family: "{Brand.brand-heading-font}"
    font-size: 22
    font-weight: bold
    color: "{Brand.brand-primary}"
    space-before: "{Spacing.heading-space-before}"
    space-after: 6
    line-height: 1.2

  Heading2:
    scope: block
    extends: Heading1                           # inherits all Heading1 properties
    font-size: 18

  Heading3:
    scope: block
    extends: Heading1
    font-size: 14
```

**Style inheritance:**

The `extends` key names a parent style. The child inherits all parent properties and overrides only the properties it declares. Inheritance chains may be up to five levels deep. Circular inheritance is a theme validation error.

```yaml
  Caption:
    scope: block
    extends: Body
    font-size: 9
    color: "{Semantic.text-secondary}"
    text-align: center

  FigureCaption:
    scope: block
    extends: Caption                            # inherits Caption → Body
    font-style: italic
```

### 11.5 Block style properties reference

| Property | Type | Description |
|---|---|---|
| `font-family` | string | CSS font-family stack, e.g. `"Georgia, serif"` |
| `font-size` | number | In pt |
| `font-weight` | string | `normal` \| `bold` \| CSS numeric weight |
| `font-style` | string | `normal` \| `italic` \| `oblique` |
| `color` | color | Text colour |
| `line-height` | number | Multiplier of font size |
| `text-align` | string | `left` \| `center` \| `right` \| `justify` |
| `space-before` | number | Space above block in pt |
| `space-after` | number | Space below block in pt |
| `left-indent` | number | Left indent in pt |
| `right-indent` | number | Right indent in pt |
| `first-line-indent` | number | First line indent in pt (overrides left-indent for first line) |
| `background` | color | Block background colour |
| `border-top` | string | `Npt solid #hex` |
| `border-bottom` | string | `Npt solid #hex` |
| `border-left` | string | `Npt solid #hex` |
| `border-right` | string | `Npt solid #hex` |
| `padding` | number | Inner padding in pt (all sides) |
| `padding-top` | number | pt |
| `padding-bottom` | number | pt |
| `padding-left` | number | pt |
| `padding-right` | number | pt |
| `text-transform` | string | `none` \| `uppercase` \| `lowercase` \| `capitalize` |
| `letter-spacing` | number | In pt |
| `keep-with-next` | boolean | Prevent page break between this block and the next |
| `page-break-before` | boolean | Force page break before this block |
| `widow-control` | boolean | Prevent single-line widows at page bottom. Default: `true` |
| `list-style` | string | `disc` \| `circle` \| `square` \| `decimal` \| `lower-alpha` \| `lower-roman` |
| `list-indent` | number | List item indent in pt |
| `icon` | object | Callout icon (see below) |

### 11.6 Callout icon property

The `icon` property attaches a decorative icon to a block style. It is valid on block-scope styles only.

```yaml
  Tip:
    scope: block
    extends: Body
    background: "#EFF6FF"
    border-left: "3pt solid #2563EB"
    padding-left: 36
    icon:
      source: font-awesome        # font-awesome | custom
      name: lightbulb             # icon name within source
      color: "#2563EB"            # icon colour
      size: 14                    # pt
      position: left              # left | top-left
```

**Rules:**
- `source: font-awesome` uses the bundled Font Awesome Free subset (offline, SVG)
- `source: custom` uses a custom SVG file referenced by `name: assets/icon.svg`
- The document body never references icons — the icon is a theme style property
- Icons are decorative; semantic meaning is carried by the style name and ARIA role

### 11.7 Inline style properties reference

| Property | Type | Description |
|---|---|---|
| `font-family` | string | |
| `font-size` | string | pt value or relative e.g. `0.9em` |
| `font-weight` | string | |
| `font-style` | string | |
| `color` | color | |
| `background` | color | Highlight colour |
| `text-decoration` | string | `underline` \| `line-through` \| `none` |
| `vertical-align` | string | `super` \| `sub` \| `baseline` |
| `letter-spacing` | number | pt |
| `text-transform` | string | |

### 11.8 Complete theme example

```yaml
fvm_theme_version: "1.0"

collections:

  Primitives:
    modes: [default]
    variables:
      color-ink:
        type: color
        default: "#111827"
      color-ink-light:
        type: color
        default: "#6B7280"
      color-blue-500:
        type: color
        default: "#2563EB"
      color-white:
        type: color
        default: "#FFFFFF"
      font-size-body:
        type: number
        default: 11
      font-size-h1:
        type: number
        default: 22
      font-size-h2:
        type: number
        default: 18
      font-size-h3:
        type: number
        default: 14
      font-body:
        type: string
        default: "Georgia, serif"
      font-heading:
        type: string
        default: "Inter, sans-serif"
      font-mono:
        type: string
        default: "JetBrains Mono, monospace"
      space-sm:
        type: number
        default: 6
      space-md:
        type: number
        default: 12
      space-lg:
        type: number
        default: 18

  Semantic:
    modes: [screen, print]
    variables:
      text-primary:
        type: color
        screen: "{Primitives.color-ink}"
        print: "#000000"
      text-secondary:
        type: color
        screen: "{Primitives.color-ink-light}"
        print: "#374151"
      accent:
        type: color
        screen: "{Primitives.color-blue-500}"
        print: "{Primitives.color-blue-500}"
      surface-tip:
        type: color
        screen: "#EFF6FF"
        print: "{Primitives.color-white}"
      surface-warning:
        type: color
        screen: "#FEF3C7"
        print: "{Primitives.color-white}"

  Brand:
    modes: [default, client-a]
    variables:
      brand-primary:
        type: color
        default: "{Semantic.accent}"
        client-a: "#D97706"
      brand-heading-font:
        type: string
        default: "{Primitives.font-heading}"
        client-a: "Playfair Display, serif"

  Spacing:
    modes: [normal, compact]
    variables:
      para-after:
        type: number
        normal: "{Primitives.space-md}"
        compact: 4
      heading-before:
        type: number
        normal: "{Primitives.space-lg}"
        compact: "{Primitives.space-sm}"

styles:

  Body:
    scope: block
    font-family: "{Primitives.font-body}"
    font-size: "{Primitives.font-size-body}"
    line-height: 1.5
    color: "{Semantic.text-primary}"
    space-after: "{Spacing.para-after}"
    space-before: 0
    text-align: justify

  Lead:
    scope: block
    extends: Body
    font-size: 13
    color: "{Semantic.text-secondary}"
    text-align: left

  Heading1:
    scope: block
    font-family: "{Brand.brand-heading-font}"
    font-size: "{Primitives.font-size-h1}"
    font-weight: bold
    color: "{Brand.brand-primary}"
    space-before: "{Spacing.heading-before}"
    space-after: "{Primitives.space-sm}"
    line-height: 1.2
    keep-with-next: true

  Heading2:
    scope: block
    extends: Heading1
    font-size: "{Primitives.font-size-h2}"

  Heading3:
    scope: block
    extends: Heading1
    font-size: "{Primitives.font-size-h3}"

  BlockQuote:
    scope: block
    extends: Body
    left-indent: 24
    color: "{Semantic.text-secondary}"
    border-left: "3pt solid {Semantic.text-secondary}"
    padding-left: 12
    text-align: left

  ListBullet:
    scope: block
    extends: Body
    list-style: disc
    list-indent: 18
    text-align: left

  ListNumber:
    scope: block
    extends: Body
    list-style: decimal
    list-indent: 18
    text-align: left

  Table:
    scope: block
    font-family: "{Primitives.font-body}"
    font-size: 10
    color: "{Semantic.text-primary}"
    space-after: "{Primitives.space-md}"

  Figure:
    scope: block
    space-before: "{Primitives.space-md}"
    space-after: "{Primitives.space-md}"
    text-align: center

  Caption:
    scope: block
    extends: Body
    font-size: 9
    color: "{Semantic.text-secondary}"
    text-align: center
    space-before: 3
    space-after: "{Primitives.space-md}"

  Tip:
    scope: block
    extends: Body
    background: "{Semantic.surface-tip}"
    border-left: "3pt solid {Brand.brand-primary}"
    padding: 12
    padding-left: 36
    text-align: left
    icon:
      source: font-awesome
      name: lightbulb
      color: "{Brand.brand-primary}"
      size: 14
      position: left

  Warning:
    scope: block
    extends: Tip
    background: "{Semantic.surface-warning}"
    border-left: "3pt solid #D97706"
    icon:
      source: font-awesome
      name: triangle-exclamation
      color: "#D97706"
      size: 14
      position: left

  Emphasis:
    scope: inline
    font-style: italic

  Strong:
    scope: inline
    font-weight: bold

  Code:
    scope: inline
    font-family: "{Primitives.font-mono}"
    font-size: "0.9em"
    background: "{Semantic.surface-tip}"
    padding-left: 2
    padding-right: 2

  DefinedTerm:
    scope: inline
    font-style: italic
    font-weight: bold

  CrossReference:
    scope: inline
    color: "{Brand.brand-primary}"

  Superscript:
    scope: inline
    vertical-align: super
    font-size: "0.75em"

  Subscript:
    scope: inline
    vertical-align: sub
    font-size: "0.75em"
```

---

## 12. Content Library Fragment Format

Library items are `.fvm` files without frontmatter. They are sequences of blocks only.

```
<!-- block:uuid -->
The [indemnifying party]{.DefinedTerm} ("Indemnifier") shall defend, indemnify,
and hold harmless the other party from and against any and all claims, damages,
losses, costs, and expenses (including reasonable legal fees) arising out of or
relating to any breach of this Agreement by the Indemnifier.

<!-- block:uuid -->
:::Warning
<!-- block:uuid -->
This clause does not limit liability for fraud, gross negligence, or wilful misconduct.
:::
```

The library item is identified and versioned by its `index.yaml` entry, not by frontmatter. When a `cell:include` resolves an item, the item's blocks are inlined at the cell position with their block IDs preserved.

---

## 13. Extension Manifest Format

Extension packages include a `manifest.yaml`:

```yaml
fvmext_version: "1.0"

id: "com.example.legalai"           # reverse-DNS identifier, unique
name: "LegalAI Drafter"
version: "2.1.0"                     # semantic version
description: "AI-powered legal clause drafting"
author: "Example Legal Tech Ltd"
license: "commercial"

# Declared permissions (user approves on install)
permissions:
  - network                          # make outbound HTTP/HTTPS calls
  - read-document                    # read document model
  - write-document                   # insert/modify blocks

# Registered cell types handled by this extension
cell_types:
  - cell:ai                          # this extension handles cell:ai blocks

# Registered data sources
data_sources:
  - id: legalai
    label: "LegalAI"

# Minimum folivm API version required
min_api_version: "1.0"

# Entry point
main: index.ts
```

---

## 14. Format Versioning

The `fvm_version` frontmatter key records the format version that produced the file. The parser uses this to apply version-specific rules.

```yaml
fvm_version: "1.0"
```

**Forward compatibility:** A parser encountering a higher `fvm_version` than it supports emits a warning and parses what it can. Unknown block types and unknown cell types are preserved as raw content.

**Backward compatibility:** Future format versions may add new block types, new cell attributes, and new theme properties. Removing or renaming existing elements is a major version change.

The theme format version is tracked separately via `fvm_theme_version`.

---

*This document is the authoritative specification for the `.fvm` and `.fvm-theme` formats. The parser test suite verifies compliance via round-trip tests on the examples in this document.*
