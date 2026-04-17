
//! YAML frontmatter parser.
//!
//! Parses the YAML section between the two `---` fences into a [`Frontmatter`] struct.
//! Unknown keys are preserved in the `extra` field so that extension-defined keys
//! survive round-trips.

use crate::model::{Frontmatter, Margins, HeaderFooter};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_yaml::Value;
use std::collections::BTreeMap;

/// Raw YAML representation used for deserialization. Mirrors [`Frontmatter`] with
/// additional flexible fields.
#[derive(Debug, Default, Deserialize)]
struct RawFrontmatter {
    // --- Required ---
    pub title: Option<String>,

    // --- Identity ---
    pub author: Option<String>,
    pub date: Option<String>,
    pub lang: Option<String>,
    pub version: Option<String>,
    pub tags: Option<Vec<String>>,

    // --- Page geometry ---
    pub page_size: Option<String>,
    pub page_width: Option<f64>,
    pub page_height: Option<f64>,
    pub orientation: Option<String>,

    // --- Margins ---
    pub margins: Option<Margins>,

    // --- Theme ---
    pub theme: Option<String>,

    // --- Header / footer ---
    pub header: Option<HeaderFooter>,
    pub footer: Option<HeaderFooter>,

    // All other keys are captured in extra (serde flattening via BTreeMap).
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}


/// Parse YAML frontmatter string into a [`Frontmatter`].
pub fn parse(yaml: &str) -> Result<Frontmatter> {
    let raw: RawFrontmatter =
        serde_yaml::from_str(yaml).context("failed to parse YAML frontmatter")?;

    // Strip known keys out of extras so only truly unknown keys remain
    let known = [
        "title", "author", "date", "lang", "version", "tags",
        "page_size", "page_width", "page_height", "orientation",
        "margins", "theme", "header", "footer",
    ];
    let extra: BTreeMap<String, Value> = raw
        .extra
        .into_iter()
        .filter(|(k, _)| !known.contains(&k.as_str()))
        .collect();

    Ok(Frontmatter {
        title: raw.title.unwrap_or_default(),
        author: raw.author,
        date: raw.date,
        lang: raw.lang,
        version: raw.version,
        tags: raw.tags.unwrap_or_default(),
        page_size: raw.page_size,
        page_width: raw.page_width,
        page_height: raw.page_height,
        orientation: raw.orientation,
        margins: raw.margins,
        theme: raw.theme,
        header: raw.header,
        footer: raw.footer,
        extra,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal() {
        let yaml = "title: My Doc\n";
        let fm = parse(yaml).unwrap();
        assert_eq!(fm.title, "My Doc");
        assert!(fm.author.is_none());
    }

    #[test]
    fn parse_full() {
        let yaml = r#"title: "Service Agreement"
author: "Acme Legal"
date: "2026-03-13"
lang: "en-AU"
page_size: A4
theme: ./themes/corp.fvm-theme
crm:
  client_id: "C-123"
"#;
        let fm = parse(yaml).unwrap();
        assert_eq!(fm.title, "Service Agreement");
        assert_eq!(fm.page_size.as_deref(), Some("A4"));
        assert!(fm.extra.contains_key("crm"));
    }
}
