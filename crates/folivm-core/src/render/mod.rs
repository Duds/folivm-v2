use serde::{Deserialize, Serialize};
use crate::layout::{BlockLayout, GlyphPosition, ScaleFactor};
use crate::text_engine::ModelPosition;

/// All coordinates and sizes are in pt (points), never px.
/// Colors are packed ARGB u32: `0xAARRGGBB`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenderInstruction {
    FillRect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: u32,
    },
    StrokeRect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: u32,
        line_width: f32,
    },
    DrawGlyph {
        x: f32,
        y: f32,
        glyph_id: u32,
        font_id: u32,
        size: f32,
        color: u32,
    },
    DrawImage {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        image_id: u32,
    },
    Cursor {
        x: f32,
        y: f32,
        height: f32,
        color: u32,
    },
    Selection {
        /// Each rect is `[x, y, w, h]` in pt.
        rects: Vec<[f32; 4]>,
        color: u32,
    },
    ClipPush {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    },
    ClipPop,
    ScrollTo {
        y: f32,
    },
    SaveState,
    RestoreState,
}

/// Postcard-encoded binary buffer for zero-copy WASM↔JS transfer.
/// Returned from all `FolvimInstance` methods that mutate state.
///
/// Encoding: `postcard::to_allocvec(&Vec<RenderInstruction>)`.
/// Decoding on the JS side mirrors this layout exactly.
pub struct RenderBuffer(pub Vec<u8>);

impl RenderBuffer {
    /// Encode a vector of RenderInstructions to a simple binary format.
    /// Format: each instruction is: [discriminant:u8] [fields...]
    /// No length prefix - the decoder reads until EOF.
    pub fn encode(instructions: &[RenderInstruction]) -> Self {
        let mut bytes = Vec::new();

        for instr in instructions {
            match instr {
                RenderInstruction::FillRect { x, y, w, h, color } => {
                    bytes.push(0);
                    bytes.extend_from_slice(&x.to_le_bytes());
                    bytes.extend_from_slice(&y.to_le_bytes());
                    bytes.extend_from_slice(&w.to_le_bytes());
                    bytes.extend_from_slice(&h.to_le_bytes());
                    bytes.extend_from_slice(&color.to_le_bytes());
                }
                RenderInstruction::StrokeRect { x, y, w, h, color, line_width } => {
                    bytes.push(1);
                    bytes.extend_from_slice(&x.to_le_bytes());
                    bytes.extend_from_slice(&y.to_le_bytes());
                    bytes.extend_from_slice(&w.to_le_bytes());
                    bytes.extend_from_slice(&h.to_le_bytes());
                    bytes.extend_from_slice(&color.to_le_bytes());
                    bytes.extend_from_slice(&line_width.to_le_bytes());
                }
                RenderInstruction::DrawGlyph { x, y, glyph_id, font_id, size, color } => {
                    bytes.push(2);
                    bytes.extend_from_slice(&x.to_le_bytes());
                    bytes.extend_from_slice(&y.to_le_bytes());
                    bytes.extend_from_slice(&glyph_id.to_le_bytes());
                    bytes.extend_from_slice(&font_id.to_le_bytes());
                    bytes.extend_from_slice(&size.to_le_bytes());
                    bytes.extend_from_slice(&color.to_le_bytes());
                }
                RenderInstruction::DrawImage { x, y, w, h, image_id } => {
                    bytes.push(3);
                    bytes.extend_from_slice(&x.to_le_bytes());
                    bytes.extend_from_slice(&y.to_le_bytes());
                    bytes.extend_from_slice(&w.to_le_bytes());
                    bytes.extend_from_slice(&h.to_le_bytes());
                    bytes.extend_from_slice(&image_id.to_le_bytes());
                }
                RenderInstruction::Cursor { x, y, height, color } => {
                    bytes.push(4);
                    bytes.extend_from_slice(&x.to_le_bytes());
                    bytes.extend_from_slice(&y.to_le_bytes());
                    bytes.extend_from_slice(&height.to_le_bytes());
                    bytes.extend_from_slice(&color.to_le_bytes());
                }
                RenderInstruction::Selection { rects, color } => {
                    bytes.push(5);
                    // Encode vec length as varint
                    let len = rects.len();
                    Self::encode_varint(&mut bytes, len as u32);
                    for rect in rects {
                        for &val in rect.iter() {
                            bytes.extend_from_slice(&val.to_le_bytes());
                        }
                    }
                    bytes.extend_from_slice(&color.to_le_bytes());
                }
                RenderInstruction::ClipPush { x, y, w, h } => {
                    bytes.push(6);
                    bytes.extend_from_slice(&x.to_le_bytes());
                    bytes.extend_from_slice(&y.to_le_bytes());
                    bytes.extend_from_slice(&w.to_le_bytes());
                    bytes.extend_from_slice(&h.to_le_bytes());
                }
                RenderInstruction::ClipPop => {
                    bytes.push(7);
                }
                RenderInstruction::ScrollTo { y } => {
                    bytes.push(8);
                    bytes.extend_from_slice(&y.to_le_bytes());
                }
                RenderInstruction::SaveState => {
                    bytes.push(9);
                }
                RenderInstruction::RestoreState => {
                    bytes.push(10);
                }
            }
        }

        RenderBuffer(bytes)
    }

    fn encode_varint(bytes: &mut Vec<u8>, mut val: u32) {
        loop {
            let byte = (val & 0x7f) as u8;
            val >>= 7;
            if val == 0 {
                bytes.push(byte);
                break;
            } else {
                bytes.push(byte | 0x80);
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

/// Glyph rendering — converts layout output to render instructions.
pub struct GlyphRenderer;

impl GlyphRenderer {
    /// Render a single block's glyphs to RenderInstructions.
    pub fn render_block_glyphs(layout: &BlockLayout) -> Vec<RenderInstruction> {
        layout
            .glyphs
            .iter()
            .map(|glyph| RenderInstruction::DrawGlyph {
                x: glyph.x_pt,
                y: glyph.y_pt,
                glyph_id: glyph.glyph_id,
                font_id: glyph.font_id,
                size: glyph.size_pt,
                color: glyph.color,
            })
            .collect()
    }

    /// Render cursor at a position.
    /// Cursor height defaults to 1.2em (approximately 14.4pt at 12pt font size).
    pub fn render_cursor(
        x: f32,
        y: f32,
        height: f32,
        color: u32,
    ) -> RenderInstruction {
        RenderInstruction::Cursor { x, y, height, color }
    }

    /// Render a selection as a set of rectangles.
    pub fn render_selection(rects: Vec<[f32; 4]>, color: u32) -> RenderInstruction {
        RenderInstruction::Selection { rects, color }
    }
}

/// Produces a complete instruction set for a full canvas repaint.
/// Called after load, resize, theme change, or mode switch.
pub struct FrameRenderer;

impl FrameRenderer {
    /// Render a complete frame from block layouts.
    ///
    /// Combines:
    /// - Background fill for canvas area
    /// - All block glyphs
    /// - Cursor position
    /// - Selection highlights
    pub fn render_full(
        blocks: &[BlockLayout],
        cursor_pos: Option<(f32, f32, f32)>, // x, y, height in pt
        selection_rects: Option<Vec<[f32; 4]>>,
        page_width: f32,
        page_height: f32,
    ) -> RenderBuffer {
        let mut instructions = Vec::new();

        // Background: white page
        instructions.push(RenderInstruction::FillRect {
            x: 0.0,
            y: 0.0,
            w: page_width,
            h: page_height,
            color: 0xFFFFFFFF, // White
        });

        // Render all block glyphs
        for block in blocks {
            instructions.extend(GlyphRenderer::render_block_glyphs(block));
        }

        // Selection (rendered before cursor so cursor appears on top)
        if let Some(rects) = selection_rects {
            instructions.push(RenderInstruction::Selection {
                rects,
                color: 0x4D0000FF, // Semi-transparent blue
            });
        }

        // Cursor (rendered last so it's on top)
        if let Some((x, y, height)) = cursor_pos {
            instructions.push(RenderInstruction::Cursor {
                x,
                y,
                height,
                color: 0xFF000000, // Black
            });
        }

        RenderBuffer::encode(&instructions)
    }
}

/// Produces a minimal delta instruction set after a single `EditOperation`.
/// Called after every keystroke, mouse event, or IME commit.
pub struct DeltaRenderer;

impl DeltaRenderer {
    /// Render only the changed regions after an edit.
    ///
    /// For now, this is a simplified implementation that re-renders affected blocks.
    /// A more sophisticated approach would compute dirty rectangles.
    pub fn render_delta(
        changed_blocks: &[BlockLayout],
        cursor_pos: Option<(f32, f32, f32)>,
        selection_rects: Option<Vec<[f32; 4]>>,
    ) -> RenderBuffer {
        let mut instructions = Vec::new();

        // Render only the changed block glyphs
        for block in changed_blocks {
            instructions.extend(GlyphRenderer::render_block_glyphs(block));
        }

        // Selection
        if let Some(rects) = selection_rects {
            instructions.push(RenderInstruction::Selection {
                rects,
                color: 0x4D0000FF,
            });
        }

        // Cursor
        if let Some((x, y, height)) = cursor_pos {
            instructions.push(RenderInstruction::Cursor {
                x,
                y,
                height,
                color: 0xFF000000,
            });
        }

        RenderBuffer::encode(&instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glyph_renderer() {
        let layout = BlockLayout {
            glyphs: vec![
                GlyphPosition {
                    x_pt: 10.0,
                    y_pt: 20.0,
                    glyph_id: 42,
                    font_id: 1,
                    size_pt: 12.0,
                    color: 0xFF000000,
                },
                GlyphPosition {
                    x_pt: 20.0,
                    y_pt: 20.0,
                    glyph_id: 43,
                    font_id: 1,
                    size_pt: 12.0,
                    color: 0xFF000000,
                },
            ],
            height_pt: 30.0,
        };

        let instructions = GlyphRenderer::render_block_glyphs(&layout);
        assert_eq!(instructions.len(), 2);
    }

    #[test]
    fn test_render_buffer_encode() {
        let instructions = vec![RenderInstruction::FillRect {
            x: 0.0,
            y: 0.0,
            w: 100.0,
            h: 100.0,
            color: 0xFFFFFFFF,
        }];

        let buffer = RenderBuffer::encode(&instructions);
        assert!(!buffer.as_bytes().is_empty());
    }

    #[test]
    fn test_frame_renderer() {
        let layout = BlockLayout {
            glyphs: vec![],
            height_pt: 100.0,
        };

        let buffer = FrameRenderer::render_full(
            &[layout],
            Some((50.0, 50.0, 14.4)),
            None,
            612.0,
            792.0,
        );

        assert!(!buffer.as_bytes().is_empty());
    }

    #[test]
    fn test_delta_renderer() {
        let layout = BlockLayout {
            glyphs: vec![],
            height_pt: 100.0,
        };

        let buffer = DeltaRenderer::render_delta(
            &[layout],
            Some((50.0, 50.0, 14.4)),
            Some(vec![[10.0, 10.0, 40.0, 14.4]]),
        );

        assert!(!buffer.as_bytes().is_empty());
    }

    #[test]
    fn test_cursor_rendering() {
        let cursor = GlyphRenderer::render_cursor(100.0, 200.0, 14.4, 0xFF000000);
        match cursor {
            RenderInstruction::Cursor { x, y, height, color } => {
                assert_eq!(x, 100.0);
                assert_eq!(y, 200.0);
                assert_eq!(height, 14.4);
                assert_eq!(color, 0xFF000000);
            }
            _ => panic!("Expected Cursor instruction"),
        }
    }

    #[test]
    fn test_selection_rendering() {
        let rects = vec![[10.0, 20.0, 50.0, 14.4], [10.0, 34.4, 30.0, 14.4]];
        let selection = GlyphRenderer::render_selection(rects.clone(), 0x4D0000FF);
        match selection {
            RenderInstruction::Selection { rects: r, color } => {
                assert_eq!(r, rects);
                assert_eq!(color, 0x4D0000FF);
            }
            _ => panic!("Expected Selection instruction"),
        }
    }
}
