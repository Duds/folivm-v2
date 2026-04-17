
//! Cell fence + YAML-like metadata parser.
//!
//! Called by [`body::BlockParser`] when it encounters a `:::cell:type` fence.
//! Unknown cell types are stored verbatim in [`CellBlock`].

use crate::model::{BlockId, CellBlock};
use anyhow::Result;
use serde_json::{Map, Value};

/// Position-aware cell parser. Advances `pos` past the closing `:::`.
///
/// On entry: `lines[*pos]` is the first line *inside* the cell (i.e. after the
/// `:::cell:type` fence line which the caller already advanced past).
pub fn parse_cell(
    id: BlockId,
    cell_type: String,
    lines: &[&str],
    pos: &mut usize,
) -> Result<CellBlock> {
    let mut attrs: Map<String, Value> = Map::new();
    let mut body: Option<String> = None;

    // Phase 1: read key: value metadata lines until "---" or ":::"
    loop {
        let line = match lines.get(*pos) {
            Some(l) => *l,
            None => break,
        };

        if line == ":::" {
            *pos += 1;
            break;
        }
        if line == "---" {
            *pos += 1;
            // Phase 2: read body until ":::"
            let mut body_lines = Vec::new();
            loop {
                let l = match lines.get(*pos) {
                    Some(l) => *l,
                    None => break,
                };
                if l == ":::" {
                    *pos += 1;
                    break;
                }
                body_lines.push(l);
                *pos += 1;
            }
            if !body_lines.is_empty() {
                body = Some(body_lines.join("\n"));
            }
            break;
        }

        // Parse key: value line
        if let Some((k, v)) = parse_kv(line) {
            attrs.insert(k, Value::String(v));
        } else if line.starts_with("  ") || line.starts_with('\t') {
            // Indented continuation — attach to last key as multi-line value
            // For simplicity, append to last string value
            if let Some(last_val) = attrs.values_mut().last() {
                if let Value::String(s) = last_val {
                    s.push('\n');
                    s.push_str(line.trim());
                }
            }
        }
        // Skip unknown lines rather than erroring — extension forward-compat
        *pos += 1;
    }

    if let Some(body_text) = body {
        attrs.insert("_body".to_string(), Value::String(body_text));
    }

    Ok(CellBlock {
        id,
        cell_type,
        attrs: Value::Object(attrs),
    })
}

/// Parse a `key: value` line. Everything after the first `: ` is the value.
fn parse_kv(line: &str) -> Option<(String, String)> {
    let (k, v) = line.split_once(": ")?;
    let k = k.trim().to_string();
    // Strip surrounding quotes if present
    let v = v.trim();
    let v = if v.starts_with('"') && v.ends_with('"') && v.len() >= 2 {
        v[1..v.len() - 1].to_string()
    } else {
        v.to_string()
    };
    Some((k, v))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn parse_image_cell() {
        let lines = [
            "src: assets/photo.png",
            "alt: A nice photo",
            "width: 80",
            ":::",
        ];
        let mut pos = 0usize;
        let cell = parse_cell(Uuid::new_v4(), "cell:image".to_string(), &lines, &mut pos).unwrap();
        assert_eq!(cell.cell_type, "cell:image");
        let obj = cell.attrs.as_object().unwrap();
        assert_eq!(obj["src"].as_str(), Some("assets/photo.png"));
        assert_eq!(obj["alt"].as_str(), Some("A nice photo"));
        assert_eq!(pos, 4);
    }

    #[test]
    fn parse_math_cell_with_body() {
        let lines = [
            "syntax: latex",
            "display: block",
            "---",
            "E = mc^2",
            ":::",
        ];
        let mut pos = 0usize;
        let cell = parse_cell(Uuid::new_v4(), "cell:math".to_string(), &lines, &mut pos).unwrap();
        let obj = cell.attrs.as_object().unwrap();
        assert_eq!(obj["syntax"].as_str(), Some("latex"));
        assert_eq!(obj["_body"].as_str(), Some("E = mc^2"));
        assert_eq!(pos, 5);
    }

    #[test]
    fn unknown_cell_type_preserved() {
        let lines = [
            "custom_key: custom_value",
            ":::",
        ];
        let mut pos = 0usize;
        let cell = parse_cell(Uuid::new_v4(), "cell:unknown".to_string(), &lines, &mut pos).unwrap();
        assert_eq!(cell.cell_type, "cell:unknown");
    }
}
