# Folivm Dev Environment - Quick Start

## One-Command Startup

```bash
./START_DEV.sh
```

This will:
1. Build WASM in dev mode (~30 seconds)
2. Start Vite dev server
3. Open browser at http://localhost:1420

## Manual Setup (If Script Doesn't Work)

### Step 1: Build WASM
```bash
wasm-pack build crates/folivm-wasm --target web --dev
```

Output: `crates/folivm-wasm/pkg/`

### Step 2: Start Vite Dev Server
```bash
cd shell
npm run dev
```

Output:
```
Local:    http://localhost:1420/
```

### Step 3: Open in Browser
- Navigate to `http://localhost:1420`
- You should see a canvas with the test document loaded
- Cursor will be visible at the start of the document

## What You Can Test

### Keyboard Input
- **Type** any characters → appear in document
- **Backspace** → delete character before cursor
- **Delete** → delete character after cursor
- **Left/Right arrows** → move cursor
- **Shift+Left/Right** → select text
- **Ctrl+Z** → undo
- **Ctrl+Y** → redo (or Cmd on Mac)
- **Enter** → split block (if implemented)

### Mouse Input
- **Click** → position cursor at clicked location
- **Shift+Click** → extend selection from current position
- **Drag** → (not yet implemented, but mousedown works)

### Rendering
- **Placeholder text** will show glyphs as monospace characters
- **Cursor** is visible as a 2pt black line
- **Selection** shows semi-transparent blue highlight (basic)
- **Page background** is white
- **Text layout** uses cosmic-text for accurate glyph positioning

## Troubleshooting

### "wasm-pack command not found"
Install: `curl https://rustwasm.org/wasm-pack/installer/init.sh -sSf | sh`

### "Module not found" errors
Ensure wasm-pack built successfully to `crates/folivm-wasm/pkg/`

### "Cannot find canvas element"
Make sure you're accessing http://localhost:1420, not a file:// URL

### Black canvas with no text
- Check browser console for errors (F12 → Console)
- Verify WASM module loaded (F12 → Network, look for .wasm file)
- Make sure JS loaded (check main.ts errors)

## Development Workflow

### When You Make Changes

**Rust changes (core, layout, editor, wasm):**
1. Stop Vite (Ctrl+C in terminal)
2. Run `wasm-pack build crates/folivm-wasm --target web --dev`
3. Restart: `cd shell && npm run dev`

**TypeScript/Canvas changes (shell):**
1. Just save the file
2. Vite automatically reloads in browser (hot reload)
3. No rebuild needed

### Monitoring Changes

Keep these terminals open:
- Terminal 1: `./START_DEV.sh` (or `cd shell && npm run dev`)
- Terminal 2: Watch for `cargo test` results (optional)

## What's Not Implemented Yet

❌ Actual glyph rendering (using placeholder monospace text)
❌ Accurate cursor byte offset (always at block start)
❌ Multi-line selection rectangles
❌ Image rendering (gray placeholders)
❌ Scroll viewport
❌ Dirty rectangle optimization

## Next Steps After Testing Basics

1. **Add more test documents** - create .fvm files in test-docs/
2. **Test multi-block editing** - Add more headings/paragraphs
3. **Check undo/redo** - Ctrl+Z should undo, Ctrl+Y should redo
4. **Test selection** - Shift+click and Shift+arrows
5. **Profile performance** - Should be smooth for reasonable document sizes

## Performance Tips

- Don't make huge documents (50k+ characters) as layout can get slow
- Cursor positioning is O(n) where n = number of glyphs (can improve with indexing)
- Selection rendering rebuilds every frame (can optimize with dirty rectangles)

## Getting Help

Check these if something goes wrong:
- Browser console (F12 → Console tab)
- Network tab (make sure .wasm file loads)
- Terminal output (look for Rust compile errors)

## When Ready for Production

```bash
# Build optimized WASM
wasm-pack build crates/folivm-wasm --target web --release

# Build optimized shell
cd shell && npm run build

# Output: shell/dist/ (ready to deploy)
```

---

**Happy editing! 🎉**

If you run into issues, check the browser console (F12) for detailed error messages.
