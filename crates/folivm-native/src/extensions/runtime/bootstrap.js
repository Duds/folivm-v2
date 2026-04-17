// folivm-native bootstrap script
// This script runs in every extension isolate to define the 'folivm' global.

globalThis.folivm = {
    extension: {
        id: Deno.core.ops.op_get_extension_id(),
        name: Deno.core.ops.op_get_extension_name(),
    },
    document: {
        getMetadata: () => Deno.core.ops.op_document_get_metadata(),
        getBlocks: () => Deno.core.ops.op_document_get_blocks(),
    },
    cells: {
        registerRenderer: (type, handler) => {
            // bridge to op_cell_register_renderer
            Deno.core.ops.op_cell_register_renderer(type);
        },
    },
    storage: {
        get: (key) => Deno.core.ops.op_storage_get(key),
        set: (key, value) => Deno.core.ops.op_storage_set(key, value.toString()),
        delete: (key) => Deno.core.ops.op_storage_delete(key),
    },
    on: (event, handler) => {
        // Placeholder for event system
    }
};

globalThis.console = {
    log: (...args) => Deno.core.ops.op_folivm_print(args.map(String).join(' ')),
    warn: (...args) => Deno.core.ops.op_folivm_print(`[WARN] ${args.map(String).join(' ')}`),
    error: (...args) => Deno.core.ops.op_folivm_print(`[ERROR] ${args.map(String).join(' ')}`),
};
