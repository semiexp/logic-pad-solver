A solver for [Logic Pad](https://logic-pad.com) based on cspuz.

# Supported rules

- Global rules
  - Forbidden patterns (`Don't make this pattern`)
  - Connectivity (`Connect all light / dark cells`)
- Cell rules
  - Fixed tiles
  - Merged tiles
  - Area Number
  - Letter
  - Viewpoint
  - Dart
  - Lotus
  - Galaxy
  - Minesweeper

Some "exceptional" clue arrangements such as "galaxies" on a corner of cell are intentionally unsupported because of their unnatural behavior on Logic Pad.

# Build

Prerequisites:

- Clone [enigma_csp](https://github.com/semiexp/enigma_csp) to `../enigma_csp` (relative to this file `README.md`)
- Install Rust as well as [emscripten](https://emscripten.org/)
- Make sure that a submodule `src/logic-pad` is correctly cloned. For this submodule, we have to make a change as follows (TODO: make this unnecessary):

```
--- a/src/data/serializer/compressor/streamCompressor.ts
+++ b/src/data/serializer/compressor/streamCompressor.ts
@@ -1,10 +1,12 @@
 import CompressorBase from './compressorBase';
 
 function ensureCompressionStream() {
+  /*
   if (!globalThis.CompressionStream || !globalThis.DecompressionStream) {
     console.log('CompressionStream not supported. Loading polyfill.');
     return import('../../../polyfill/streamPolyfill');
   }
+  */
   return Promise.resolve();
 }
```

How to build:

- `npm run build-rust`: Build internal solver. This should be run before `npm run dev` and `npm run build`.
- `npm run dev`: Start the dev server.
- `npm run build`: Create a release build.
