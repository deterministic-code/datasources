import { readFileSync, utimesSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitepress";
import { withMermaid } from "vitepress-plugin-mermaid";
import { dialectsVitePlugin, transformDialects } from "../../scripts/dialects.mjs";
import { viewSourceMiddleware } from "../../scripts/view-source.mjs";
import { nestSidebar, parseToc, sidebarTitle } from "../../scripts/toc.mjs";
import { fontSizeBoot } from "./theme/fontSize.js";

const configPath = fileURLToPath(import.meta.url);
const docsDir = join(dirname(configPath), "..");
const docsBase = process.env.DOCS_BASE;
const tocPath = join(docsDir, "toc.yaml");
const { pages } = parseToc(readFileSync(tocPath, "utf8"));

const toLink = ({ path, hash }) =>
  `${path === "introduction.md" ? "/" : `/${path.replace(/\.md$/, "")}`}${hash ? `#${hash}` : ""}`;

const toItem = (page) => ({ text: sidebarTitle(page.title), link: toLink(page) });

const groups = [...new Set(pages.map((page) => page.group).filter(Boolean))].map((group) => ({
  text: group,
  collapsed: false,
  items: nestSidebar(
    pages.filter((page) => page.group === group),
    toItem,
  ),
}));
const sidebar = [
  ...nestSidebar(
    pages.filter((page) => !page.group),
    toItem,
  ),
  ...groups.filter((group) => group.text === "Articles"),
  ...groups.filter((group) => group.text === "Code Reference"),
  ...groups.filter((group) => group.text !== "Articles" && group.text !== "Code Reference"),
];

const redirects = {
  "/sections/flat-to-type-mapper": "/sections/flat-to-nested-mapper",
  "/sections/builders": "/sections/statements",
  "/sections/where-builder": "/sections/where-statement",
  "/sections/stored-procedure-statement": "/sections/execute-stored-procedure-statement",
  "/sections/crud-mapper": "/sections/crud-repository#crudrepositoryoptions",
  "/sections/enriched-repository": "/sections/readonly-repository",
};

const watchToc = () => ({
  name: "watch-toc",
  configureServer(server) {
    server.middlewares.use(viewSourceMiddleware);
    server.watcher.add(tocPath);
    server.watcher.on("all", (event, file) => {
      if (event !== "change" && event !== "add") return;
      if (resolve(file) !== resolve(tocPath)) return;
      utimesSync(configPath, new Date(), new Date());
    });
    server.middlewares.use((req, res, next) => {
      const path = req.url?.split("?")[0]?.replace(/\/$/, "") ?? "";
      const to = redirects[path];
      if (!to) return next();
      res.statusCode = 301;
      res.setHeader("Location", to);
      res.end();
    });
  },
  configurePreviewServer(server) {
    server.middlewares.use(viewSourceMiddleware);
  },
});

export default withMermaid(defineConfig({
  title: "Datasources",
  description: "Data-access and object-relational mapper with parity across languages and providers.",
  base: docsBase ?? "/",
  cleanUrls: !docsBase,
  head: [["script", {}, fontSizeBoot]],
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
      md.core.ruler.before("normalize", "dialects", (state) => {
        state.src = transformDialects(state.src, "vitepress");
      });
    },
  },
  vite: {
    server: { port: 5174 },
    plugins: [dialectsVitePlugin(), watchToc()],
    optimizeDeps: {
      include: ["mermaid", "fastdom"],
    },
  },
}));
