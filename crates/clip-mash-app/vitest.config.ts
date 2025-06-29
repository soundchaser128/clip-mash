import * as path from "path"
import {defineConfig} from "vitest/config"
import react from "@vitejs/plugin-react-swc"
import tsconfigPaths from "vite-tsconfig-paths"

export default defineConfig({
  test: {
    environment: "jsdom",
    environmentOptions: {
      jsdom: {
        url: "http://localhost:3000",
      },
    },
    globals: true,
    includeSource: ["app/**/*.{ts,tsx}"],
    exclude: ["node_modules"],
    // TODO
    // coverage: {
    //   reporter: process.env.CI ? "json" : "html-spa",
    // },
    setupFiles: ["./src/vitest.ts"],
  },
  resolve: {
    alias: {
      "~": path.resolve(__dirname, "app"),
    },
  },
  plugins: [react(), tsconfigPaths()],
})
