/**
 * Type definitions for FolvimInstance (WASM bindings).
 * Generated from folivm-wasm crate via wasm-bindgen.
 * All methods return Uint8Array containing postcard-encoded RenderBuffer.
 */

export interface FolvimInstance {
  // Document
  load_document(fvm: string): Uint8Array
  load_theme(theme: string): void
  register_font(family: string, weight: number, data: Uint8Array): void

  // Canvas
  set_canvas_size(width: number, height: number, dpi: number, zoom: number): Uint8Array

  // Keyboard & Mouse
  on_keydown(key: string, modifiers: number): Uint8Array
  on_mousedown(x: number, y: number, modifiers: number): Uint8Array
  on_mousemove(x: number, y: number, modifiers: number): Uint8Array
  on_mouseup(x: number, y: number, modifiers: number): Uint8Array
  on_wheel(deltaY: number): Uint8Array

  // IME
  on_composition_start(text: string): Uint8Array
  on_composition_update(text: string): Uint8Array
  on_composition_end(text: string): Uint8Array

  // Clipboard
  on_paste_text(text: string): Uint8Array
  on_paste_fvm(fvm: string): Uint8Array

  // Modes
  set_mode(mode: 'outline' | 'design'): Uint8Array
  set_theme_mode(collection: string, mode: 'light' | 'dark'): Uint8Array

  // State
  serialise(): string
  undo(): Uint8Array
  redo(): Uint8Array
  full_frame(): Uint8Array
}
