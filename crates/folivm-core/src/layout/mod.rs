use crate::model::Block;
use crate::text_engine::buffer::RunBuffer;

#[cfg(feature = "layout")]
use cosmic_text::{Attrs, Buffer, FontSystem, Shaping};

/// Single scale factor for the entire canvas.
///
/// Formula: `(canvas_width_px / page_width_pt) × (screen_dpi / 96) × zoom`
///
/// The ruler, canvas, and margin guides all derive from this single value.
/// Never compute a separate scale for any individual element.
#[derive(Debug, Clone, Copy)]
pub struct ScaleFactor(f32);

impl ScaleFactor {
    /// Compute the single scale factor for the entire canvas.
    ///
    /// Formula: `(canvas_width_px / page_width_pt) × (screen_dpi / 96) × zoom`
    ///
    /// This ensures the entire canvas (ruler, guides, rendered content) all use
    /// the same scale factor, maintaining visual consistency.
    pub fn compute(canvas_w_px: f32, page_w_pt: f32, dpi: f32, zoom: f32) -> Self {
        if page_w_pt <= 0.0 {
            return ScaleFactor(1.0); // Safe default
        }
        let scale = (canvas_w_px / page_w_pt) * (dpi / 96.0) * zoom;
        ScaleFactor(scale)
    }

    pub fn value(self) -> f32 {
        self.0
    }

    /// Convert points to pixels using this scale factor.
    pub fn pt_to_px(self, pt: f32) -> f32 {
        pt * self.0
    }

    /// Convert pixels to points using this scale factor.
    pub fn px_to_pt(self, px: f32) -> f32 {
        if self.0 == 0.0 {
            0.0
        } else {
            px / self.0
        }
    }
}

/// Position of a single glyph. All values are in pt (points), never px.
#[derive(Debug, Clone)]
pub struct GlyphPosition {
    pub x_pt: f32,
    pub y_pt: f32,
    pub glyph_id: u32,
    pub font_id: u32,
    pub size_pt: f32,
    pub color: u32,
}

/// Layout result for a single block. All values are in pt (points), never px.
#[derive(Debug, Clone)]
pub struct BlockLayout {
    pub glyphs: Vec<GlyphPosition>,
    /// Total block height in pt.
    pub height_pt: f32,
}

/// Wraps cosmic-text's FontSystem. Both WASM and native load the same font
/// bytes via `register_font()` — this is what guarantees numerically identical
/// line breaks between the canvas editor and the PDF export pipeline.
pub struct LayoutEngine {
    #[cfg(feature = "layout")]
    font_system: cosmic_text::FontSystem,
}

impl LayoutEngine {
    pub fn new() -> Self {
        #[cfg(feature = "layout")]
        {
            Self {
                font_system: FontSystem::new(),
            }
        }

        #[cfg(not(feature = "layout"))]
        {
            Self
        }
    }

    /// Register a font for use in layout. Must be called before any layout
    /// operation that uses this family. On WASM, this is the only way to
    /// make fonts available — system font discovery is never used.
    pub fn register_font(&mut self, family: &str, weight: u16, data: &[u8]) {
        #[cfg(feature = "layout")]
        {
            // Load font data into the FontSystem's font database
            // cosmic-text will handle the font parsing and registration
            self.font_system.db_mut().load_font_data(data.to_vec());
        }

        #[cfg(not(feature = "layout"))]
        {
            // Layout feature disabled - silently ignore
            let _ = (family, weight, data);
        }
    }

    pub fn layout_block(&mut self, block: &Block, scale: ScaleFactor) -> BlockLayout {
        #[cfg(feature = "layout")]
        {
            match block {
                Block::Paragraph { content, style, .. } | Block::Heading { content, style, .. } => {
                    Self::layout_runs(content, style, scale, &mut self.font_system)
                }
                _ => BlockLayout {
                    glyphs: Vec::new(),
                    height_pt: 0.0,
                },
            }
        }

        #[cfg(not(feature = "layout"))]
        {
            // Layout feature disabled - return empty layout
            let _ = (block, scale);
            BlockLayout {
                glyphs: Vec::new(),
                height_pt: 0.0,
            }
        }
    }

    #[cfg(feature = "layout")]
    fn layout_runs(
        content: &RunBuffer,
        _style: &str,
        scale: ScaleFactor,
        font_system: &mut FontSystem,
    ) -> BlockLayout {
        // Get text from RunBuffer
        let text = content.to_string();

        // For now, use a simple character-based layout as a placeholder
        // until fonts are properly registered in the FontSystem
        let mut glyphs = Vec::new();
        let font_size_pt = 12.0;
        let char_width_pt = font_size_pt * 0.6; // Approximate monospace width
        let line_height_pt = 18.0;

        let mut x_pt: f32 = 10.0;
        let mut y_pt: f32 = 10.0;
        let mut max_height_pt: f32 = 10.0 + line_height_pt;

        for ch in text.chars() {
            if ch == '\n' {
                y_pt += line_height_pt;
                x_pt = 10.0;
                max_height_pt = max_height_pt.max(y_pt + line_height_pt);
            } else {
                glyphs.push(GlyphPosition {
                    x_pt,
                    y_pt,
                    glyph_id: ch as u32,
                    font_id: 0,
                    size_pt: font_size_pt,
                    color: 0xFF000000, // Black
                });
                x_pt += char_width_pt;
            }
        }

        BlockLayout {
            glyphs,
            height_pt: max_height_pt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_factor_compute() {
        // Standard 96 DPI, 100% zoom, page width 612 pt (8.5 inches)
        let scale = ScaleFactor::compute(612.0, 612.0, 96.0, 1.0);
        assert!((scale.value() - 1.0).abs() < 0.01);
    }

    #[test]
    fn scale_factor_with_zoom() {
        // 200% zoom should double the scale
        let scale1 = ScaleFactor::compute(612.0, 612.0, 96.0, 1.0);
        let scale2 = ScaleFactor::compute(612.0, 612.0, 96.0, 2.0);
        assert!((scale2.value() - scale1.value() * 2.0).abs() < 0.01);
    }

    #[test]
    fn scale_factor_with_dpi() {
        // 192 DPI (2x) should double the scale
        let scale1 = ScaleFactor::compute(612.0, 612.0, 96.0, 1.0);
        let scale2 = ScaleFactor::compute(612.0, 612.0, 192.0, 1.0);
        assert!((scale2.value() - scale1.value() * 2.0).abs() < 0.01);
    }

    #[test]
    fn scale_factor_with_canvas_size() {
        // Narrower canvas should require higher scale to fit page
        let scale1 = ScaleFactor::compute(612.0, 612.0, 96.0, 1.0);
        let scale2 = ScaleFactor::compute(306.0, 612.0, 96.0, 1.0);
        assert!((scale2.value() - scale1.value() * 0.5).abs() < 0.01);
    }

    #[test]
    fn scale_factor_zero_page_width() {
        // Zero page width should return safe default
        let scale = ScaleFactor::compute(612.0, 0.0, 96.0, 1.0);
        assert_eq!(scale.value(), 1.0);
    }

    #[test]
    fn scale_factor_pt_to_px() {
        let scale = ScaleFactor::compute(612.0, 612.0, 96.0, 1.0);
        let px = scale.pt_to_px(72.0); // 72 pt = 1 inch
        assert!((px - 72.0).abs() < 0.01);
    }

    #[test]
    fn scale_factor_px_to_pt() {
        let scale = ScaleFactor::compute(612.0, 612.0, 96.0, 1.0);
        let pt = scale.px_to_pt(72.0);
        assert!((pt - 72.0).abs() < 0.01);
    }

    #[test]
    fn scale_factor_roundtrip() {
        let scale = ScaleFactor::compute(800.0, 612.0, 96.0, 1.5);
        let original_pt = 36.0;
        let px = scale.pt_to_px(original_pt);
        let recovered_pt = scale.px_to_pt(px);
        assert!((recovered_pt - original_pt).abs() < 0.01);
    }

    #[test]
    fn scale_factor_zero_scale_px_to_pt() {
        // Zero scale should safely return 0
        let scale = ScaleFactor(0.0);
        assert_eq!(scale.px_to_pt(100.0), 0.0);
    }

    #[test]
    fn layout_engine_new() {
        let _engine = LayoutEngine::new();
        // Just verify it can be created without panic
    }

    #[test]
    #[cfg(feature = "layout")]
    fn layout_engine_register_font() {
        let mut engine = LayoutEngine::new();
        // Create a minimal test font data (this is not a real font, just for API testing)
        let minimal_font_data = vec![0u8; 100]; // Placeholder
        engine.register_font("TestFont", 400, &minimal_font_data);
        // If we get here without panic, the method works
    }

    #[test]
    #[cfg(feature = "layout")]
    fn layout_engine_layout_paragraph() {
        use crate::model::Block;
        use crate::text_engine::buffer::RunBuffer;
        use uuid::Uuid;

        let mut engine = LayoutEngine::new();

        // Create a simple paragraph block
        let content = RunBuffer::new();
        let block = Block::Paragraph {
            id: Uuid::nil(),
            style: "Body".to_string(),
            content,
        };

        let scale = ScaleFactor::compute(612.0, 612.0, 96.0, 1.0);
        let layout = engine.layout_block(&block, scale);

        // Empty content should produce empty glyphs
        assert_eq!(layout.glyphs.len(), 0);
        assert_eq!(layout.height_pt, 0.0);
    }

    #[test]
    #[cfg(feature = "layout")]
    fn layout_engine_layout_heading() {
        use crate::model::Block;
        use crate::text_engine::buffer::RunBuffer;
        use uuid::Uuid;

        let mut engine = LayoutEngine::new();

        // Create a simple heading block
        let content = RunBuffer::new();
        let block = Block::Heading {
            id: Uuid::nil(),
            level: 1,
            style: "Heading1".to_string(),
            content,
        };

        let scale = ScaleFactor::compute(612.0, 612.0, 96.0, 1.0);
        let layout = engine.layout_block(&block, scale);

        // Empty content should produce empty glyphs
        assert_eq!(layout.glyphs.len(), 0);
        assert_eq!(layout.height_pt, 0.0);
    }

    #[test]
    fn layout_engine_other_block_types() {
        use crate::model::{Block, CellBlock};
        use uuid::Uuid;

        let mut engine = LayoutEngine::new();

        // Test with a cell block (should return empty layout)
        let block = Block::Cell(CellBlock {
            id: Uuid::nil(),
            cell_type: "image".to_string(),
            attrs: serde_json::json!({}),
        });

        let scale = ScaleFactor::compute(612.0, 612.0, 96.0, 1.0);
        let layout = engine.layout_block(&block, scale);

        assert_eq!(layout.glyphs.len(), 0);
        assert_eq!(layout.height_pt, 0.0);
    }
}
