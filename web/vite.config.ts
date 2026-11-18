import { defineConfig } from "vite";

// The engine runs in the page (web/src/engine-worker.ts), so there is no API
// to proxy: the dev server only serves files.
export default defineConfig({
  worker: { format: "es" },
});
