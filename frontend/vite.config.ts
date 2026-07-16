import { defineConfig } from "vite";

export default defineConfig({
  root: ".",
  optimizeDeps: {
    exclude: ["katex"],
  },
});
