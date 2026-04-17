
use crate::model::{Block, BlockId, CellBlock, Inline, ListItem, SectionAttrs, TableCell, TableRow};
use crate::text_engine::buffer::RunBuffer;
use super::cell;
use super::inline as inline_parser;
use anyhow::{bail, Context, Result};
use uuid::Uuid;

/// Parse the body of a `.fvm` document into a list of [`Block`]s.
///
/// The body is everything after the closing `---` of the frontmatter fence.
pub fn parse_blocks(body: &str) -> Result<Vec<Block>> {
    let lines: Vec<&str> = body.lines().collect();
    let mut parser = BlockParser::new(&lines);
    parser.parse_block_sequence()
}

// ---------------------------------------------------------------------------
// Block parser state machine
// ---------------------------------------------------------------------------

struct BlockParser<'a> {
    lines: &'a [&'a str],
    pos: usize,
}

impl<'a> BlockParser<'a> {
    fn new(lines: &'a [&'a str]) -> Self {
        Self { lines, pos: 0 }
    }

    fn peek(&self) -> Option<&'a str> {
        self.lines.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<&'a str> {
        let line = self.lines.get(self.pos).copied();
        if line.is_some() {
            self.pos += 1;
        }
        line
    }

    fn skip_blank_lines(&mut self) {
        while let Some(line) = self.peek() {
            if line.trim().is_empty() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Parse a sequence of blocks until EOF or until `stop_at_fence` matches the
    /// current line (used for Callout/Section inner content).
    fn parse_block_sequence(&mut self) -> Result<Vec<Block>> {
        self.parse_blocks_until(None)
    }

    fn parse_blocks_until(&mut self, stop_fence: Option<&str>) -> Result<Vec<Block>> {
        let mut blocks = Vec::new();
        loop {
            self.skip_blank_lines();
            match self.peek() {
                None => break,
                Some(line) if stop_fence.map_or(false, |f| line == f) => break,
                Some(line) if line == ":::" => break,
                _ => {}
            }

            // Check for block UUID comment
            let id = if let Some(line) = self.peek() {
                if let Some(uuid) = parse_block_id_comment(line) {
                    self.advance();
                    uuid
                } else {
                    Uuid::new_v4()
                }
            } else {
                break;
            };

            self.skip_blank_lines();

            // Now determine block type from the next non-blank line
            let block = match self.peek() {
                None => break,
                Some(line) if line == ":::" || stop_fence.map_or(false, |f| line == f) => break,
                Some(line) => self.parse_one_block(id, line)?,
            };
            blocks.push(block);
        }
        Ok(blocks)
    }

    fn parse_one_block(&mut self, id: BlockId, first_line: &str) -> Result<Block> {
        // Cell fence: :::cell:type
        if let Some(cell_type) = first_line.strip_prefix(":::cell:") {
            let cell_type = cell_type.to_string();
            self.advance(); // consume the fence line
            let cell_block = cell::parse_cell(id, cell_type, self.lines, &mut self.pos)?;
            return Ok(Block::Cell(cell_block));
        }

        // Section fence: :::section ...attrs
        if first_line.starts_with(":::section") {
            let attrs_str = first_line[":::section".len()..].trim();
            let attrs = parse_section_attrs(attrs_str);
            self.advance();
            let inner = self.parse_blocks_until(Some(":::"))?;
            // Consume closing :::
            if self.peek().map_or(false, |l| l == ":::") {
                self.advance();
            }
            return Ok(Block::Section { id, attrs, blocks: inner });
        }

        // Callout fence: :::StyleName (not cell:, not section)
        if let Some(style) = first_line.strip_prefix(":::") {
            if !style.is_empty() && !style.contains(':') {
                let style = style.to_string();
                self.advance();
                let inner = self.parse_blocks_until(Some(":::"))?;
                if self.peek().map_or(false, |l| l == ":::") {
                    self.advance();
                }
                return Ok(Block::Callout { id, style, blocks: inner });
            }
        }

        // Heading: # ... ######
        if first_line.starts_with('#') {
            return self.parse_heading(id, first_line);
        }

        // Blockquote: > ...
        if first_line.starts_with("> ") || first_line == ">" {
            return self.parse_blockquote(id);
        }

        // Table: | ... |
        if first_line.starts_with('|') {
            return self.parse_table(id);
        }

        // List: - / * / + / N.
        if is_list_item(first_line) {
            return self.parse_list(id);
        }

        // Paragraph (default)
        self.parse_paragraph(id)
    }

    // -------------------------------------------------------------------------
    // Individual block parsers
    // -------------------------------------------------------------------------

    fn parse_heading(&mut self, id: BlockId, line: &str) -> Result<Block> {
        // Extract style override from block id line if it was stored differently
        // Here we already consumed the id comment and "line" is the heading line.
        // Check for style override in the comment (handled by parse_block_id_with_style)
        // For now we detect level and parse inline content
        let (level, rest) = parse_heading_level(line)
            .ok_or_else(|| anyhow::anyhow!("invalid heading line: {:?}", line))?;

        // Check for style attribute in the comment line (already consumed). We handle
        // style overrides by re-checking lines; default style is HeadingN.
        let default_style = format!("Heading{}", level);

        let inlines = inline_parser::parse_inline(rest)?;
        let content = RunBuffer::from_inlines(inlines);
        self.advance();
        Ok(Block::Heading { id, level, style: default_style, content })
    }

    fn parse_blockquote(&mut self, id: BlockId) -> Result<Block> {
        let mut raw_lines = Vec::new();
        while let Some(line) = self.peek() {
            if line.starts_with("> ") {
                raw_lines.push(&line[2..]);
                self.advance();
            } else if line == ">" {
                raw_lines.push("");
                self.advance();
            } else {
                break;
            }
        }
        let text = raw_lines.join("\n");
        let inlines = inline_parser::parse_inline(&text)?;
        let content = RunBuffer::from_inlines(inlines);
        let inner = vec![Block::Paragraph {
            id: Uuid::new_v4(),
            style: "Body".to_string(),
            content,
        }];
        Ok(Block::Blockquote { id, style: "BlockQuote".to_string(), blocks: inner })
    }

    fn parse_table(&mut self, id: BlockId) -> Result<Block> {
        let mut raw_rows: Vec<Vec<String>> = Vec::new();
        while let Some(line) = self.peek() {
            if line.trim().is_empty() {
                break;
            }
            if !line.starts_with('|') {
                break;
            }
            let cells: Vec<String> = line
                .trim_matches('|')
                .split('|')
                .map(|c| c.trim().to_string())
                .collect();

            // Detect separator row (all cells are --- variations) - skip it
            let is_sep = cells.iter().all(|c| {
                let t = c.trim_matches('-').trim_matches(':');
                t.is_empty()
            });
            if !is_sep {
                raw_rows.push(cells);
            }
            self.advance();
        }

        // Build rows (skip separator conceptually — it's already removed)
        let rows = raw_rows
            .into_iter()
            .map(|cells| {
                let tcells = cells
                    .into_iter()
                    .map(|text| {
                        let inlines = inline_parser::parse_inline(&text).unwrap_or_default();
                        let content = RunBuffer::from_inlines(inlines);
                        TableCell {
                            colspan: 1,
                            rowspan: 1,
                            blocks: vec![Block::Paragraph {
                                id: Uuid::new_v4(),
                                style: "Body".to_string(),
                                content,
                            }],
                        }
                    })
                    .collect();
                TableRow { cells: tcells }
            })
            .collect();

        Ok(Block::Table { id, style: "Table".to_string(), rows })
    }

    fn parse_list(&mut self, id: BlockId) -> Result<Block> {
        let mut items: Vec<ListItem> = Vec::new();
        let mut ordered = false;

        // We parse list items at indent level 0; nested items become sub-blocks
        while let Some(line) = self.peek() {
            if line.trim().is_empty() {
                break;
            }
            if let Some((item_ordered, indent, content)) = parse_list_marker(line) {
                if indent == 0 {
                    if item_ordered { ordered = true; }
                    let inlines = inline_parser::parse_inline(content)?;
                    let content_buf = RunBuffer::from_inlines(inlines);
                    let block = Block::Paragraph {
                        id: Uuid::new_v4(),
                        style: "Body".to_string(),
                        content: content_buf,
                    };
                    items.push(ListItem { blocks: vec![block] });
                    self.advance();
                } else {
                    // Nested item — attach to last item as a sub-block paragraph
                    let inlines = inline_parser::parse_inline(content)?;
                    let content_buf = RunBuffer::from_inlines(inlines);
                    let block = Block::Paragraph {
                        id: Uuid::new_v4(),
                        style: "Body".to_string(),
                        content: content_buf,
                    };
                    if let Some(last) = items.last_mut() {
                        last.blocks.push(block);
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }

        Ok(Block::List { id, style: "ListBullet".to_string(), items, ordered })
    }

    fn parse_paragraph(&mut self, id: BlockId) -> Result<Block> {
        let mut lines_acc: Vec<&str> = Vec::new();
        // Check if the *original* block-id line carried a style attribute
        // (handled externally; here we use default "Body")
        let style = "Body".to_string();

        while let Some(line) = self.peek() {
            if line.trim().is_empty() {
                break;
            }
            // Stop at any fence opener or UUID comment
            if line.starts_with(":::") || parse_block_id_comment(line).is_some() {
                break;
            }
            lines_acc.push(line);
            self.advance();
        }

        // Join multi-line paragraphs (normalised on serialise)
        let text = lines_acc.join(" ");
        let inlines = inline_parser::parse_inline(text.trim())?;
        let content = RunBuffer::from_inlines(inlines);
        Ok(Block::Paragraph { id, style, content })
    }
}

// ---------------------------------------------------------------------------
// Block comment / UUID parsing
// ---------------------------------------------------------------------------

/// Returns the BlockId if the line is a `<!-- block:UUID -->` comment.
///
/// Also accepts `<!-- block:UUID style="Name" -->` — style is ignored here
/// (the body parser handles style from the block content line).
pub fn parse_block_id_comment(line: &str) -> Option<BlockId> {
    let line = line.trim();
    let inner = line.strip_prefix("<!-- block:")?.strip_suffix("-->")?;
    let inner = inner.trim();
    // May have extra attributes: take the UUID (first word)
    let uuid_str = inner.split_whitespace().next()?;
    Uuid::parse_str(uuid_str).ok()
}

/// Returns (level 1-6, rest of line) from an ATX heading line.
fn parse_heading_level(line: &str) -> Option<(u8, &str)> {
    let level = line.chars().take_while(|&c| c == '#').count() as u8;
    if level == 0 || level > 6 {
        return None;
    }
    let rest = line[level as usize..].trim_start_matches(' ');
    Some((level, rest))
}

/// Returns (ordered, indent_level, content) for a list item line, or None.
fn parse_list_marker(line: &str) -> Option<(bool, usize, &str)> {
    // Count leading spaces (2 spaces per indent level)
    let trimmed = line.trim_start();
    let indent = (line.len() - trimmed.len()) / 2;

    // Unordered: - / * / +
    if let Some(rest) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")).or_else(|| trimmed.strip_prefix("+ ")) {
        return Some((false, indent, rest));
    }
    // Ordered: N.
    if let Some(dot) = trimmed.find(". ") {
        let prefix = &trimmed[..dot];
        if prefix.chars().all(|c| c.is_ascii_digit()) && !prefix.is_empty() {
            return Some((true, indent, &trimmed[dot + 2..]));
        }
    }
    None
}

fn is_list_item(line: &str) -> bool {
    parse_list_marker(line).is_some()
}

/// Parse key=value attribute pairs from a section fence line.
/// Format: ` id="foo" title="Bar" numbered="true" page-break-before="false"`
fn parse_section_attrs(attrs_str: &str) -> SectionAttrs {
    let mut id = None;
    let mut title = None;
    let mut numbered = false;
    let mut page_break_before = false;

    // Simple scanner: key="value" pairs
    let mut remaining = attrs_str;
    while !remaining.is_empty() {
        remaining = remaining.trim_start();
        if remaining.is_empty() { break; }

        // Find key
        let eq = match remaining.find('=') {
            Some(i) => i,
            None => break,
        };
        let key = remaining[..eq].trim();
        remaining = &remaining[eq + 1..];

        // Value is either "quoted" or unquoted word
        let value = if remaining.starts_with('"') {
            let close = remaining[1..].find('"').map(|i| i + 1);
            match close {
                Some(c) => {
                    let v = &remaining[1..c];
                    remaining = &remaining[c + 1..];
                    v
                }
                None => break,
            }
        } else {
            let end = remaining.find(' ').unwrap_or(remaining.len());
            let v = &remaining[..end];
            remaining = &remaining[end..];
            v
        };

        match key {
            "id" => id = Some(value.to_string()),
            "title" => title = Some(value.to_string()),
            "numbered" => numbered = matches!(value, "true" | "yes" | "1"),
            "page-break-before" => page_break_before = matches!(value, "true" | "yes" | "1"),
            _ => {}
        }
    }

    SectionAttrs { id, title, numbered, page_break_before }
}
