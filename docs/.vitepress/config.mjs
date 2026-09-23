import { readFileSync, utimesSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitepress";
import { dialectsVitePlugin, transformDialects } from "../../scripts/dialects.mjs";
import { nestSidebar, parseToc, sidebarTitle } from "../../scripts/toc.mjs";

const configPath = fileURLToPath(import.meta.url);
const docsDir = join(dirname(configPath), "..");
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
  ...pages.filter((page) => !page.group).map(toItem),
  ...groups.filter((group) => group.text === "Code Reference"),
  ...groups.filter((group) => group.text !== "Code Reference"),
];

const redirects = {
  "/sections/flat-to-type-mapper": "/sections/flat-to-nested-mapper",
  "/sections/builders": "/sections/statements",
  "/sections/where-builder": "/sections/where-statement",
  "/sections/stored-procedure-statement": "/sections/execute-stored-procedure-statement",
  "/sections/crud-mapper": "/sections/crud-repository#crudrepositoryoptions",
};

const watchToc = () => ({
  name: "watch-toc",
  configureServer(server) {
    server.watcher.add(tocPath);
    server.watcher.on("change", (file) => {
      if (file === tocPath || file.endsWith("toc.yaml"))
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
      md.core.ruler.before("normalize", "dialects", (state) => {
        state.src = transformDialects(state.src, "vitepress");
      });
    },
  },
  vite: {
    plugins: [dialectsVitePlugin(), watchToc()],
  },
});
