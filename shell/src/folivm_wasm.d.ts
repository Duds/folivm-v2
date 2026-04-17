// Hand-written stub type declarations for the folivm-wasm WASM module.
// This file is replaced by the output of `wasm-pack build` once the Rust
// implementation is complete. Delete this file when wasm-pack generates
// the real bindings in pkg/folivm_wasm.d.ts.

/** The WASM entry point. One instance per editor tab. */
export class FolvimInstance {
  constructor()
  free(): void

  load_document(fvm: string): unknown
  load_theme(theme: string): void
  register_font(family: string, weight: number, data: Uint8Array): void

  set_canvas_size(width: number, height: number, dpi: number, zoom: number): Uint8Array

  on_keydown(key: string, modifiers: number): Uint8Array
  on_mousedown(x: number, y: number, modifiers: number): Uint8Array
  on_mousemove(x: number, y: number, modifiers: number): Uint8Array
  on_mouseup(x: number, y: number, modifiers: number): Uint8Array
  on_wheel(delta_y: number): Uint8Array

  on_composition_start(text: string): Uint8Array
  on_composition_update(text: string): Uint8Array
  on_composition_end(text: string): Uint8Array

  on_paste_text(text: string): Uint8Array
  on_paste_fvm(fvm: string): Uint8Array

  set_mode(mode: 'outline' | 'design'): Uint8Array
  set_theme_mode(collection: string, mode: string): Uint8Array

  serialise(): string

  undo(): Uint8Array
  redo(): Uint8Array

  /** Returns a full frame render buffer without mutating state. */
  full_frame(): Uint8Array
}

/** Initialise the WASM module. Must be awaited before constructing FolvimInstance. */
export function init(): Promise<void>
