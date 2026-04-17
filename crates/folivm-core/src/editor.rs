//! Document editing operations.
//!
//! Provides functionality to apply EditOperations to a DocumentModel,
//! with proper error handling and validation of invariants.

use crate::model::{Block, BlockId, DocumentModel, EditOperation};
use crate::text_engine::buffer::RunBuffer;
use anyhow::{anyhow, Result};
use uuid::Uuid;

/// Apply an edit operation to the document.
///
/// Maintains the invariant: `parse(serialise(model)) == model`
pub fn apply_operation(model: &mut DocumentModel, op: &EditOperation) -> Result<()> {
    match op {
        EditOperation::Insert { block_id, offset, text } => {
            apply_insert(model, *block_id, *offset, text)
        }
        EditOperation::Delete { block_id, start, end } => {
            apply_delete(model, *block_id, *start, *end)
        }
        EditOperation::Split { block_id, offset } => {
            apply_split(model, *block_id, *offset)
        }
        EditOperation::Merge { block_a, block_b } => {
            apply_merge(model, *block_a, *block_b)
        }
        EditOperation::SetBlock { block_id, style } => {
            apply_set_block(model, *block_id, style)
        }
        EditOperation::SetInline { block_id, start, end, style } => {
            apply_set_inline(model, *block_id, *start, *end, style.clone())
        }
        EditOperation::InsertBlock { after, block } => {
            apply_insert_block(model, *after, block.clone())
        }
        EditOperation::RemoveBlock { block_id } => {
            apply_remove_block(model, *block_id)
        }
    }
}

/// Insert text into a block at the given offset.
fn apply_insert(model: &mut DocumentModel, block_id: BlockId, offset: usize, text: &str) -> Result<()> {
    let block = find_block_mut(&mut model.blocks, block_id)
        .ok_or_else(|| anyhow!("Block not found: {:?}", block_id))?;

    match block {
        Block::Paragraph { content, .. } | Block::Heading { content, .. } => {
            content.insert(offset, text);
            Ok(())
        }
        _ => Err(anyhow!("Cannot insert text into non-text block")),
    }
}

/// Delete a range of text from a block.
fn apply_delete(
    model: &mut DocumentModel,
    block_id: BlockId,
    start: usize,
    end: usize,
) -> Result<()> {
    if start >= end {
        return Ok(()); // No-op for invalid ranges
    }

    let block = find_block_mut(&mut model.blocks, block_id)
        .ok_or_else(|| anyhow!("Block not found: {:?}", block_id))?;

    match block {
        Block::Paragraph { content, .. } | Block::Heading { content, .. } => {
            content.delete(start..end);
            Ok(())
        }
        _ => Err(anyhow!("Cannot delete text from non-text block")),
    }
}

/// Split a block at the given offset, creating a new block with the text after the offset.
fn apply_split(model: &mut DocumentModel, block_id: BlockId, offset: usize) -> Result<()> {
    // Find the block index
    let block_index = model
        .blocks
        .iter()
        .position(|b| b.id() == Some(block_id))
        .ok_or_else(|| anyhow!("Block not found: {:?}", block_id))?;

    // Extract block data before mutable borrow
    let (is_paragraph, text, style, level) = {
        let block = &model.blocks[block_index];
        match block {
            Block::Paragraph { content, style, .. } => {
                (true, content.to_string(), style.clone(), 0)
            }
            Block::Heading { level, style, content, .. } => {
                (false, content.to_string(), style.clone(), *level)
            }
            _ => return Err(anyhow!("Cannot split non-text block")),
        }
    };

    if offset >= text.len() {
        return Ok(()); // No-op if offset is beyond end
    }

    // Create the new content for the second block
    let after_text = text[offset..].to_string();
    let before_content = {
        let mut buf = RunBuffer::new();
        if offset > 0 {
            buf.insert(0, &text[..offset]);
        }
        buf
    };

    let inlines_after = crate::parser::inline::parse_inline(&after_text)?;
    let new_content = RunBuffer::from_inlines(inlines_after);

    if is_paragraph {
        // Update the current block
        model.blocks[block_index] = Block::Paragraph {
            id: block_id,
            style: style.clone(),
            content: before_content,
        };

        // Insert the new block after the current one
        let new_block = Block::Paragraph {
            id: Uuid::new_v4(),
            style: style,
            content: new_content,
        };

        model.blocks.insert(block_index + 1, new_block);
    } else {
        // Update current block (Heading)
        model.blocks[block_index] = Block::Heading {
            id: block_id,
            level,
            style: style.clone(),
            content: before_content,
        };

        // Insert new paragraph (not heading) after split
        let new_block = Block::Paragraph {
            id: Uuid::new_v4(),
            style: "Body".to_string(),
            content: new_content,
        };

        model.blocks.insert(block_index + 1, new_block);
    }

    Ok(())
}

/// Merge two adjacent blocks.
fn apply_merge(model: &mut DocumentModel, block_a: BlockId, block_b: BlockId) -> Result<()> {
    let idx_a = model
        .blocks
        .iter()
        .position(|b| b.id() == Some(block_a))
        .ok_or_else(|| anyhow!("Block A not found: {:?}", block_a))?;

    let idx_b = model
        .blocks
        .iter()
        .position(|b| b.id() == Some(block_b))
        .ok_or_else(|| anyhow!("Block B not found: {:?}", block_b))?;

    if idx_a >= idx_b {
        return Err(anyhow!("Block A must come before Block B"));
    }

    // Both blocks must be text blocks
    let (content_a, content_b) = match (&model.blocks[idx_a], &model.blocks[idx_b]) {
        (
            Block::Paragraph { content: ca, .. } | Block::Heading { content: ca, .. },
            Block::Paragraph { content: cb, .. } | Block::Heading { content: cb, .. },
        ) => (ca.clone(), cb.clone()),
        _ => return Err(anyhow!("Can only merge text blocks")),
    };

    // Append B's content to A
    let mut merged_content = content_a;
    let text_b = content_b.to_string();
    merged_content.insert(merged_content.len(), &text_b);

    // Update block A with merged content
    match &mut model.blocks[idx_a] {
        Block::Paragraph { content, .. } => {
            *content = merged_content;
        }
        Block::Heading { content, .. } => {
            *content = merged_content;
        }
        _ => unreachable!(),
    }

    // Remove block B
    model.blocks.remove(idx_b);
    Ok(())
}

/// Change the style of a block.
fn apply_set_block(model: &mut DocumentModel, block_id: BlockId, style: &str) -> Result<()> {
    let block = find_block_mut(&mut model.blocks, block_id)
        .ok_or_else(|| anyhow!("Block not found: {:?}", block_id))?;

    match block {
        Block::Paragraph { style: s, .. }
        | Block::Heading { style: s, .. }
        | Block::List { style: s, .. }
        | Block::Blockquote { style: s, .. }
        | Block::Table { style: s, .. }
        | Block::Callout { style: s, .. } => {
            *s = style.to_string();
            Ok(())
        }
        Block::Section { .. } | Block::Cell { .. } => {
            Err(anyhow!("Cannot set style on Section or Cell blocks"))
        }
    }
}

/// Apply an inline style to a range of text in a block.
fn apply_set_inline(
    model: &mut DocumentModel,
    block_id: BlockId,
    start: usize,
    end: usize,
    style: Option<String>,
) -> Result<()> {
    let block = find_block_mut(&mut model.blocks, block_id)
        .ok_or_else(|| anyhow!("Block not found: {:?}", block_id))?;

    match block {
        Block::Paragraph { content, .. } | Block::Heading { content, .. } => {
            if let Some(s) = style {
                content.apply_style(start..end, s);
            }
            Ok(())
        }
        _ => Err(anyhow!("Cannot apply inline style to non-text block")),
    }
}

/// Insert a new block after the given block.
fn apply_insert_block(
    model: &mut DocumentModel,
    after: Option<BlockId>,
    block: Block,
) -> Result<()> {
    let insert_pos = if let Some(after_id) = after {
        model
            .blocks
            .iter()
            .position(|b| b.id() == Some(after_id))
            .ok_or_else(|| anyhow!("After block not found: {:?}", after_id))?
            + 1
    } else {
        0 // Insert at beginning if no "after" specified
    };

    model.blocks.insert(insert_pos, block);
    Ok(())
}

/// Remove a block from the document.
fn apply_remove_block(model: &mut DocumentModel, block_id: BlockId) -> Result<()> {
    let idx = model
        .blocks
        .iter()
        .position(|b| b.id() == Some(block_id))
        .ok_or_else(|| anyhow!("Block not found: {:?}", block_id))?;

    model.blocks.remove(idx);
    Ok(())
}

/// Find a mutable reference to a block by ID.
fn find_block_mut(blocks: &mut [Block], block_id: BlockId) -> Option<&mut Block> {
    blocks.iter_mut().find(|b| b.id() == Some(block_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Frontmatter;
    use std::path::PathBuf;

    fn test_model() -> DocumentModel {
        DocumentModel {
            path: PathBuf::new(),
            frontmatter: Frontmatter::default(),
            blocks: vec![Block::Paragraph {
                id: Uuid::nil(),
                style: "Body".to_string(),
                content: RunBuffer::from_inlines(vec![crate::model::Inline::Text("hello".to_string())]),
            }],
            fvm_version: "1.0".to_string(),
        }
    }

    #[test]
    fn test_apply_insert() {
        let mut model = test_model();
        let block_id = Uuid::nil();

        let op = EditOperation::Insert {
            block_id,
            offset: 5,
            text: " world".to_string(),
        };

        assert!(apply_operation(&mut model, &op).is_ok());

        if let Block::Paragraph { content, .. } = &model.blocks[0] {
            assert_eq!(content.to_string(), "hello world");
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_apply_delete() {
        let mut model = test_model();
        let block_id = Uuid::nil();

        let op = EditOperation::Delete {
            block_id,
            start: 1,
            end: 4,
        };

        assert!(apply_operation(&mut model, &op).is_ok());

        if let Block::Paragraph { content, .. } = &model.blocks[0] {
            assert_eq!(content.to_string(), "ho");
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_apply_insert_into_nonexistent_block() {
        let mut model = test_model();

        let op = EditOperation::Insert {
            block_id: Uuid::new_v4(),
            offset: 0,
            text: "text".to_string(),
        };

        assert!(apply_operation(&mut model, &op).is_err());
    }

    #[test]
    fn test_apply_set_block() {
        let mut model = test_model();
        let block_id = Uuid::nil();

        let op = EditOperation::SetBlock {
            block_id,
            style: "Emphasis".to_string(),
        };

        assert!(apply_operation(&mut model, &op).is_ok());

        if let Block::Paragraph { style, .. } = &model.blocks[0] {
            assert_eq!(style, "Emphasis");
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_apply_insert_block() {
        let mut model = test_model();
        let block_id = Uuid::nil();

        let new_block = Block::Paragraph {
            id: Uuid::new_v4(),
            style: "Body".to_string(),
            content: RunBuffer::from_inlines(vec![crate::model::Inline::Text("new".to_string())]),
        };

        let op = EditOperation::InsertBlock {
            after: Some(block_id),
            block: new_block,
        };

        assert!(apply_operation(&mut model, &op).is_ok());
        assert_eq!(model.blocks.len(), 2);
    }

    #[test]
    fn test_apply_remove_block() {
        let mut model = test_model();
        let block_id = Uuid::nil();

        assert_eq!(model.blocks.len(), 1);

        let op = EditOperation::RemoveBlock { block_id };

        assert!(apply_operation(&mut model, &op).is_ok());
        assert_eq!(model.blocks.len(), 0);
    }

    #[test]
    fn test_apply_remove_nonexistent_block() {
        let mut model = test_model();

        let op = EditOperation::RemoveBlock {
            block_id: Uuid::new_v4(),
        };

        assert!(apply_operation(&mut model, &op).is_err());
    }

    #[test]
    fn test_apply_split_paragraph() {
        let mut model = test_model();
        let block_id = Uuid::nil();

        let op = EditOperation::Split {
            block_id,
            offset: 3,
        };

        assert!(apply_operation(&mut model, &op).is_ok());
        assert_eq!(model.blocks.len(), 2);

        // First block should contain "hel"
        if let Block::Paragraph { content, .. } = &model.blocks[0] {
            assert_eq!(content.to_string(), "hel");
        } else {
            panic!("Expected paragraph block");
        }

        // Second block should contain "lo"
        if let Block::Paragraph { content, .. } = &model.blocks[1] {
            assert_eq!(content.to_string(), "lo");
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_apply_split_at_end() {
        let mut model = test_model();
        let block_id = Uuid::nil();

        let op = EditOperation::Split {
            block_id,
            offset: 100, // Beyond end
        };

        assert!(apply_operation(&mut model, &op).is_ok());
        assert_eq!(model.blocks.len(), 1); // No-op for out of bounds
    }

    #[test]
    fn test_apply_set_inline() {
        let mut model = test_model();
        let block_id = Uuid::nil();

        let op = EditOperation::SetInline {
            block_id,
            start: 0,
            end: 5,
            style: Some("bold".to_string()),
        };

        assert!(apply_operation(&mut model, &op).is_ok());

        if let Block::Paragraph { content, .. } = &model.blocks[0] {
            // Verify style was applied (content should still be "hello")
            assert_eq!(content.to_string(), "hello");
            // Check that runs have the style
            assert!(!content.runs.is_empty());
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_apply_merge_paragraphs() {
        let block_id_a = Uuid::nil();
        let block_id_b = Uuid::new_v4();

        let mut model = DocumentModel {
            path: std::path::PathBuf::new(),
            frontmatter: Frontmatter::default(),
            blocks: vec![
                Block::Paragraph {
                    id: block_id_a,
                    style: "Body".to_string(),
                    content: RunBuffer::from_inlines(vec![crate::model::Inline::Text(
                        "hello".to_string(),
                    )]),
                },
                Block::Paragraph {
                    id: block_id_b,
                    style: "Body".to_string(),
                    content: RunBuffer::from_inlines(vec![crate::model::Inline::Text(
                        "world".to_string(),
                    )]),
                },
            ],
            fvm_version: "1.0".to_string(),
        };

        let op = EditOperation::Merge {
            block_a: block_id_a,
            block_b: block_id_b,
        };

        assert!(apply_operation(&mut model, &op).is_ok());
        assert_eq!(model.blocks.len(), 1);

        if let Block::Paragraph { content, .. } = &model.blocks[0] {
            assert_eq!(content.to_string(), "helloworld");
        } else {
            panic!("Expected paragraph block");
        }
    }

    #[test]
    fn test_apply_merge_wrong_order() {
        let block_id_a = Uuid::nil();
        let block_id_b = Uuid::new_v4();

        let mut model = DocumentModel {
            path: std::path::PathBuf::new(),
            frontmatter: Frontmatter::default(),
            blocks: vec![
                Block::Paragraph {
                    id: block_id_a,
                    style: "Body".to_string(),
                    content: RunBuffer::from_inlines(vec![crate::model::Inline::Text(
                        "hello".to_string(),
                    )]),
                },
                Block::Paragraph {
                    id: block_id_b,
                    style: "Body".to_string(),
                    content: RunBuffer::from_inlines(vec![crate::model::Inline::Text(
                        "world".to_string(),
                    )]),
                },
            ],
            fvm_version: "1.0".to_string(),
        };

        // Try to merge B into A (wrong order)
        let op = EditOperation::Merge {
            block_a: block_id_b,
            block_b: block_id_a,
        };

        assert!(apply_operation(&mut model, &op).is_err());
    }

    #[test]
    fn test_apply_multiple_operations() {
        let mut model = test_model();
        let block_id = Uuid::nil();

        // Insert
        assert!(apply_operation(
            &mut model,
            &EditOperation::Insert {
                block_id,
                offset: 5,
                text: " there".to_string(),
            }
        )
        .is_ok());

        // Delete
        assert!(apply_operation(
            &mut model,
            &EditOperation::Delete {
                block_id,
                start: 0,
                end: 1,
            }
        )
        .is_ok());

        // Set style
        assert!(apply_operation(
            &mut model,
            &EditOperation::SetBlock {
                block_id,
                style: "Emphasis".to_string(),
            }
        )
        .is_ok());

        if let Block::Paragraph { content, style, .. } = &model.blocks[0] {
            assert_eq!(content.to_string(), "ello there");
            assert_eq!(style, "Emphasis");
        } else {
            panic!("Expected paragraph block");
        }
    }
}
