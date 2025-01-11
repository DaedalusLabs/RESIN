import { defineConfig } from "vitest/config";
import vue from "@vitejs/plugin-vue";
import { fileURLToPath } from "node:url";

export default defineConfig({
   plugins: [vue()],
   test: {
      globals: true,
      environment: "happy-dom",
      include: ["**/*.{test,spec}.{js,ts,jsx,tsx}"],
      coverage: {
         provider: "v8",
         reporter: ["text", "json", "html"],
      },
      deps: {
         inline: ["@vue", "@nuxt"],
      },
   },
   resolve: {
      alias: {
         "@": fileURLToPath(new URL("./", import.meta.url)),
         "~": fileURLToPath(new URL("./", import.meta.url)),
      },
   },
});
