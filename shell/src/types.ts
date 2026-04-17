// TypeScript mirror of folivm-core's RenderInstruction enum.
// Must stay in sync with crates/folivm-core/src/render/mod.rs.
// All coordinates and sizes are in pt (points), never px.
// Colors are packed ARGB u32: 0xAARRGGBB.

export type RenderInstruction =
  | { tag: 'FillRect';    x: number; y: number; w: number; h: number; color: number }
  | { tag: 'StrokeRect';  x: number; y: number; w: number; h: number; color: number; line_width: number }
  | { tag: 'DrawGlyph';   x: number; y: number; glyph_id: number; font_id: number; size: number; color: number }
  | { tag: 'DrawImage';   x: number; y: number; w: number; h: number; image_id: number }
  | { tag: 'Cursor';      x: number; y: number; height: number; color: number }
  | { tag: 'Selection';   rects: [number, number, number, number][]; color: number }
  | { tag: 'ClipPush';    x: number; y: number; w: number; h: number }
  | { tag: 'ClipPop' }
  | { tag: 'ScrollTo';    y: number }
  | { tag: 'SaveState' }
  | { tag: 'RestoreState' }
