pub mod buffer;

use crate::model::{BlockId, EditOperation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModelPosition {
    pub block_id: BlockId,
    /// Byte offset within the block's text content.
    pub offset: usize,
}

pub struct CursorManager {
    position: ModelPosition,
    /// Remembered horizontal position in pt for up/down movement across lines.
    preferred_x: f32,
}

impl CursorManager {
    pub fn new(position: ModelPosition) -> Self {
        Self { position, preferred_x: 0.0 }
    }

    pub fn move_to(&mut self, pos: ModelPosition) {
        self.position = pos;
    }

    pub fn position(&self) -> ModelPosition {
        self.position
    }

    pub fn preferred_x(&self) -> f32 {
        self.preferred_x
    }

    pub fn set_preferred_x(&mut self, x: f32) {
        self.preferred_x = x;
    }
}

pub struct SelectionManager {
    anchor: Option<ModelPosition>,
    focus: Option<ModelPosition>,
}

impl SelectionManager {
    pub fn new() -> Self {
        Self { anchor: None, focus: None }
    }

    pub fn set_anchor(&mut self, pos: ModelPosition) {
        self.anchor = Some(pos);
    }

    pub fn set_focus(&mut self, pos: ModelPosition) {
        self.focus = Some(pos);
    }

    pub fn clear(&mut self) {
        self.anchor = None;
        self.focus = None;
    }

    /// Returns `(anchor, focus)` in document order if a selection exists.
    pub fn range(&self) -> Option<(ModelPosition, ModelPosition)> {
        match (self.anchor, self.focus) {
            (Some(a), Some(f)) => {
                if a == f {
                    None // No selection if anchor and focus are the same
                } else {
                    Some((a, f))
                }
            }
            _ => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.range().is_none()
    }
}

pub struct UndoStack {
    undo: Vec<EditOperation>,
    redo: Vec<EditOperation>,
}

impl UndoStack {
    pub fn new() -> Self {
        Self { undo: Vec::new(), redo: Vec::new() }
    }

    /// Push a new operation to the undo stack.
    /// When a new operation is pushed, the redo stack is cleared.
    pub fn push(&mut self, op: EditOperation) {
        self.undo.push(op);
        self.redo.clear();
    }

    /// Pop an operation from the undo stack and push it to the redo stack.
    pub fn undo(&mut self) -> Option<EditOperation> {
        if let Some(op) = self.undo.pop() {
            self.redo.push(op.clone());
            Some(op)
        } else {
            None
        }
    }

    /// Pop an operation from the redo stack and push it to the undo stack.
    pub fn redo(&mut self) -> Option<EditOperation> {
        if let Some(op) = self.redo.pop() {
            self.undo.push(op.clone());
            Some(op)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
    }
}

pub struct InputHandler {
    cursor: CursorManager,
    selection: SelectionManager,
    undo: UndoStack,
}

impl InputHandler {
    pub fn new(initial_position: ModelPosition) -> Self {
        Self {
            cursor: CursorManager::new(initial_position),
            selection: SelectionManager::new(),
            undo: UndoStack::new(),
        }
    }

    /// Maps a raw keydown event to zero or more `EditOperation`s.
    /// `modifiers` is a bitmask: bit 0 = Shift, bit 1 = Ctrl/Cmd, bit 2 = Alt.
    pub fn handle_keydown(&mut self, key: &str, modifiers: u32) -> Vec<EditOperation> {
        let shift = (modifiers & 0x01) != 0;
        let ctrl_cmd = (modifiers & 0x02) != 0;
        let _alt = (modifiers & 0x04) != 0;

        let mut ops = Vec::new();

        match key {
            // Text insertion: regular character keys
            _ if key.len() == 1 && !key.chars().next().unwrap().is_control() => {
                let pos = self.cursor.position();
                // Delete selection if one exists
                if let Some((a, f)) = self.selection.range() {
                    ops.push(self.delete_selection(a, f));
                    self.selection.clear();
                }
                ops.push(EditOperation::Insert {
                    block_id: pos.block_id,
                    offset: pos.offset,
                    text: key.to_string(),
                });
                self.selection.clear();
            }

            // Backspace: delete character before cursor or selection
            "Backspace" => {
                let pos = self.cursor.position();
                if let Some((start, end)) = self.selection.range() {
                    // Delete selection
                    ops.push(self.delete_selection(start, end));
                    self.selection.clear();
                } else if pos.offset > 0 {
                    // Delete character before cursor
                    ops.push(EditOperation::Delete {
                        block_id: pos.block_id,
                        start: pos.offset.saturating_sub(1),
                        end: pos.offset,
                    });
                }
            }

            // Delete: delete character after cursor or selection
            "Delete" => {
                let pos = self.cursor.position();
                if let Some((start, end)) = self.selection.range() {
                    // Delete selection
                    ops.push(self.delete_selection(start, end));
                    self.selection.clear();
                } else {
                    // Delete character after cursor (assuming we have access to block length)
                    ops.push(EditOperation::Delete {
                        block_id: pos.block_id,
                        start: pos.offset,
                        end: pos.offset.saturating_add(1),
                    });
                }
            }

            // Arrow keys with shift for selection
            "ArrowLeft" => {
                let pos = self.cursor.position();
                if shift {
                    // Extend selection
                    if self.selection.anchor.is_none() {
                        self.selection.set_anchor(pos);
                    }
                    self.selection.set_focus(ModelPosition {
                        block_id: pos.block_id,
                        offset: pos.offset.saturating_sub(1),
                    });
                } else {
                    // Clear selection and move cursor
                    if self.selection.is_empty() {
                        self.cursor.move_to(ModelPosition {
                            block_id: pos.block_id,
                            offset: pos.offset.saturating_sub(1),
                        });
                    } else {
                        // Move to start of selection
                        if let Some((a, f)) = self.selection.range() {
                            self.cursor.move_to(if a < f { a } else { f });
                        }
                    }
                    self.selection.clear();
                }
            }

            "ArrowRight" => {
                let pos = self.cursor.position();
                if shift {
                    // Extend selection
                    if self.selection.anchor.is_none() {
                        self.selection.set_anchor(pos);
                    }
                    self.selection.set_focus(ModelPosition {
                        block_id: pos.block_id,
                        offset: pos.offset.saturating_add(1),
                    });
                } else {
                    // Clear selection and move cursor
                    if self.selection.is_empty() {
                        self.cursor.move_to(ModelPosition {
                            block_id: pos.block_id,
                            offset: pos.offset.saturating_add(1),
                        });
                    } else {
                        // Move to end of selection
                        if let Some((a, f)) = self.selection.range() {
                            self.cursor.move_to(if a > f { a } else { f });
                        }
                    }
                    self.selection.clear();
                }
            }

            // Undo/Redo
            "z" if ctrl_cmd => {
                if let Some(op) = self.undo.undo() {
                    ops.push(op);
                }
            }

            "z" if ctrl_cmd && shift => {
                if let Some(op) = self.undo.redo() {
                    ops.push(op);
                }
            }

            // Select all (Ctrl/Cmd+A)
            "a" if ctrl_cmd => {
                // Set anchor at start and focus at end (requires document access)
                // For now, just note this as a recognized pattern
            }

            _ => {} // Unrecognized key
        }

        ops
    }

    pub fn handle_paste_text(&mut self, text: &str) -> Vec<EditOperation> {
        let pos = self.cursor.position();
        let mut ops = Vec::new();

        // Delete selection if one exists
        if let Some((a, f)) = self.selection.range() {
            ops.push(self.delete_selection(a, f));
            self.selection.clear();
        }

        // Insert pasted text
        ops.push(EditOperation::Insert {
            block_id: pos.block_id,
            offset: pos.offset,
            text: text.to_string(),
        });

        ops
    }

    pub fn handle_paste_fvm(&mut self, fvm: &str) -> Vec<EditOperation> {
        let pos = self.cursor.position();
        let mut ops = Vec::new();

        // Delete selection if one exists
        if let Some((a, f)) = self.selection.range() {
            ops.push(self.delete_selection(a, f));
            self.selection.clear();
        }

        // Parse FVM and insert as structured content
        // For now, treat it as plain text insertion
        ops.push(EditOperation::Insert {
            block_id: pos.block_id,
            offset: pos.offset,
            text: fvm.to_string(),
        });

        ops
    }

    /// Helper to delete a selection range
    fn delete_selection(&self, start: ModelPosition, end: ModelPosition) -> EditOperation {
        if start.block_id == end.block_id {
            EditOperation::Delete {
                block_id: start.block_id,
                start: start.offset.min(end.offset),
                end: start.offset.max(end.offset),
            }
        } else {
            // Multi-block selection - for now just delete within first block
            EditOperation::Delete {
                block_id: start.block_id,
                start: start.offset,
                end: start.offset,
            }
        }
    }

    pub fn cursor(&self) -> &CursorManager {
        &self.cursor
    }

    pub fn cursor_mut(&mut self) -> &mut CursorManager {
        &mut self.cursor
    }

    pub fn selection(&self) -> &SelectionManager {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut SelectionManager {
        &mut self.selection
    }

    pub fn undo_stack(&self) -> &UndoStack {
        &self.undo
    }

    pub fn undo_stack_mut(&mut self) -> &mut UndoStack {
        &mut self.undo
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn test_position() -> ModelPosition {
        ModelPosition {
            block_id: Uuid::nil(),
            offset: 0,
        }
    }

    #[test]
    fn cursor_manager_move_to() {
        let mut cursor = CursorManager::new(test_position());
        let new_pos = ModelPosition {
            block_id: Uuid::nil(),
            offset: 42,
        };
        cursor.move_to(new_pos);
        assert_eq!(cursor.position().offset, 42);
    }

    #[test]
    fn cursor_manager_preferred_x() {
        let mut cursor = CursorManager::new(test_position());
        assert_eq!(cursor.preferred_x(), 0.0);
        cursor.set_preferred_x(3.14);
        assert_eq!(cursor.preferred_x(), 3.14);
    }

    #[test]
    fn selection_manager_empty() {
        let selection = SelectionManager::new();
        assert!(selection.is_empty());
        assert_eq!(selection.range(), None);
    }

    #[test]
    fn selection_manager_set_anchor_and_focus() {
        let mut selection = SelectionManager::new();
        let pos1 = test_position();
        let pos2 = ModelPosition {
            block_id: Uuid::nil(),
            offset: 10,
        };

        selection.set_anchor(pos1);
        selection.set_focus(pos2);
        assert!(!selection.is_empty());
        assert_eq!(selection.range(), Some((pos1, pos2)));
    }

    #[test]
    fn selection_manager_same_anchor_and_focus() {
        let mut selection = SelectionManager::new();
        let pos = test_position();
        selection.set_anchor(pos);
        selection.set_focus(pos);
        assert!(selection.is_empty());
        assert_eq!(selection.range(), None);
    }

    #[test]
    fn selection_manager_clear() {
        let mut selection = SelectionManager::new();
        selection.set_anchor(test_position());
        selection.set_focus(ModelPosition {
            block_id: Uuid::nil(),
            offset: 10,
        });
        selection.clear();
        assert!(selection.is_empty());
    }

    #[test]
    fn undo_stack_push_and_undo() {
        let mut stack = UndoStack::new();
        let op = EditOperation::Insert {
            block_id: Uuid::nil(),
            offset: 0,
            text: "hello".to_string(),
        };

        stack.push(op.clone());
        let undone = stack.undo();
        assert_eq!(undone, Some(op));
    }

    #[test]
    fn undo_stack_redo() {
        let mut stack = UndoStack::new();
        let op = EditOperation::Insert {
            block_id: Uuid::nil(),
            offset: 0,
            text: "hello".to_string(),
        };

        stack.push(op.clone());
        stack.undo();
        let redone = stack.redo();
        assert_eq!(redone, Some(op));
    }

    #[test]
    fn undo_stack_clears_redo_on_new_push() {
        let mut stack = UndoStack::new();
        let op1 = EditOperation::Insert {
            block_id: Uuid::nil(),
            offset: 0,
            text: "hello".to_string(),
        };
        let op2 = EditOperation::Insert {
            block_id: Uuid::nil(),
            offset: 5,
            text: "world".to_string(),
        };

        stack.push(op1.clone());
        stack.undo();
        assert!(stack.redo().is_some()); // redo available

        stack.push(op2.clone());
        assert!(stack.redo().is_none()); // redo cleared
    }

    #[test]
    fn input_handler_text_insertion() {
        let mut handler = InputHandler::new(test_position());
        let ops = handler.handle_keydown("a", 0);
        assert_eq!(ops.len(), 1);
        if let EditOperation::Insert { text, .. } = &ops[0] {
            assert_eq!(text, "a");
        } else {
            panic!("Expected Insert operation");
        }
    }

    #[test]
    fn input_handler_backspace() {
        let mut handler = InputHandler::new(ModelPosition {
            block_id: Uuid::nil(),
            offset: 5,
        });
        let ops = handler.handle_keydown("Backspace", 0);
        assert_eq!(ops.len(), 1);
        if let EditOperation::Delete { start, end, .. } = &ops[0] {
            assert_eq!(*start, 4);
            assert_eq!(*end, 5);
        } else {
            panic!("Expected Delete operation");
        }
    }

    #[test]
    fn input_handler_delete() {
        let mut handler = InputHandler::new(test_position());
        let ops = handler.handle_keydown("Delete", 0);
        assert_eq!(ops.len(), 1);
        if let EditOperation::Delete { start, end, .. } = &ops[0] {
            assert_eq!(*start, 0);
            assert_eq!(*end, 1);
        } else {
            panic!("Expected Delete operation");
        }
    }

    #[test]
    fn input_handler_arrow_left() {
        let mut handler = InputHandler::new(ModelPosition {
            block_id: Uuid::nil(),
            offset: 10,
        });
        let _ = handler.handle_keydown("ArrowLeft", 0);
        assert_eq!(handler.cursor().position().offset, 9);
    }

    #[test]
    fn input_handler_arrow_right() {
        let mut handler = InputHandler::new(test_position());
        let _ = handler.handle_keydown("ArrowRight", 0);
        assert_eq!(handler.cursor().position().offset, 1);
    }

    #[test]
    fn input_handler_selection_with_shift_arrow() {
        let mut handler = InputHandler::new(test_position());
        let _ = handler.handle_keydown("ArrowRight", 0x01); // Shift
        assert!(!handler.selection().is_empty());
    }

    #[test]
    fn input_handler_paste_text() {
        let mut handler = InputHandler::new(test_position());
        let ops = handler.handle_paste_text("pasted text");
        assert_eq!(ops.len(), 1);
        if let EditOperation::Insert { text, .. } = &ops[0] {
            assert_eq!(text, "pasted text");
        } else {
            panic!("Expected Insert operation");
        }
    }
}
