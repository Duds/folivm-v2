
//! Canonical `.fvm` serialiser.
//!
//! `serialise(model)` produces deterministic, round-trip-stable output:
//! - Frontmatter in YAML
//! - Each block preceded by its `<!-- block:UUID -->` comment
//! - Blocks separated by exactly one blank line
//! - File ends with exactly one `\n`
//! - Lists normalised: `-` bullets, sequential numbers starting at 1
//! - Paragraphs serialised as a single line (no hard-wrapping)

use crate::model::{Block, CellBlock, DocumentModel, Frontmatter, Inline, ListItem};
use anyhow::Result;

/// Serialise a [`DocumentModel`] to a canonical `.fvm` string.
pub fn serialise(model: &DocumentModel) -> Result<String> {
    let mut out = String::new();

    // --- Frontmatter ---
    out.push_str("---\n");
    out.push_str(&serialise_frontmatter(&model.frontmatter)?);
    out.push_str("---\n");

    // --- Body blocks ---
    let mut first = true;
    for block in &model.blocks {
        if first {
            out.push('\n');
            first = false;
        } else {
            out.push('\n');
            out.push('\n');
        }
        serialise_block(block, &mut out, 0)?;
    }

    // Ensure file ends with exactly one newline
    while out.ends_with("\n\n") {
        out.pop();
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }

    Ok(out)
}

// ---------------------------------------------------------------------------
// Frontmatter serialisation
// ---------------------------------------------------------------------------

fn serialise_frontmatter(fm: &Frontmatter) -> Result<String> {
    use serde_yaml::Value;

    let mut map = serde_yaml::Mapping::new();

    // Required
    map.insert(Value::String("title".into()), Value::String(fm.title.clone()));

    // Optional identity fields
    macro_rules! opt_str {
        ($field:expr, $key:literal) => {
            if let Some(v) = &$field {
                map.insert(Value::String($key.into()), Value::String(v.clone()));
            }
        };
    }
    opt_str!(fm.author, "author");
    opt_str!(fm.date, "date");
    opt_str!(fm.lang, "lang");
    opt_str!(fm.version, "version");

    if !fm.tags.is_empty() {
        let tags = fm
            .tags
            .iter()
            .map(|t| Value::String(t.clone()))
            .collect::<Vec<_>>();
        map.insert(Value::String("tags".into()), Value::Sequence(tags));
    }

    // Page geometry
    opt_str!(fm.page_size, "page_size");
    if let Some(w) = fm.page_width {
        map.insert(Value::String("page_width".into()), Value::Number(serde_yaml::Number::from(w as f64)));
    }
    if let Some(h) = fm.page_height {
        map.insert(Value::String("page_height".into()), Value::Number(serde_yaml::Number::from(h as f64)));
    }
    opt_str!(fm.orientation, "orientation");

    // Margins
    if let Some(m) = &fm.margins {
        let mut margin_map = serde_yaml::Mapping::new();
        macro_rules! opt_num {
            ($f:expr, $k:literal) => {
                if let Some(v) = $f {
                    margin_map.insert(Value::String($k.into()), Value::Number(serde_yaml::Number::from(v)));
                }
            };
        }
        opt_num!(m.top, "top");
        opt_num!(m.bottom, "bottom");
        opt_num!(m.left, "left");
        opt_num!(m.right, "right");
        if !margin_map.is_empty() {
            map.insert(Value::String("margins".into()), Value::Mapping(margin_map));
        }
    }

    opt_str!(fm.theme, "theme");

    // Header / footer
    if let Some(h) = &fm.header {
        let mut hmap = serde_yaml::Mapping::new();
        macro_rules! hf_opt {
            ($f:expr, $k:literal) => {
                if let Some(v) = &$f {
                    hmap.insert(Value::String($k.into()), Value::String(v.clone()));
                }
            };
        }
        hf_opt!(h.left, "left");
        hf_opt!(h.center, "center");
        hf_opt!(h.right, "right");
        map.insert(Value::String("header".into()), Value::Mapping(hmap));
    }
    if let Some(f) = &fm.footer {
        let mut fmap = serde_yaml::Mapping::new();
        macro_rules! hf_opt {
            ($field:expr, $k:literal) => {
                if let Some(v) = &$field {
                    fmap.insert(Value::String($k.into()), Value::String(v.clone()));
                }
            };
        }
        hf_opt!(f.left, "left");
        hf_opt!(f.center, "center");
        hf_opt!(f.right, "right");
        map.insert(Value::String("footer".into()), Value::Mapping(fmap));
    }

    // Extension keys (extra)
    for (k, v) in &fm.extra {
        map.insert(Value::String(k.clone()), v.clone());
    }

    let yaml = serde_yaml::to_string(&Value::Mapping(map))?;
    // serde_yaml produces "---\n...\n" prefix/suffix in some versions — strip them
    let yaml = yaml.strip_prefix("---\n").unwrap_or(&yaml);
    let yaml = yaml.strip_suffix("...\n").unwrap_or(yaml);
    Ok(yaml.to_string())
}

// ---------------------------------------------------------------------------
// Block serialisation
// ---------------------------------------------------------------------------

fn serialise_block(block: &Block, out: &mut String, indent: usize) -> Result<()> {
    let pad = "  ".repeat(indent);

    match block {
        Block::Paragraph { id, style, content } => {
            write_block_comment(out, &pad, id, Some(style));
            out.push_str(&pad);
            let inlines = content.to_inlines();
            out.push_str(&serialise_inlines(&inlines));
            out.push('\n');
        }

        Block::Heading { id, level, style, content } => {
            let default_style = format!("Heading{}", level);
            let style_override = if style != &default_style { Some(style.as_str()) } else { None };
            write_block_comment(out, &pad, id, style_override);
            out.push_str(&pad);
            for _ in 0..*level {
                out.push('#');
            }
            out.push(' ');
            let inlines = content.to_inlines();
            out.push_str(&serialise_inlines(&inlines));
            out.push('\n');
        }

        Block::List { id, style, items, ordered } => {
            write_block_comment(out, &pad, id, Some(style));
            for (i, item) in items.iter().enumerate() {
                serialise_list_items(item, *ordered, i + 1, out, &pad, 0)?;
            }
        }

        Block::Blockquote { id, style, blocks } => {
            write_block_comment(out, &pad, id, Some(style));
            for inner in blocks {
                // Blockquotes: prefix each line with "> "
                let mut inner_out = String::new();
                serialise_block(inner, &mut inner_out, 0)?;
                for line in inner_out.lines() {
                    out.push_str(&pad);
                    out.push('>');
                    if !line.is_empty() {
                        out.push(' ');
                        out.push_str(line);
                    }
                    out.push('\n');
                }
            }
        }

        Block::Table { id, style, rows } => {
            write_block_comment(out, &pad, id, Some(style));
            if rows.is_empty() {
                return Ok(());
            }

            // Build raw cell strings
            let raw: Vec<Vec<String>> = rows
                .iter()
                .map(|row| {
                    row.cells
                        .iter()
                        .map(|cell| {
                            // Extract inline text from cell's first block
                            let text = cell.blocks.first().and_then(|b| {
                                if let Block::Paragraph { content, .. } = b {
                                    let inlines = content.to_inlines();
                                    Some(serialise_inlines(&inlines))
                                } else {
                                    None
                                }
                            }).unwrap_or_default();
                            text
                        })
                        .collect()
                })
                .collect();

            // Compute column widths for alignment
            let n_cols = raw.iter().map(|r| r.len()).max().unwrap_or(0);
            let mut col_widths = vec![3usize; n_cols]; // min 3 for "---"
            for row in &raw {
                for (c, cell) in row.iter().enumerate() {
                    if c < col_widths.len() {
                        col_widths[c] = col_widths[c].max(cell.len());
                    }
                }
            }

            // Serialise header row
            let header = &raw[0];
            out.push_str(&pad);
            out.push('|');
            for (c, cell) in header.iter().enumerate() {
                let width = col_widths.get(c).copied().unwrap_or(3);
                out.push_str(&format!(" {:width$} |", cell, width = width));
            }
            out.push('\n');

            // Separator row
            out.push_str(&pad);
            out.push('|');
            for w in &col_widths {
                out.push_str(&format!(" {} |", "-".repeat(*w)));
            }
            out.push('\n');

            // Data rows
            for row in raw.iter().skip(1) {
                out.push_str(&pad);
                out.push('|');
                for (c, cell) in row.iter().enumerate() {
                    let width = col_widths.get(c).copied().unwrap_or(3);
                    out.push_str(&format!(" {:width$} |", cell, width = width));
                }
                out.push('\n');
            }
        }

        Block::Callout { id, style, blocks } => {
            write_block_comment(out, &pad, id, None);
            out.push_str(&pad);
            out.push_str(":::");
            out.push_str(style);
            out.push('\n');
            for (i, inner) in blocks.iter().enumerate() {
                if i > 0 {
                    out.push('\n');
                }
                serialise_block(inner, out, indent)?;
            }
            out.push_str(&pad);
            out.push_str(":::\n");
        }

        Block::Section { id, attrs, blocks } => {
            write_block_comment(out, &pad, id, None);
            out.push_str(&pad);
            out.push_str(":::section");

            if let Some(id_val) = &attrs.id {
                out.push_str(&format!(r#" id="{}""#, id_val));
            }
            if let Some(title) = &attrs.title {
                out.push_str(&format!(r#" title="{}""#, title));
            }
            if attrs.numbered {
                out.push_str(r#" numbered="true""#);
            }
            if attrs.page_break_before {
                out.push_str(r#" page-break-before="true""#);
            }
            out.push('\n');

            for (i, inner) in blocks.iter().enumerate() {
                if i > 0 {
                    out.push('\n');
                }
                serialise_block(inner, out, indent)?;
            }
            out.push_str(&pad);
            out.push_str(":::\n");
        }

        Block::Cell(cell) => {
            serialise_cell(cell, out, &pad)?;
        }
    }

    Ok(())
}

fn serialise_list_items(
    item: &ListItem,
    ordered: bool,
    index: usize,
    out: &mut String,
    pad: &str,
    indent: usize,
) -> Result<()> {
    let i_pad = "  ".repeat(indent);
    for block in &item.blocks {
        if let Block::Paragraph { content, .. } = block {
            let inlines = content.to_inlines();
            let text = serialise_inlines(&inlines);
            out.push_str(pad);
            out.push_str(&i_pad);
            if ordered {
                out.push_str(&format!("{}. {}", index, text));
            } else {
                out.push_str("- ");
                out.push_str(&text);
            }
            out.push('\n');
        }
    }
    Ok(())
}

fn serialise_cell(cell: &CellBlock, out: &mut String, pad: &str) -> Result<()> {
    // Write <!-- block:UUID -->
    out.push_str(pad);
    out.push_str(&format!("<!-- block:{} -->\n", cell.id));

    out.push_str(pad);
    out.push_str(":::");
    out.push_str(&cell.cell_type);
    out.push('\n');

    // Write attrs
    let obj = cell.attrs.as_object();
    let mut has_body = false;
    let mut body_text = String::new();

    if let Some(obj) = obj {
        for (k, v) in obj {
            if k == "_body" {
                has_body = true;
                body_text = v.as_str().unwrap_or("").to_string();
                continue;
            }
            let v_str = v.as_str().map(|s| s.to_string()).unwrap_or_else(|| v.to_string());
            out.push_str(pad);
            out.push_str(&format!("{}: {}\n", k, v_str));
        }
    }

    if has_body {
        out.push_str(pad);
        out.push_str("---\n");
        for line in body_text.lines() {
            out.push_str(pad);
            out.push_str(line);
            out.push('\n');
        }
    }

    out.push_str(pad);
    out.push_str(":::\n");
    Ok(())
}

// ---------------------------------------------------------------------------
// Inline serialisation
// ---------------------------------------------------------------------------

fn serialise_inlines(inlines: &[Inline]) -> String {
    inlines.iter().map(serialise_inline).collect()
}

fn serialise_inline(inline: &Inline) -> String {
    match inline {
        Inline::Text(s) => escape_inline_text(s),
        Inline::Token { path } => format!("{{{}}}", path),
        Inline::Styled { style, inlines } => {
            let inner = serialise_inlines(inlines);
            match style.as_str() {
                "Emphasis" => format!("*{}*", inner),
                "Strong" => format!("**{}**", inner),
                "Code" => format!("`{}`", inner),
                "Superscript" => format!("^{}^", inner),
                "Subscript" => format!("~{}~", inner),
                // Named span syntax
                _ => format!("[{}]{{.{}}}", inner, style),
            }
        }
    }
}

/// Escape special inline characters in plain text.
fn escape_inline_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '*' | '_' | '`' | '^' | '~' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn write_block_comment(out: &mut String, pad: &str, id: &uuid::Uuid, style: Option<&str>) {
    out.push_str(pad);
    if let Some(s) = style {
        if s == "Body" || s.is_empty() {
            // default style — omit from comment (clean output)
            out.push_str(&format!("<!-- block:{} -->\n", id));
        } else {
            out.push_str(&format!("<!-- block:{} style=\"{}\" -->\n", id, s));
        }
    } else {
        out.push_str(&format!("<!-- block:{} -->\n", id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;
    use uuid::Uuid;

    fn simple_model() -> DocumentModel {
        use crate::text_engine::buffer::RunBuffer;
        DocumentModel {
            path: std::path::PathBuf::new(),
            frontmatter: Frontmatter {
                title: "Test Doc".to_string(),
                ..Default::default()
            },
            blocks: vec![Block::Paragraph {
                id: Uuid::parse_str("a3f2b1c4-e5d6-47a8-b9c0-d1e2f3a4b5c6").unwrap(),
                style: "Body".to_string(),
                content: RunBuffer::from_inlines(vec![Inline::Text("Hello world".to_string())]),
            }],
            fvm_version: "1.0".to_string(),
        }
    }

    #[test]
    fn serialise_paragraph() {
        let model = simple_model();
        let out = serialise(&model).unwrap();
        assert!(out.starts_with("---\n"));
        assert!(out.contains("title: Test Doc"));
        assert!(out.contains("<!-- block:a3f2b1c4-e5d6-47a8-b9c0-d1e2f3a4b5c6 -->"));
        assert!(out.contains("Hello world"));
        assert!(out.ends_with('\n'));
    }

    #[test]
    fn heading_serialises_hashes() {
        use crate::text_engine::buffer::RunBuffer;
        let model = DocumentModel {
            blocks: vec![Block::Heading {
                id: Uuid::new_v4(),
                level: 2,
                style: "Heading2".to_string(),
                content: RunBuffer::from_inlines(vec![Inline::Text("My Heading".to_string())]),
            }],
            ..simple_model()
        };
        let out = serialise(&model).unwrap();
        assert!(out.contains("## My Heading"));
    }
}
