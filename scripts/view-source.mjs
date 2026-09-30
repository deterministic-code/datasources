import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");

const esc = (value) => value.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");

const inside = (base, path) => {
  if (!path || path.startsWith("/") || path.includes("\\") || path.split("/").includes("..")) return null;
  const file = resolve(base, path);
  const rel = relative(base, file);
  if (!rel || rel === ".." || rel.startsWith(`..${sep}`)) return null;
  return file;
};

export const sourcePage = (url, base = root) => {
  const query = new URL(url, "http://local").searchParams;
  const path = query.get("path") ?? "";
  const file = inside(base, path);
  if (!file) return null;
  let text;
  try {
    text = readFileSync(file, "utf8");
  } catch {
    return null;
  }
  const from = Number(query.get("from"));
  const to = Number(query.get("to"));
  const rows = text
    .replace(/\n$/, "")
    .split("\n")
    .map((line, i) => {
      const n = i + 1;
      const hit = n >= from && n <= to;
      return `<tr id="L${n}"${hit ? ' class="hit"' : ""}><td>${n}</td><td>${esc(line) || " "}</td></tr>`;
    })
    .join("");
  const jump = Number.isInteger(from) ? `<script>document.getElementById("L${from}")?.scrollIntoView()</script>` : "";
  return `<!doctype html>
<html lang="en">
<meta charset="utf-8">
<title>${esc(path)}</title>
<style>
  body { margin: 0; font: 13px/1.5 ui-monospace, SFMono-Regular, Menlo, monospace; }
  h1 { margin: 0; padding: 12px 16px; font: 600 13px/1.4 ui-sans-serif, system-ui, sans-serif; border-bottom: 1px solid #ddd; }
  table { border-collapse: collapse; width: 100%; }
  td { padding: 0 12px; white-space: pre; vertical-align: top; }
  td:first-child { color: #888; text-align: right; user-select: none; width: 1%; }
  .hit { background: #fff3bf; }
  @media (prefers-color-scheme: dark) {
    body { background: #1b1b1f; color: #e4e4e7; }
    h1 { border-color: #333; }
    .hit { background: #3d3416; }
  }
</style>
<h1>${esc(path)}</h1>
<table>${rows}</table>
${jump}`;
};

export const viewSourceMiddleware = (req, res, next) => {
  const url = req.url ?? "";
  if (url.split("?")[0] !== "/view-source") return next();
  const html = sourcePage(url);
  if (!html) {
    res.statusCode = 404;
    res.end("Source not found");
    return;
  }
  res.setHeader("content-type", "text/html; charset=utf-8");
  res.end(html);
};
