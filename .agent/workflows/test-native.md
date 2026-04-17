---
description: Automated test workflow for the folivm-native extension host
---

# Workflow: Test Native Host

This workflow ensures that the native extension host tests can run successfully, even if the environment is missing certain assets (like Tauri icons) that cause build panics.

## Steps

1. **Verify Icon Presence**
   Ensure that a placeholder icon exists to satisfy the Tauri build process.
   // turbo
   ```bash
   mkdir -p crates/folivm-native/icons && [ ! -f crates/folivm-native/icons/icon.png ] && python3 -c "import zlib, struct; def chunk(t, d): return struct.pack('>I', len(d)) + t + d + struct.pack('>I', zlib.crc32(t + d) & 0xFFFFFFFF); s = b'\x89PNG\r\n\x1a\n'; ihdr = chunk(b'IHDR', struct.pack('>IIBBBBB', 32, 32, 8, 2, 0, 0, 0)); idat = chunk(b'IDAT', zlib.compress(bytes([0] + [0,0,0]*32)*32)); iend = chunk(b'IEND', b''); open('crates/folivm-native/icons/icon.png', 'wb').write(s + ihdr + idat + iend)" || true
   ```

2. **Run Extension Host Tests**
   Execute the specific test module for the extension host.
   // turbo
   ```bash
   cargo test -p folivm-native extensions::tests
   ```

3. **Cleanup (Optional)**
   Optionally remove the temporary icon if it was created specifically for the test run.
   ```bash
   # rm crates/folivm-native/icons/icon.png
   ```
