import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitepress";
import { dialectsVitePlugin, transformDialects } from "../../scripts/dialects.mjs";

const docsDir = join(dirname(fileURLToPath(import.meta.url)), "..");
const toc = readFileSync(join(docsDir, "toc.yaml"), "utf8");
const pages = [...toc.matchAll(/^\s+- path:\s+(\S+)\s*\n\s+title:\s*(.+)$/gm)].map((match) => ({
  path: match[1],
  title: match[2].trim(),
}));

const toLink = (path) => `/${path.replace(/\.md$/, "")}`;

export default defineConfig({
  title: "Datasources",
  description: "Data-access and object-relational mapper with parity across languages and providers.",
  cleanUrls: true,
  rewrites: {
    "introduction.md": "index.md",
  },
  themeConfig: {
    sidebar: pages.map(({ path, title }) => ({
      text: title,
      link: path === "introduction.md" ? "/" : toLink(path),
    })),
    outline: "deep",
  },
  markdown: {
    config(md) {
      const render = md.render.bind(md);
      md.render = (src, env) => render(transformDialects(src, "vitepress"), env);
    },
  },
  vite: {
    plugins: [dialectsVitePlugin()],
  },
});
