
//! Inline content parser for `.fvm` documents.
//!
//! Parses a string of inline content and produces a `Vec<Inline>`.
//!
//! Supported syntax:
//! - Plain text (escaped `\*`, `\_`, etc.)
//! - `*text*` / `_text_` → `Inline::Styled { style: "Emphasis", … }`
//! - `**text**` / `__text__` → `Inline::Styled { style: "Strong", … }`
//! - `` `text` `` → `Inline::Styled { style: "Code", … }`
//! - `^text^` → `Inline::Styled { style: "Superscript", … }`
//! - `~text~` → `Inline::Styled { style: "Subscript", … }`
//! - `[content]{.StyleName}` → `Inline::Styled { style: "StyleName", … }`
//! - `{token.path}` → `Inline::Token { path: "token.path" }`

use crate::model::Inline;
use anyhow::Result;

/// Parse inline content from a string slice.
pub fn parse_inline(input: &str) -> Result<Vec<Inline>> {
    let mut parser = InlineParser::new(input);
    parser.parse()
}

// ---------------------------------------------------------------------------
// Inline parser
// ---------------------------------------------------------------------------

struct InlineParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> InlineParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn remaining(&self) -> &str {
        &self.input[self.pos..]
    }

    fn is_done(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn advance_by(&mut self, n: usize) {
        self.pos = (self.pos + n).min(self.input.len());
    }

    fn parse(&mut self) -> Result<Vec<Inline>> {
        let mut result = Vec::new();
        let mut text_buf = String::new();

        while !self.is_done() {
            // Escape sequences
            if self.remaining().starts_with('\\') && self.remaining().len() > 1 {
                self.advance_by(1);
                let ch = self.remaining().chars().next().unwrap();
                text_buf.push(ch);
                self.advance_by(ch.len_utf8());
                continue;
            }

            // Token: {path}
            if self.remaining().starts_with('{') {
                if let Some(close) = self.remaining().find('}') {
                    let rem = self.remaining();
                    let path = &rem[1..close];
                    // Validate token path: dotted identifiers
                    if !path.is_empty() && path.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_') {
                        flush_text(&mut text_buf, &mut result);
                        result.push(Inline::Token { path: path.to_string() });
                        self.advance_by(close + 1);
                        continue;
                    }
                }
            }

            // Named span: [content]{.StyleName}
            if self.remaining().starts_with('[') {
                if let Some(inline) = self.try_parse_named_span() {
                    flush_text(&mut text_buf, &mut result);
                    result.push(inline);
                    continue;
                }
            }

            // Strong: **text** or __text__
            if self.remaining().starts_with("**") || self.remaining().starts_with("__") {
                let marker = self.remaining()[..2].to_string();
                if let Some(inline) = self.try_parse_delimited(&marker, &marker, "Strong") {
                    flush_text(&mut text_buf, &mut result);
                    result.push(inline);
                    continue;
                }
            }

            // Emphasis: *text* or _text_
            if self.remaining().starts_with('*') || self.remaining().starts_with('_') {
                let marker = self.remaining()[..1].to_string();
                // Make sure it's not ** or __
                let next = self.remaining().get(1..2);
                let is_double = next == Some(&marker);
                if !is_double {
                    if let Some(inline) = self.try_parse_delimited(&marker, &marker, "Emphasis") {
                        flush_text(&mut text_buf, &mut result);
                        result.push(inline);
                        continue;
                    }
                }
            }

            // Code: `text`
            if self.remaining().starts_with('`') {
                if let Some(inline) = self.try_parse_delimited("`", "`", "Code") {
                    flush_text(&mut text_buf, &mut result);
                    result.push(inline);
                    continue;
                }
            }

            // Superscript: ^text^
            if self.remaining().starts_with('^') {
                if let Some(inline) = self.try_parse_delimited("^", "^", "Superscript") {
                    flush_text(&mut text_buf, &mut result);
                    result.push(inline);
                    continue;
                }
            }

            // Subscript: ~text~
            if self.remaining().starts_with('~') {
                if let Some(inline) = self.try_parse_delimited("~", "~", "Subscript") {
                    flush_text(&mut text_buf, &mut result);
                    result.push(inline);
                    continue;
                }
            }

            // Plain text character
            let ch = self.peek_char().unwrap();
            text_buf.push(ch);
            self.advance_by(ch.len_utf8());
        }

        flush_text(&mut text_buf, &mut result);
        Ok(result)
    }

    /// Try to parse a delimited span: `open_marker ... close_marker`.
    /// Returns `None` on failure, leaving `self.pos` unchanged.
    fn try_parse_delimited(&mut self, open: &str, close: &str, style: &str) -> Option<Inline> {
        let rem = self.remaining();
        if !rem.starts_with(open) {
            return None;
        }
        let after_open = &rem[open.len()..];
        // Find closing marker (must not be empty between markers)
        let close_pos = after_open.find(close)?;
        if close_pos == 0 {
            return None; // empty span, treat as literal
        }
        let inner_text = after_open[..close_pos].to_string();
        let total_len = open.len() + close_pos + close.len();

        let inner_inlines = parse_inline(&inner_text).ok()?;
        self.advance_by(total_len);
        Some(Inline::Styled { style: style.to_string(), inlines: inner_inlines })
    }

    /// Try to parse `[content]{.StyleName}`.
    fn try_parse_named_span(&mut self) -> Option<Inline> {
        let rem = self.remaining();
        // Find the closing ]
        let bracket_close = rem.find("]{")?;
        if !rem.get(bracket_close + 2..)?.starts_with('.') {
            return None;
        }
        let after_dot = &rem[bracket_close + 3..]; // skip ].
        let brace_close = after_dot.find('}')?;
        let style_name = after_dot[..brace_close].to_string();

        // Validate style name: letters digits hyphens only
        if style_name.is_empty()
            || !style_name.chars().all(|c| c.is_alphanumeric() || c == '-')
        {
            return None;
        }

        let inner_text = rem[1..bracket_close].to_string();
        let total_len = 1 + bracket_close + 2 + 1 + brace_close + 1; // [content]{.Style}

        let inner_inlines = parse_inline(&inner_text).ok()?;
        self.advance_by(total_len);
        Some(Inline::Styled { style: style_name, inlines: inner_inlines })
    }
}

fn flush_text(buf: &mut String, out: &mut Vec<Inline>) {
    if !buf.is_empty() {
        out.push(Inline::Text(std::mem::take(buf)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text() {
        let result = parse_inline("hello world").unwrap();
        assert_eq!(result, vec![Inline::Text("hello world".to_string())]);
    }

    #[test]
    fn emphasis() {
        let result = parse_inline("do *this* now").unwrap();
        assert_eq!(
            result,
            vec![
                Inline::Text("do ".to_string()),
                Inline::Styled { style: "Emphasis".to_string(), inlines: vec![Inline::Text("this".to_string())] },
                Inline::Text(" now".to_string()),
            ]
        );
    }

    #[test]
    fn strong() {
        let result = parse_inline("**bold**").unwrap();
        assert!(matches!(&result[0], Inline::Styled { style, .. } if style == "Strong"));
    }

    #[test]
    fn code_span() {
        let result = parse_inline("`code`").unwrap();
        assert!(matches!(&result[0], Inline::Styled { style, .. } if style == "Code"));
    }

    #[test]
    fn named_span() {
        let result = parse_inline("[force majeure]{.DefinedTerm}").unwrap();
        assert!(matches!(&result[0], Inline::Styled { style, .. } if style == "DefinedTerm"));
    }

    #[test]
    fn token() {
        let result = parse_inline("Today is {date}.").unwrap();
        assert_eq!(result[1], Inline::Token { path: "date".to_string() });
    }

    #[test]
    fn nested_token() {
        let result = parse_inline("{crm.client.name}").unwrap();
        assert_eq!(result[0], Inline::Token { path: "crm.client.name".to_string() });
    }

    #[test]
    fn escape_sequence() {
        let result = parse_inline("literal \\* asterisk").unwrap();
        assert_eq!(result, vec![Inline::Text("literal * asterisk".to_string())]);
    }
}
