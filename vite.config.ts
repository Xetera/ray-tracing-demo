import { defineConfig } from "vite";

export default defineConfig({
  // root: ".",
  build: {
    sourcemap: true,
  },
  // optimizeDeps: {
  //   exclude: ["node_modules"],
  // },
  // pwass your local crate path to the plugin
  worker: {
    format: "es",
  },
  // worker: {
  //   plugins: [comlink()],
  // },
  server: {
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
      "Access-Control-Allow-Origin": "http://localhost:3001",
    },
  },
});
