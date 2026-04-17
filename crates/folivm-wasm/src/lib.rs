#![allow(unused_variables)]

use wasm_bindgen::prelude::*;
use folivm_core::{
    model::DocumentModel,
    layout::{ScaleFactor, LayoutEngine},
    render::{FrameRenderer, RenderBuffer},
    text_engine::{InputHandler, ModelPosition},
    parser, editor,
};
use uuid::Uuid;

/// The WASM entry point. One instance per editor tab.
///
/// All methods that mutate state return a `Uint8Array` containing a
/// postcard-encoded `Vec<RenderInstruction>` (a `RenderBuffer`). The shell
/// decodes this and executes it on the Canvas 2D context via `canvas.ts`.
/// Maps glyph positions for hit-testing
#[derive(Clone)]
struct GlyphHitMap {
    /// (block_id, glyph_index) → (x_pt, y_pt, width_pt, height_pt)
    glyphs: Vec<(folivm_core::model::BlockId, usize, f32, f32, f32, f32)>,
}

#[wasm_bindgen]
pub struct FolvimInstance {
    document: Option<DocumentModel>,
    layout_engine: LayoutEngine,
    input_handler: InputHandler,
    hit_map: GlyphHitMap,
    canvas_width_px: f32,
    canvas_height_px: f32,
    dpi: f32,
    zoom: f32,
    page_width_pt: f32,
    page_height_pt: f32,
}

#[wasm_bindgen]
impl FolvimInstance {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FolvimInstance {
        // Default initial position at block 0, offset 0
        let default_block_id = Uuid::nil();

        FolvimInstance {
            document: None,
            layout_engine: LayoutEngine::new(),
            input_handler: InputHandler::new(ModelPosition {
                block_id: default_block_id,
                offset: 0,
            }),
            hit_map: GlyphHitMap {
                glyphs: Vec::new(),
            },
            canvas_width_px: 800.0,
            canvas_height_px: 600.0,
            dpi: 96.0,
            zoom: 1.0,
            page_width_pt: 612.0,  // Standard US Letter width
            page_height_pt: 792.0, // Standard US Letter height
        }
    }

    /// Parse and load a `.fvm` document string. Returns the initial full frame.
    pub fn load_document(&mut self, fvm: &str) -> Result<js_sys::Uint8Array, JsValue> {
        match parser::parse(fvm) {
            Ok(doc) => {
                // Get the first block ID for initializing cursor position
                let first_block_id = doc
                    .blocks
                    .first()
                    .and_then(|b| b.id())
                    .unwrap_or_else(|| Uuid::nil());

                // Reset input handler with cursor at the beginning of the first block
                self.input_handler = InputHandler::new(ModelPosition {
                    block_id: first_block_id,
                    offset: 0,
                });

                self.document = Some(doc.clone());
                Ok(self.render_full_frame())
            }
            Err(e) => Err(JsValue::from_str(&format!("Parse error: {}", e))),
        }
    }

    /// Parse and apply a `.fvm-theme` string.
    pub fn load_theme(&mut self, theme: &str) -> Result<(), JsValue> {
        // TODO: parse theme and apply to document
        Ok(())
    }

    /// Register a font for layout. Must be called before any layout that uses
    /// this family. This is the only way to make fonts available on WASM —
    /// system font discovery is never used.
    pub fn register_font(&mut self, family: &str, weight: u16, data: &[u8]) {
        // TODO: wire to LayoutEngine.register_font()
    }

    /// Notify the instance of a canvas resize or zoom change.
    /// Returns a full frame render buffer.
    pub fn set_canvas_size(
        &mut self,
        width: f32,
        height: f32,
        dpi: f32,
        zoom: f32,
    ) -> js_sys::Uint8Array {
        self.canvas_width_px = width;
        self.canvas_height_px = height;
        self.dpi = dpi;
        self.zoom = zoom;
        self.render_full_frame()
    }

    /// `key` is a KeyboardEvent.key string; `modifiers` is a bitmask:
    /// bit 0 = Shift, bit 1 = Ctrl/Cmd, bit 2 = Alt.
    pub fn on_keydown(&mut self, key: &str, modifiers: u32) -> js_sys::Uint8Array {
        if self.document.is_none() {
            return js_sys::Uint8Array::new(&js_sys::Object::new());
        }

        // Generate EditOperations from keyboard input via InputHandler
        let operations = self.input_handler.handle_keydown(key, modifiers);

        // Apply each operation to the document
        if let Some(doc) = &mut self.document {
            for op in operations {
                if let Err(_) = editor::apply_operation(doc, &op) {
                    // Silently ignore operation errors for now
                }
            }
        }

        // Re-render after operations
        self.render_full_frame()
    }

    pub fn on_mousedown(&mut self, x: f32, y: f32, modifiers: u32) -> js_sys::Uint8Array {
        if self.document.is_none() {
            return js_sys::Uint8Array::new(&js_sys::Object::new());
        }

        let shift = (modifiers & 1) != 0;
        let _ctrl = (modifiers & 2) != 0;
        let _alt = (modifiers & 4) != 0;

        // Convert pixel coordinates to points
        let scale = ScaleFactor::compute(
            self.canvas_width_px,
            self.page_width_pt,
            self.dpi,
            self.zoom,
        );
        let x_pt = scale.px_to_pt(x);
        let y_pt = scale.px_to_pt(y);

        // Hit-test against glyph positions
        if let Some(new_pos) = self.hit_test_glyphs(x_pt, y_pt) {
            if shift {
                // Extend selection
                self.input_handler.selection_mut().set_focus(new_pos);
            } else {
                // Move cursor and clear selection
                self.input_handler.cursor_mut().move_to(new_pos);
                self.input_handler.selection_mut().clear();
            }
        }

        self.render_full_frame()
    }

    pub fn on_mousemove(&mut self, x: f32, y: f32, modifiers: u32) -> js_sys::Uint8Array {
        if self.document.is_none() {
            return js_sys::Uint8Array::new(&js_sys::Object::new());
        }

        let _shift = (modifiers & 1) != 0;
        let _ctrl = (modifiers & 2) != 0;
        let _alt = (modifiers & 4) != 0;

        // Convert pixel coordinates to points
        let scale = ScaleFactor::compute(
            self.canvas_width_px,
            self.page_width_pt,
            self.dpi,
            self.zoom,
        );
        let _x_pt = scale.px_to_pt(x);
        let _y_pt = scale.px_to_pt(y);

        // TODO: Update selection focus if button is held
        // For now, just render (no change unless selection is active)
        self.render_full_frame()
    }

    pub fn on_mouseup(&mut self, x: f32, y: f32, modifiers: u32) -> js_sys::Uint8Array {
        if self.document.is_none() {
            return js_sys::Uint8Array::new(&js_sys::Object::new());
        }

        // TODO: Finalize selection if dragging
        // For now, just render
        self.render_full_frame()
    }

    pub fn on_wheel(&mut self, delta_y: f32) -> js_sys::Uint8Array {
        if self.document.is_none() {
            return js_sys::Uint8Array::new(&js_sys::Object::new());
        }

        // Scroll by moving cursor down/up through blocks
        let shift_lines = if delta_y > 0.0 { 1 } else { -1 };

        let current_pos = self.input_handler.cursor().position();

        // TODO: Implement vertical navigation through blocks
        // For now, just re-render without changing cursor
        self.render_full_frame()
    }

    // IME composition events — driven by the hidden <input> in ime.ts.
    pub fn on_composition_start(&mut self, text: &str) -> js_sys::Uint8Array {
        js_sys::Uint8Array::new(&js_sys::Object::new())
    }

    pub fn on_composition_update(&mut self, text: &str) -> js_sys::Uint8Array {
        js_sys::Uint8Array::new(&js_sys::Object::new())
    }

    pub fn on_composition_end(&mut self, text: &str) -> js_sys::Uint8Array {
        self.on_paste_text(text)
    }

    pub fn on_paste_text(&mut self, text: &str) -> js_sys::Uint8Array {
        if self.document.is_none() || text.is_empty() {
            return js_sys::Uint8Array::new(&js_sys::Object::new());
        }

        // Generate Insert operation via InputHandler
        let operations = self.input_handler.handle_paste_text(text);

        // Apply operations to document
        if let Some(doc) = &mut self.document {
            for op in operations {
                if let Err(_) = editor::apply_operation(doc, &op) {
                    // Silently ignore operation errors
                }
            }
        }

        // Re-render
        self.render_full_frame()
    }

    pub fn on_paste_fvm(&mut self, fvm: &str) -> js_sys::Uint8Array {
        // TODO: parse and insert fvm content
        js_sys::Uint8Array::new(&js_sys::Object::new())
    }

    /// `mode` is "outline" | "design".
    pub fn set_mode(&mut self, mode: &str) -> js_sys::Uint8Array {
        // TODO: switch editor mode
        self.render_full_frame()
    }

    /// Switch theme collection and/or light/dark mode.
    pub fn set_theme_mode(&mut self, collection: &str, mode: &str) -> js_sys::Uint8Array {
        // TODO: apply theme
        self.render_full_frame()
    }

    /// Serialise the current document to a `.fvm` string.
    pub fn serialise(&self) -> String {
        match &self.document {
            Some(doc) => match folivm_core::parser::serialise(doc) {
                Ok(s) => s,
                Err(_) => String::new(),
            },
            None => String::new(),
        }
    }

    pub fn undo(&mut self) -> js_sys::Uint8Array {
        // TODO: wire to UndoStack
        js_sys::Uint8Array::new(&js_sys::Object::new())
    }

    pub fn redo(&mut self) -> js_sys::Uint8Array {
        // TODO: wire to UndoStack
        js_sys::Uint8Array::new(&js_sys::Object::new())
    }

    /// Return a full frame render buffer without mutating state.
    /// Used after tab focus restore or window unhide.
    pub fn full_frame(&self) -> js_sys::Uint8Array {
        // Convert to mutable to call render_full_frame
        // This is a workaround; ideally we'd have a const render path
        js_sys::Uint8Array::new(&js_sys::Object::new())
    }

    // Internal helpers

    fn render_full_frame(&mut self) -> js_sys::Uint8Array {
        match &self.document {
            Some(doc) => {
                // Compute scale factor
                let page_width_pt = doc
                    .frontmatter
                    .page_width
                    .unwrap_or(self.page_width_pt as f64) as f32;
                let page_height_pt = doc
                    .frontmatter
                    .page_height
                    .unwrap_or(self.page_height_pt as f64) as f32;

                let scale = ScaleFactor::compute(
                    self.canvas_width_px,
                    page_width_pt,
                    self.dpi,
                    self.zoom,
                );

                // Layout all blocks and build hit map
                let mut layouts = Vec::new();
                let mut hit_glyphs = Vec::new();

                let mut y_offset = 10.0; // Top margin
                for block in &doc.blocks {
                    let layout = self.layout_engine.layout_block(block, scale);

                    // Track glyph positions for hit-testing
                    if let Some(block_id) = block.id() {
                        for (glyph_idx, glyph) in layout.glyphs.iter().enumerate() {
                            let width_pt = glyph.size_pt * 0.6; // Approximate glyph width
                            hit_glyphs.push((
                                block_id,
                                glyph_idx,
                                glyph.x_pt,
                                glyph.y_pt,
                                width_pt,
                                glyph.size_pt,
                            ));
                        }
                    }

                    layouts.push(layout);
                    y_offset += 14.4; // Line height
                }

                self.hit_map.glyphs = hit_glyphs;

                // Get cursor position from InputHandler
                let cursor_pos = {
                    let cursor = self.input_handler.cursor().position();

                    // Find glyph at cursor position in hit map
                    self.hit_map.glyphs.iter().find(|(bid, _, _, _, _, _)| *bid == cursor.block_id)
                        .map(|(_, _, x, y, _, h)| {
                            // Place cursor at glyph position
                            (*x, *y, h * 1.2) // height with 20% padding
                        })
                        .or_else(|| {
                            // Fallback: use default position if no glyphs in block
                            Some((10.0, 10.0, 14.4))
                        })
                };

                let selection_rects = self.input_handler.selection().range().map(|(_a, _f)| {
                    // TODO: Compute selection rectangles from layout
                    // For now, return empty selection
                    Vec::new()
                });

                // Render full frame (white background + glyphs + selection + cursor)
                let render_buffer = FrameRenderer::render_full(
                    &layouts,
                    cursor_pos,
                    selection_rects,
                    page_width_pt,
                    page_height_pt,
                );

                // Convert RenderBuffer to Uint8Array for WASM→JS transfer
                render_buffer_to_uint8array(render_buffer)
            }
            None => {
                // No document loaded — return empty render buffer
                js_sys::Uint8Array::new(&js_sys::Object::new())
            }
        }
    }
}

/// Convert a RenderBuffer to a Uint8Array for zero-copy transfer to JavaScript.
fn render_buffer_to_uint8array(buffer: RenderBuffer) -> js_sys::Uint8Array {
    let bytes = buffer.into_vec();
    // Create a Uint8Array from the Vec without copying
    js_sys::Uint8Array::from(bytes.as_slice())
}

impl FolvimInstance {
    /// Find the closest position to a click in points
    fn hit_test_glyphs(&self, x_pt: f32, y_pt: f32) -> Option<ModelPosition> {
        if self.hit_map.glyphs.is_empty() {
            return None;
        }

        // Find the glyph closest to the clicked position
        let mut closest: Option<(ModelPosition, f32)> = None;

        for (block_id, _glyph_idx, glyph_x, glyph_y, glyph_w, glyph_h) in &self.hit_map.glyphs {
            // Simple bounding box check
            let glyph_x_max = glyph_x + glyph_w;
            let glyph_y_max = glyph_y + glyph_h;

            if x_pt >= *glyph_x && x_pt <= glyph_x_max && y_pt >= *glyph_y && y_pt <= glyph_y_max {
                // Click is within this glyph's bounds
                // Position cursor at start of glyph (could be before/after depending on side)
                let pos = ModelPosition {
                    block_id: *block_id,
                    offset: 0,
                };
                return Some(pos);
            }

            // Track closest glyph for fallback
            let dx = (x_pt - glyph_x).abs();
            let dy = (y_pt - glyph_y).abs();
            let dist = dx * dx + dy * dy; // squared distance

            if let Some((_, prev_dist)) = closest {
                if dist < prev_dist {
                    closest = Some((
                        ModelPosition {
                            block_id: *block_id,
                            offset: 0,
                        },
                        dist,
                    ));
                }
            } else {
                closest = Some((
                    ModelPosition {
                        block_id: *block_id,
                        offset: 0,
                    },
                    dist,
                ));
            }
        }

        closest.map(|(pos, _)| pos)
    }
}
