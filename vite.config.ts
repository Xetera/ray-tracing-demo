import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
// import comlink from "vite-plugin-comlink";

export default defineConfig({
  // pass your local crate path to the plugin
  plugins: [
    // comlink(),
    wasm({
      // By default ALL `.wasm` imports will be transformed to WebAssembly ES module.
      // You can also set a filter (function or regex) to match files you want to transform.
      // Other files will fallback to Vite's default WASM loader (i.e. You need to call `initWasm()` for them).
      filter: /ray_tracing_bg.wasm$/,
    }),
  ],
  // worker: {
  //   plugins: [comlink()],
  // },
  server: {
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    },
  },
});
