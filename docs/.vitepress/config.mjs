import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitepress";
import { dialectsVitePlugin, transformDialects } from "../../scripts/dialects.mjs";
import { parseToc } from "../../scripts/toc.mjs";

const docsDir = join(dirname(fileURLToPath(import.meta.url)), "..");
const tocPath = join(docsDir, "toc.yaml");
const { pages } = parseToc(readFileSync(tocPath, "utf8"));

const toLink = ({ path, hash }) =>
  `${path === "introduction.md" ? "/" : `/${path.replace(/\.md$/, "")}`}${hash ? `#${hash}` : ""}`;

const toItem = (page) => ({ text: page.title, link: toLink(page) });

const groups = [...new Set(pages.map((page) => page.group).filter(Boolean))].map((group) => ({
  text: group,
  collapsed: false,
  items: pages.filter((page) => page.group === group).map(toItem),
}));
const sidebar = [
  ...groups.filter((group) => group.text === "Code Reference"),
  ...pages.filter((page) => !page.group).map(toItem),
  ...groups.filter((group) => group.text !== "Code Reference"),
];

const watchToc = () => ({
  name: "watch-toc",
  configureServer(server) {
    server.watcher.add(tocPath);
    server.watcher.on("change", (file) => {
      if (file === tocPath) server.restart();
    });
  },
});

export default defineConfig({
  title: "Datasources",
  description: "Data-access and object-relational mapper with parity across languages and providers.",
  cleanUrls: true,
  rewrites: {
    "introduction.md": "index.md",
  },
  themeConfig: {
    sidebar,
    outline: "deep",
    socialLinks: [{ icon: "github", link: "https://github.com/deterministic-code/datasources" }],
  },
  markdown: {
    config(md) {
      const render = md.render.bind(md);
      md.render = (src, env) => render(transformDialects(src, "vitepress"), env);
    },
  },
  vite: {
    plugins: [dialectsVitePlugin(), watchToc()],
  },
});
