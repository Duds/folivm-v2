import type { FolvimInstance } from './folivm_wasm'
import { executeFrame } from './canvas'

/**
 * Mount a hidden <input> element for IME composition handling and wire its
 * events to the WASM instance.
 *
 * IME flow:
 * 1. compositionstart  → instance.on_composition_start()
 * 2. compositionupdate → instance.on_composition_update() (per keystroke)
 * 3. compositionend    → instance.on_composition_end()
 *
 * The shell is responsible for positioning the hidden input at the cursor
 * location reported by the WASM instance so the OS IME window appears in
 * the correct position.
 */
export function setupIME(
  instance: FolvimInstance,
  ctx: CanvasRenderingContext2D,
  scale: number,
): void {
  const input = document.createElement('input')
  input.id = 'ime-input'
  input.type = 'text'
  input.style.position = 'fixed'
  input.style.opacity = '0'
  input.style.pointerEvents = 'none'
  input.style.left = '0'
  input.style.top = '0'
  document.body.appendChild(input)

  // Focus the input by default so IME is active
  input.focus()

  // compositionstart: User begins IME composition
  input.addEventListener('compositionstart', (e: CompositionEvent) => {
    const text = e.data || ''
    const buffer = instance.on_composition_start(text)
    executeFrame(buffer, ctx, scale)
  })

  // compositionupdate: Text changes during IME composition (per keystroke)
  input.addEventListener('compositionupdate', (e: CompositionEvent) => {
    const text = e.data || ''
    const buffer = instance.on_composition_update(text)
    executeFrame(buffer, ctx, scale)
  })

  // compositionend: User commits the IME composition
  input.addEventListener('compositionend', (e: CompositionEvent) => {
    const text = e.data || ''
    const buffer = instance.on_composition_end(text)
    executeFrame(buffer, ctx, scale)
    // Clear the input for the next composition
    input.value = ''
  })

  // Also wire regular input for direct text entry (bypasses IME)
  input.addEventListener('input', (e: InputEvent) => {
    const text = input.value
    if (text) {
      const buffer = instance.on_paste_text(text)
      executeFrame(buffer, ctx, scale)
      input.value = ''
    }
  })
}

/**
 * Move the hidden IME input to the current cursor position so the OS IME
 * candidate window appears at the right place on screen.
 * @param x Cursor X position in pixels
 * @param y Cursor Y position in pixels
 * @param height Cursor height in pixels
 */
export function positionIMEInput(x: number, y: number, height: number): void {
  const input = document.getElementById('ime-input') as HTMLInputElement | null
  if (!input) return

  input.style.left = `${x}px`
  input.style.top = `${y}px`
  input.style.height = `${height}px`
}
