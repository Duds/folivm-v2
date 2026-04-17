// Shell entry point.
//
// Responsibilities (in order):
// 1. Initialise the WASM module
// 2. Construct a FolvimInstance
// 3. Size the canvas to the window (device pixel ratio aware)
// 4. Wire keyboard, mouse, and wheel events → WASM → canvas
// 5. Wire ResizeObserver → set_canvas_size → canvas
// 6. Set up IME handling (ime.ts)
// 7. Initialise ARIA shadow tree (accessibility.ts)
// 8. Listen for Tauri events (file:changed, theme:updated, dpi:changed)

import type { FolvimInstance } from './folivm_wasm'
import { executeFrame } from './canvas'
import { setupIME } from './ime'
import { initARIA } from './accessibility'

// ---------------------------------------------------------------------------
// WASM init
// ---------------------------------------------------------------------------

async function initWasm(): Promise<FolvimInstance> {
  // Import and initialise the WASM module.
  // wasm-pack generates a default export init() function.
  // Once initialised, we can construct a FolvimInstance.
  const wasmModule = await import('./wasm/folivm_wasm.js')
  await wasmModule.default()
  const instance = new wasmModule.FolvimInstance()
  return instance as unknown as FolvimInstance
}

// ---------------------------------------------------------------------------
// Canvas sizing
// ---------------------------------------------------------------------------

function sizeCanvas(canvas: HTMLCanvasElement): { width: number; height: number; dpi: number } {
  const dpi = window.devicePixelRatio || 1
  const width = canvas.clientWidth
  const height = canvas.clientHeight
  canvas.width = Math.round(width * dpi)
  canvas.height = Math.round(height * dpi)
  return { width, height, dpi }
}

// ---------------------------------------------------------------------------
// Event modifiers bitmask
// ---------------------------------------------------------------------------

function modifiers(e: KeyboardEvent | MouseEvent): number {
  return (e.shiftKey ? 1 : 0) | (e.ctrlKey || e.metaKey ? 2 : 0) | (e.altKey ? 4 : 0)
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main(): Promise<void> {
  const canvas = document.getElementById('editor') as HTMLCanvasElement | null
  if (!canvas) throw new Error('No #editor canvas element found')

  const ctx = canvas.getContext('2d')
  if (!ctx) throw new Error('Could not get 2D rendering context')

  const instance = await initWasm()

  // Test document for development - minimal version for debugging
  const testFvm = `---
title: Test
---
Hello world
`

  // Load test document
  try {
    instance.load_document(testFvm)
  } catch (e) {
    console.error('Failed to load document:', e)
  }

  // Scale factor for canvas: (canvas_w_px / page_w_pt) × (dpi/96) × zoom
  // We'll assume a standard US Letter page (612pt wide) for now.
  // This should come from document metadata.
  const pageWidthPt = 612
  let scale = 1.0

  function computeScale(canvasWidthPx: number, dpi: number, zoom: number): number {
    return (canvasWidthPx / pageWidthPt) * (dpi / 96) * zoom
  }

  // Initial sizing
  const { width, height, dpi } = sizeCanvas(canvas)
  scale = computeScale(width, dpi, 1.0)
  const initialFrame = instance.set_canvas_size(width, height, dpi, 1.0)
  console.log('Initial frame buffer size:', initialFrame.byteLength, 'bytes')
  executeFrame(initialFrame, ctx, scale)

  // Test: if no instructions, draw test text
  if (initialFrame.byteLength === 0) {
    ctx.fillStyle = '#000000'
    ctx.font = '16px monospace'
    ctx.fillText('Test: WASM returned empty buffer', 10, 30)
  }

  // Resize observer
  const observer = new ResizeObserver(() => {
    const { width, height, dpi } = sizeCanvas(canvas)
    scale = computeScale(width, dpi, 1.0)
    const frame = instance.set_canvas_size(width, height, dpi, 1.0)
    executeFrame(frame, ctx, scale)
  })
  observer.observe(canvas)

  // Keyboard events
  window.addEventListener('keydown', (e: KeyboardEvent) => {
    e.preventDefault()
    const frame = instance.on_keydown(e.key, modifiers(e))
    executeFrame(frame, ctx, scale)
  })

  // Mouse events
  canvas.addEventListener('mousedown', (e: MouseEvent) => {
    const frame = instance.on_mousedown(e.offsetX, e.offsetY, modifiers(e))
    executeFrame(frame, ctx, scale)
  })
  canvas.addEventListener('mousemove', (e: MouseEvent) => {
    const frame = instance.on_mousemove(e.offsetX, e.offsetY, modifiers(e))
    executeFrame(frame, ctx, scale)
  })
  canvas.addEventListener('mouseup', (e: MouseEvent) => {
    const frame = instance.on_mouseup(e.offsetX, e.offsetY, modifiers(e))
    executeFrame(frame, ctx, scale)
  })

  // Wheel / scroll
  canvas.addEventListener('wheel', (e: WheelEvent) => {
    e.preventDefault()
    const frame = instance.on_wheel(e.deltaY)
    executeFrame(frame, ctx, scale)
  }, { passive: false })

  // IME
  setupIME(instance, ctx, scale)

  // Accessibility
  initARIA()

  // TODO: listen for Tauri events (file:changed, theme:updated, dpi:changed)
  // TODO: open initial document
}

main().catch(console.error)
