/* tslint:disable */
/* eslint-disable */

export class FolvimInstance {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Return a full frame render buffer without mutating state.
     * Used after tab focus restore or window unhide.
     */
    full_frame(): Uint8Array;
    /**
     * Parse and load a `.fvm` document string. Returns the initial full frame.
     */
    load_document(fvm: string): Uint8Array;
    /**
     * Parse and apply a `.fvm-theme` string.
     */
    load_theme(theme: string): void;
    constructor();
    on_composition_end(text: string): Uint8Array;
    on_composition_start(text: string): Uint8Array;
    on_composition_update(text: string): Uint8Array;
    /**
     * `key` is a KeyboardEvent.key string; `modifiers` is a bitmask:
     * bit 0 = Shift, bit 1 = Ctrl/Cmd, bit 2 = Alt.
     */
    on_keydown(key: string, modifiers: number): Uint8Array;
    on_mousedown(x: number, y: number, modifiers: number): Uint8Array;
    on_mousemove(x: number, y: number, modifiers: number): Uint8Array;
    on_mouseup(x: number, y: number, modifiers: number): Uint8Array;
    on_paste_fvm(fvm: string): Uint8Array;
    on_paste_text(text: string): Uint8Array;
    on_wheel(delta_y: number): Uint8Array;
    redo(): Uint8Array;
    /**
     * Register a font for layout. Must be called before any layout that uses
     * this family. This is the only way to make fonts available on WASM —
     * system font discovery is never used.
     */
    register_font(family: string, weight: number, data: Uint8Array): void;
    /**
     * Serialise the current document to a `.fvm` string.
     */
    serialise(): string;
    /**
     * Notify the instance of a canvas resize or zoom change.
     * Returns a full frame render buffer.
     */
    set_canvas_size(width: number, height: number, dpi: number, zoom: number): Uint8Array;
    /**
     * `mode` is "outline" | "design".
     */
    set_mode(mode: string): Uint8Array;
    /**
     * Switch theme collection and/or light/dark mode.
     */
    set_theme_mode(collection: string, mode: string): Uint8Array;
    undo(): Uint8Array;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly __wbg_folviminstance_free: (a: number, b: number) => void;
    readonly folviminstance_full_frame: (a: number) => any;
    readonly folviminstance_load_document: (a: number, b: number, c: number) => [number, number, number];
    readonly folviminstance_load_theme: (a: number, b: number, c: number) => [number, number];
    readonly folviminstance_new: () => number;
    readonly folviminstance_on_composition_end: (a: number, b: number, c: number) => any;
    readonly folviminstance_on_composition_start: (a: number, b: number, c: number) => any;
    readonly folviminstance_on_composition_update: (a: number, b: number, c: number) => any;
    readonly folviminstance_on_keydown: (a: number, b: number, c: number, d: number) => any;
    readonly folviminstance_on_mousedown: (a: number, b: number, c: number, d: number) => any;
    readonly folviminstance_on_mousemove: (a: number, b: number, c: number, d: number) => any;
    readonly folviminstance_on_mouseup: (a: number, b: number, c: number, d: number) => any;
    readonly folviminstance_on_paste_fvm: (a: number, b: number, c: number) => any;
    readonly folviminstance_on_paste_text: (a: number, b: number, c: number) => any;
    readonly folviminstance_on_wheel: (a: number, b: number) => any;
    readonly folviminstance_redo: (a: number) => any;
    readonly folviminstance_register_font: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly folviminstance_serialise: (a: number) => [number, number];
    readonly folviminstance_set_canvas_size: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly folviminstance_set_mode: (a: number, b: number, c: number) => any;
    readonly folviminstance_set_theme_mode: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly folviminstance_undo: (a: number) => any;
    readonly memory: WebAssembly.Memory;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput, memory?: WebAssembly.Memory }} module - Passing `SyncInitInput` directly is deprecated.
 * @param {WebAssembly.Memory} memory - Deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput, memory?: WebAssembly.Memory } | SyncInitInput, memory?: WebAssembly.Memory): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory }} module_or_path - Passing `InitInput` directly is deprecated.
 * @param {WebAssembly.Memory} memory - Deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory } | InitInput | Promise<InitInput>, memory?: WebAssembly.Memory): Promise<InitOutput>;
