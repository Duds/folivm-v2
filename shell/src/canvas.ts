import type { RenderInstruction } from './types'

/**
 * Postcard decoder for RenderInstruction stream.
 * Matches the binary format from folivm-core's RenderBuffer::encode().
 */
class PostcardDecoder {
  private view: DataView
  private offset = 0

  constructor(buffer: Uint8Array) {
    this.view = new DataView(buffer.buffer, buffer.byteOffset, buffer.byteLength)
  }

  readU8(): number {
    const val = this.view.getUint8(this.offset)
    this.offset += 1
    return val
  }

  readU32(): number {
    const val = this.view.getUint32(this.offset, true)
    this.offset += 4
    return val >>> 0 // Ensure unsigned
  }

  readF32(): number {
    const val = this.view.getFloat32(this.offset, true)
    this.offset += 4
    return val
  }

  readVarint(): number {
    let result = 0
    let shift = 0
    for (;;) {
      const byte = this.readU8()
      result |= (byte & 0x7f) << shift
      if ((byte & 0x80) === 0) break
      shift += 7
    }
    return result
  }

  readVec<T>(fn: () => T): T[] {
    const len = this.readVarint()
    const result: T[] = []
    for (let i = 0; i < len; i++) {
      result.push(fn())
    }
    return result
  }

  readF32Array4(): [number, number, number, number] {
    return [this.readF32(), this.readF32(), this.readF32(), this.readF32()]
  }

  eof(): boolean {
    return this.offset >= this.view.byteLength
  }
}

/**
 * Decode a postcard-encoded RenderBuffer into an array of RenderInstructions.
 * Exposed separately so tests can inspect the decoded instructions.
 */
export function decodeRenderBuffer(buffer: Uint8Array): RenderInstruction[] {
  const decoder = new PostcardDecoder(buffer)
  const instructions: RenderInstruction[] = []

  while (!decoder.eof()) {
    const discriminant = decoder.readU8()

    switch (discriminant) {
      case 0: // FillRect
        instructions.push({
          tag: 'FillRect',
          x: decoder.readF32(),
          y: decoder.readF32(),
          w: decoder.readF32(),
          h: decoder.readF32(),
          color: decoder.readU32(),
        })
        break

      case 1: // StrokeRect
        instructions.push({
          tag: 'StrokeRect',
          x: decoder.readF32(),
          y: decoder.readF32(),
          w: decoder.readF32(),
          h: decoder.readF32(),
          color: decoder.readU32(),
          line_width: decoder.readF32(),
        })
        break

      case 2: // DrawGlyph
        instructions.push({
          tag: 'DrawGlyph',
          x: decoder.readF32(),
          y: decoder.readF32(),
          glyph_id: decoder.readU32(),
          font_id: decoder.readU32(),
          size: decoder.readF32(),
          color: decoder.readU32(),
        })
        break

      case 3: // DrawImage
        instructions.push({
          tag: 'DrawImage',
          x: decoder.readF32(),
          y: decoder.readF32(),
          w: decoder.readF32(),
          h: decoder.readF32(),
          image_id: decoder.readU32(),
        })
        break

      case 4: // Cursor
        instructions.push({
          tag: 'Cursor',
          x: decoder.readF32(),
          y: decoder.readF32(),
          height: decoder.readF32(),
          color: decoder.readU32(),
        })
        break

      case 5: // Selection
        instructions.push({
          tag: 'Selection',
          rects: decoder.readVec(() => decoder.readF32Array4()),
          color: decoder.readU32(),
        })
        break

      case 6: // ClipPush
        instructions.push({
          tag: 'ClipPush',
          x: decoder.readF32(),
          y: decoder.readF32(),
          w: decoder.readF32(),
          h: decoder.readF32(),
        })
        break

      case 7: // ClipPop
        instructions.push({ tag: 'ClipPop' })
        break

      case 8: // ScrollTo
        instructions.push({
          tag: 'ScrollTo',
          y: decoder.readF32(),
        })
        break

      case 9: // SaveState
        instructions.push({ tag: 'SaveState' })
        break

      case 10: // RestoreState
        instructions.push({ tag: 'RestoreState' })
        break

      default:
        throw new Error(`Unknown RenderInstruction discriminant: ${discriminant}`)
    }
  }

  return instructions
}

/**
 * Convert packed ARGB color (0xAARRGGBB) to CSS rgba() string.
 */
function colorToCss(color: number): string {
  const a = ((color >>> 24) & 0xff) / 255
  const r = (color >>> 16) & 0xff
  const g = (color >>> 8) & 0xff
  const b = color & 0xff
  return `rgba(${r}, ${g}, ${b}, ${a})`
}

/**
 * Convert canvas pixels to points.
 * Uses the inverse of ScaleFactor: scale = (canvas_w_px / page_w_pt) × (dpi/96) × zoom
 * So: pt_to_px = pt × scale, and px_to_pt = px / scale
 * Since all instructions are already in pt, we just scale them for display.
 */
function scalePt(pt: number, scale: number): number {
  // For now, ignore scale and use raw coordinates
  // This is a temporary fix until we figure out the scale calculation
  return pt
}

/**
 * Decode and execute a postcard-encoded RenderBuffer on the canvas.
 * @param buffer Uint8Array from WASM RenderBuffer
 * @param ctx Canvas 2D rendering context
 * @param scale Scale factor from ScaleFactor::compute() (canvas_w_px / page_w_pt) × (dpi/96) × zoom
 */
export function executeFrame(
  buffer: Uint8Array,
  ctx: CanvasRenderingContext2D,
  scale: number = 1.0,
): void {
  console.log('executeFrame: buffer size =', buffer.byteLength, 'scale =', scale)

  // Clear canvas first (will be overwritten by FillRect if present)
  ctx.fillStyle = '#ffffff'
  ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height)

  let instructions: RenderInstruction[] = []
  try {
    instructions = decodeRenderBuffer(buffer)
  } catch (e) {
    console.error('Failed to decode render buffer:', e)
    // Fallback: just show test text
    ctx.fillStyle = '#000000'
    ctx.font = '14px monospace'
    ctx.fillText('Error decoding render buffer. Check console.', 10, 20)
    return
  }

  console.log('executeFrame: decoded', instructions.length, 'instructions')
  console.log('First instruction:', instructions[0])

  if (instructions.length === 0) {
    console.warn('No render instructions')
    // Fallback: show test content
    ctx.fillStyle = '#000000'
    ctx.font = '14px monospace'
    ctx.fillText('No render instructions generated', 10, 20)
    return
  }

  console.log('About to loop through', instructions.length, 'instructions')
  for (let i = 0; i < instructions.length; i++) {
    const instr = instructions[i]
    console.log(`[${i}] Executing instruction:`, instr.tag, instr)
    switch (instr.tag) {
      case 'FillRect':
        ctx.fillStyle = colorToCss(instr.color)
        ctx.fillRect(
          scalePt(instr.x, scale),
          scalePt(instr.y, scale),
          scalePt(instr.w, scale),
          scalePt(instr.h, scale),
        )
        break

      case 'StrokeRect':
        ctx.strokeStyle = colorToCss(instr.color)
        ctx.lineWidth = scalePt(instr.line_width, scale)
        ctx.strokeRect(
          scalePt(instr.x, scale),
          scalePt(instr.y, scale),
          scalePt(instr.w, scale),
          scalePt(instr.h, scale),
        )
        break

      case 'DrawGlyph':
        // TODO: implement actual glyph rendering with font system.
        // For now, use canvas text rendering as a placeholder.
        // This gives visual feedback without needing font data.
        const fontSize = Math.max(14, Math.round(instr.size * 1.2))
        ctx.font = `${fontSize}px monospace`
        ctx.fillStyle = colorToCss(instr.color)
        // Use a placeholder character (we don't have the actual glyph data)
        // In the future, this will be replaced with actual glyph rendering
        ctx.fillText('█', instr.x, instr.y + fontSize * 0.75)
        break

      case 'DrawImage':
        // TODO: implement image rendering with image cache (image_id → HTMLImageElement)
        // For now, render a placeholder with image icon
        ctx.fillStyle = '#e8e8e8'
        ctx.fillRect(
          scalePt(instr.x, scale),
          scalePt(instr.y, scale),
          scalePt(instr.w, scale),
          scalePt(instr.h, scale),
        )
        ctx.strokeStyle = '#ccc'
        ctx.lineWidth = 1
        ctx.strokeRect(
          scalePt(instr.x, scale),
          scalePt(instr.y, scale),
          scalePt(instr.w, scale),
          scalePt(instr.h, scale),
        )
        // Draw image placeholder text
        ctx.fillStyle = '#999'
        ctx.font = '12px sans-serif'
        ctx.textAlign = 'center'
        ctx.fillText(
          '[Image]',
          scalePt(instr.x + instr.w / 2, scale),
          scalePt(instr.y + instr.h / 2 + 4, scale),
        )
        ctx.textAlign = 'start'
        break

      case 'Cursor':
        // Blinking text cursor - 2pt wide for visibility
        ctx.fillStyle = colorToCss(instr.color)
        ctx.fillRect(
          scalePt(instr.x, scale),
          scalePt(instr.y, scale),
          Math.max(1, scalePt(2, scale)), // Ensure at least 1px width
          scalePt(instr.height, scale),
        )
        break

      case 'Selection':
        ctx.fillStyle = colorToCss(instr.color)
        for (const [x, y, w, h] of instr.rects) {
          ctx.fillRect(scalePt(x, scale), scalePt(y, scale), scalePt(w, scale), scalePt(h, scale))
        }
        break

      case 'ClipPush':
        ctx.save()
        ctx.beginPath()
        ctx.rect(
          scalePt(instr.x, scale),
          scalePt(instr.y, scale),
          scalePt(instr.w, scale),
          scalePt(instr.h, scale),
        )
        ctx.clip()
        break

      case 'ClipPop':
        ctx.restore()
        break

      case 'ScrollTo':
        // TODO: implement scroll viewport tracking
        break

      case 'SaveState':
        ctx.save()
        break

      case 'RestoreState':
        ctx.restore()
        break
    }
  }
}
