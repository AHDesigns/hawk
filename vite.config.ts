import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  plugins: [solidPlugin()],
  clearScreen: false,
  build: {
    target: "esnext",
    polyfillDynamicImport: false,
  },
});
