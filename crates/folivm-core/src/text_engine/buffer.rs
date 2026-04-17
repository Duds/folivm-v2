use std::ops::Range;
use serde::{Deserialize, Serialize};
use crate::model::Inline;

/// A segment of text with an optional named style.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Run {
    pub text: String,
    pub style: Option<String>,
}

impl Run {
    pub fn new(text: String) -> Self {
        Self { text, style: None }
    }

    pub fn with_style(text: String, style: String) -> Self {
        Self { text, style: Some(style) }
    }

    /// Length of the text in bytes.
    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Character count (not byte count).
    pub fn char_len(&self) -> usize {
        self.text.chars().count()
    }
}

/// A flattened, editable text representation that simplifies cursor management.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunBuffer {
    pub runs: Vec<Run>,
}

impl RunBuffer {
    pub fn new() -> Self {
        Self { runs: Vec::new() }
    }

    /// Create a RunBuffer from a vector of Runs.
    pub fn from_runs(runs: Vec<Run>) -> Self {
        Self { runs }
    }

    /// Total byte length of all text.
    pub fn len(&self) -> usize {
        self.runs.iter().map(|r| r.len()).sum()
    }

    /// Total character count.
    pub fn char_len(&self) -> usize {
        self.runs.iter().map(|r| r.char_len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the concatenated text of all runs.
    pub fn to_string(&self) -> String {
        self.runs.iter().map(|r| r.text.as_str()).collect()
    }

    /// Insert text at the given byte offset.
    /// If the offset falls within a styled run, the inserted text inherits that style.
    pub fn insert(&mut self, offset: usize, text: &str) {
        if text.is_empty() {
            return;
        }

        if self.runs.is_empty() {
            self.runs.push(Run::new(text.to_string()));
            return;
        }

        let (run_idx, run_offset) = self.find_run(offset);

        if run_idx == self.runs.len() {
            // Append to the end
            self.runs.push(Run::new(text.to_string()));
        } else {
            let run = &mut self.runs[run_idx];

            if run_offset == 0 {
                // Insert before this run, inherit its style
                let new_run = Run { text: text.to_string(), style: run.style.clone() };
                self.runs.insert(run_idx, new_run);
            } else if run_offset == run.len() {
                // Insert after this run
                let new_run = Run { text: text.to_string(), style: run.style.clone() };
                self.runs.insert(run_idx + 1, new_run);
            } else {
                // Split the run
                let before = run.text[..run_offset].to_string();
                let after = run.text[run_offset..].to_string();
                let style = run.style.clone();

                run.text = before;

                let new_run = Run { text: text.to_string(), style: style.clone() };
                let after_run = Run { text: after, style };

                self.runs.insert(run_idx + 1, new_run);
                self.runs.insert(run_idx + 2, after_run);
            }
        }

        self.normalize();
    }

    /// Delete the byte range from the buffer.
    pub fn delete(&mut self, range: Range<usize>) {
        if range.start >= range.end || range.end > self.len() {
            return;
        }

        let (start_idx, start_offset) = self.find_run(range.start);
        let (end_idx, end_offset) = self.find_run(range.end);

        if start_idx == end_idx {
            // Deletion within a single run
            let run = &mut self.runs[start_idx];
            let mut text = run.text.clone();
            text.drain(start_offset..end_offset);
            run.text = text;

            if run.is_empty() {
                self.runs.remove(start_idx);
            }
        } else {
            // Deletion spans multiple runs
            // Keep the portion of the first run before start_offset
            self.runs[start_idx].text.truncate(start_offset);

            // Remove intermediate runs and the beginning of the final run
            for _ in start_idx + 1..=end_idx {
                if start_idx + 1 < self.runs.len() {
                    self.runs.remove(start_idx + 1);
                }
            }

            // Append the portion of the final run after end_offset
            if end_idx < self.runs.len() {
                let remaining = self.runs[end_idx].text[end_offset..].to_string();
                self.runs[start_idx].text.push_str(&remaining);
            }
        }

        // Remove empty runs
        self.runs.retain(|r| !r.is_empty());
        self.normalize();
    }

    /// Apply a style to text in the given byte range.
    pub fn apply_style(&mut self, range: Range<usize>, style: String) {
        if range.start >= range.end || range.end > self.len() {
            return;
        }

        let (start_idx, start_offset) = self.find_run(range.start);
        let (mut end_idx, end_offset) = self.find_run(range.end);

        // Split at the end boundary first (to preserve the start indices)
        if end_offset > 0 && end_idx < self.runs.len() {
            self.split_run_at(end_idx, end_offset);
        }

        // Split at the start boundary
        if start_offset > 0 && start_idx < self.runs.len() {
            self.split_run_at(start_idx, start_offset);
            end_idx += 1; // Adjust end_idx after inserting a new run
        }

        // Apply style to all runs in the range
        let start_apply = if start_offset > 0 { start_idx + 1 } else { start_idx };
        let end_apply = end_idx;

        for i in start_apply..=end_apply {
            if i < self.runs.len() {
                self.runs[i].style = Some(style.clone());
            }
        }

        self.normalize();
    }

    /// Find the run index and offset within that run for a given byte position.
    fn find_run(&self, offset: usize) -> (usize, usize) {
        let mut pos = 0;
        for (i, run) in self.runs.iter().enumerate() {
            if pos + run.len() >= offset {
                return (i, offset - pos);
            }
            pos += run.len();
        }
        (self.runs.len(), 0)
    }

    /// Split a run at the given byte offset.
    fn split_run_at(&mut self, run_idx: usize, offset: usize) {
        if run_idx >= self.runs.len() || offset >= self.runs[run_idx].len() {
            return;
        }

        let run = self.runs[run_idx].clone();
        let before = run.text[..offset].to_string();
        let after = run.text[offset..].to_string();

        self.runs[run_idx].text = before;
        self.runs.insert(run_idx + 1, Run { text: after, style: run.style });
    }

    /// Merge consecutive runs with the same style.
    fn normalize(&mut self) {
        let mut i = 0;
        while i + 1 < self.runs.len() {
            if self.runs[i].style == self.runs[i + 1].style {
                let next_text = self.runs[i + 1].text.clone();
                self.runs[i].text.push_str(&next_text);
                self.runs.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    /// Convert from a flat Vec<Inline> tree into a RunBuffer.
    /// Flattens the inline hierarchy into a single sequence of runs.
    pub fn from_inlines(inlines: Vec<Inline>) -> Self {
        let mut runs = Vec::new();
        Self::flatten_inlines(&inlines, None, &mut runs);
        let mut buf = Self::from_runs(runs);
        buf.normalize();
        buf
    }

    /// Flatten the inline tree, accumulating runs in the provided vector.
    /// Current style is passed down through the recursion.
    fn flatten_inlines(inlines: &[Inline], current_style: Option<String>, runs: &mut Vec<Run>) {
        for inline in inlines {
            match inline {
                Inline::Text(text) => {
                    runs.push(Run { text: text.clone(), style: current_style.clone() });
                }
                Inline::Styled { style, inlines } => {
                    // For nested styles, we use the most specific (innermost) style.
                    Self::flatten_inlines(inlines, Some(style.clone()), runs);
                }
                Inline::Token { path } => {
                    // Tokens are represented as special text markers with a token style
                    let token_text = format!("{{{}}}", path);
                    let token_style = current_style.clone().or_else(|| Some("token".to_string()));
                    runs.push(Run {
                        text: token_text,
                        style: token_style
                    });
                }
            }
        }
    }

    /// Convert RunBuffer back to Vec<Inline>.
    /// Consecutive runs with the same style are merged into Styled nodes.
    pub fn to_inlines(&self) -> Vec<Inline> {
        let mut inlines = Vec::new();

        for run in &self.runs {
            if let Some(style) = &run.style {
                inlines.push(Inline::Styled {
                    style: style.clone(),
                    inlines: vec![Inline::Text(run.text.clone())],
                });
            } else {
                inlines.push(Inline::Text(run.text.clone()));
            }
        }

        // Merge consecutive Styled nodes with the same style
        let mut merged = Vec::new();
        for inline in inlines {
            if let Some(Inline::Styled { style: last_style, inlines: last_inlines }) = merged.last_mut() {
                if let Inline::Styled { style, inlines } = &inline {
                    if style == last_style {
                        // Merge the text content
                        if let Inline::Text(text) = last_inlines.first().unwrap_or(&Inline::Text(String::new())) {
                            if let Inline::Text(new_text) = inlines.first().unwrap_or(&Inline::Text(String::new())) {
                                let mut merged_text = text.clone();
                                merged_text.push_str(new_text);
                                *last_inlines = vec![Inline::Text(merged_text)];
                            }
                        }
                        continue;
                    }
                }
            }
            merged.push(inline);
        }

        merged
    }
}

impl Default for RunBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_into_empty() {
        let mut buf = RunBuffer::new();
        buf.insert(0, "hello");
        assert_eq!(buf.to_string(), "hello");
        assert_eq!(buf.len(), 5);
    }

    #[test]
    fn test_insert_at_end() {
        let mut buf = RunBuffer::new();
        buf.insert(0, "hello");
        buf.insert(5, " world");
        assert_eq!(buf.to_string(), "hello world");
    }

    #[test]
    fn test_insert_in_middle() {
        let mut buf = RunBuffer::new();
        buf.insert(0, "heo");
        buf.insert(2, "ll");
        assert_eq!(buf.to_string(), "hello");
    }

    #[test]
    fn test_delete_range() {
        let mut buf = RunBuffer::new();
        buf.insert(0, "hello world");
        buf.delete(5..11);
        assert_eq!(buf.to_string(), "hello");
    }

    #[test]
    fn test_apply_style() {
        let mut buf = RunBuffer::new();
        buf.insert(0, "hello");
        buf.apply_style(0..5, "bold".to_string());

        assert_eq!(buf.runs.len(), 1);
        assert_eq!(buf.runs[0].style, Some("bold".to_string()));
    }

    #[test]
    fn test_apply_style_to_part() {
        let mut buf = RunBuffer::new();
        buf.insert(0, "hello");
        buf.apply_style(0..2, "bold".to_string());

        assert_eq!(buf.runs.len(), 2);
        assert_eq!(buf.runs[0].text, "he");
        assert_eq!(buf.runs[0].style, Some("bold".to_string()));
        assert_eq!(buf.runs[1].text, "llo");
        assert_eq!(buf.runs[1].style, None);
    }

    #[test]
    fn test_normalize_consecutive_runs() {
        let runs = vec![
            Run::with_style("hello".to_string(), "bold".to_string()),
            Run::with_style(" world".to_string(), "bold".to_string()),
        ];

        let mut buf = RunBuffer::from_runs(runs);
        buf.normalize();

        assert_eq!(buf.runs.len(), 1);
        assert_eq!(buf.to_string(), "hello world");
    }

    #[test]
    fn test_char_len() {
        let mut buf = RunBuffer::new();
        buf.insert(0, "hello");
        assert_eq!(buf.char_len(), 5);
    }

    #[test]
    fn test_from_inlines_plain_text() {
        let inlines = vec![Inline::Text("hello world".to_string())];
        let buf = RunBuffer::from_inlines(inlines);

        assert_eq!(buf.to_string(), "hello world");
        assert_eq!(buf.runs.len(), 1);
        assert_eq!(buf.runs[0].style, None);
    }

    #[test]
    fn test_from_inlines_styled() {
        let inlines = vec![
            Inline::Text("hello ".to_string()),
            Inline::Styled {
                style: "bold".to_string(),
                inlines: vec![Inline::Text("world".to_string())],
            },
        ];
        let buf = RunBuffer::from_inlines(inlines);

        assert_eq!(buf.to_string(), "hello world");
        assert_eq!(buf.runs.len(), 2);
        assert_eq!(buf.runs[0].text, "hello ");
        assert_eq!(buf.runs[0].style, None);
        assert_eq!(buf.runs[1].text, "world");
        assert_eq!(buf.runs[1].style, Some("bold".to_string()));
    }

    #[test]
    fn test_roundtrip_inlines() {
        let inlines = vec![
            Inline::Text("hello ".to_string()),
            Inline::Styled {
                style: "bold".to_string(),
                inlines: vec![Inline::Text("world".to_string())],
            },
        ];
        let buf = RunBuffer::from_inlines(inlines.clone());
        let result = buf.to_inlines();

        assert_eq!(buf.to_string(), "hello world");
        // Check that the result reconstructs the original structure
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_from_inlines_nested_styled() {
        let inlines = vec![Inline::Styled {
            style: "italic".to_string(),
            inlines: vec![
                Inline::Text("hello ".to_string()),
                Inline::Styled {
                    style: "bold".to_string(),
                    inlines: vec![Inline::Text("world".to_string())],
                },
            ],
        }];
        let buf = RunBuffer::from_inlines(inlines);

        assert_eq!(buf.to_string(), "hello world");
        // Nested styles should use the most specific (innermost) style
        assert_eq!(buf.runs.len(), 2);
        assert_eq!(buf.runs[0].style, Some("italic".to_string()));
        assert_eq!(buf.runs[1].style, Some("bold".to_string()));
    }
}
