
//! Parser for the `.fvm` document format.
//!
//! Entry point: [`parse`] — converts a raw `.fvm` string into a [`DocumentModel`].
//!
//! Architecture:
//! - [`frontmatter`] — YAML frontmatter via serde_yaml
//! - [`body`] — line-level block tokeniser
//! - [`cell`] — cell fence + YAML metadata parser
//! - [`inline`] — inline content (Markdown shorthand, named spans, tokens)
//! - [`serializer`] — canonical `DocumentModel → .fvm` string

pub mod body;
pub mod cell;
pub mod frontmatter;
pub mod inline;
pub mod serializer;

#[cfg(test)]
mod tests;

use crate::model::DocumentModel;
use anyhow::{Context, Result};

/// Parse a `.fvm` string into a [`DocumentModel`].
///
/// The path stored in the model is left empty — callers should set `model.path`
/// after parsing from disk.
pub fn parse(input: &str) -> Result<DocumentModel> {
    // Normalise line endings: CRLF → LF
    let input = input.replace("\r\n", "\n");

    // Split frontmatter from body.
    // The file must start with "---\n". The closing "---\n" ends the frontmatter.
    let (fm_yaml, body) = split_frontmatter(&input)
        .context("document must begin with a YAML frontmatter block (--- ... ---)")?;

    let frontmatter = frontmatter::parse(fm_yaml)?;
    let blocks = body::parse_blocks(body)?;

    Ok(DocumentModel {
        path: std::path::PathBuf::new(),
        frontmatter,
        blocks,
        fvm_version: "1.0".to_string(),
    })
}

/// Splits a `.fvm` string into (frontmatter_yaml, body).
///
/// Returns `None` if the file does not begin with `---\n` or has no closing `---`.
fn split_frontmatter(input: &str) -> Option<(&str, &str)> {
    let input = input.strip_prefix("---\n")?;
    // Find the closing fence
    let close = input.find("\n---\n").or_else(|| {
        // Handle "---" at end of file (no trailing body)
        if input.ends_with("\n---") {
            Some(input.len() - 4)
        } else {
            None
        }
    })?;
    let fm = &input[..close + 1]; // include the trailing newline
    let body_start = close + "\n---\n".len();
    let body = if body_start <= input.len() {
        &input[body_start..]
    } else {
        ""
    };
    Some((fm, body))
}

/// Serialise a [`DocumentModel`] to a canonical `.fvm` string.
///
/// This is a convenience re-export of [`serializer::serialise`].
pub fn serialise(model: &DocumentModel) -> Result<String> {
    serializer::serialise(model)
}

#[cfg(test)]
mod split_tests {
    use super::*;

    #[test]
    fn split_basic() {
        let input = "---\ntitle: Test\n---\nBody here\n";
        let (fm, body) = split_frontmatter(input).unwrap();
        assert_eq!(fm, "title: Test\n");
        assert_eq!(body, "Body here\n");
    }

    #[test]
    fn split_requires_leading_fence() {
        assert!(split_frontmatter("no frontmatter").is_none());
    }
}
