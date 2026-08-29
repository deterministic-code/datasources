/** @typedef {{ lang: string, code: string }} DialectFence */

const LABELS = {
  csharp: "C#",
  cs: "C#",
  rust: "Rust",
  rs: "Rust",
  typescript: "TypeScript",
  ts: "TypeScript",
  javascript: "JavaScript",
  js: "JavaScript",
};

const DIALECT_BLOCK = /^::: dialects\r?\n([\s\S]*?)^:::\s*$/gm;
const FENCE = /^```([a-zA-Z0-9_+-]+)[^\n]*\r?\n([\s\S]*?)^```\s*$/gm;

export const labelFor = (lang) => LABELS[lang.toLowerCase()] ?? lang;

export const parseFences = (body) => {
  const fences = [];
  for (const match of body.matchAll(FENCE)) {
    fences.push({ lang: match[1], code: match[2].replace(/\n$/, "") });
  }
  return fences;
};

export const toReadmeDialects = (body) =>
  parseFences(body)
    .map(({ lang, code }) => `#### ${labelFor(lang)}\n\n\`\`\`${lang}\n${code}\n\`\`\``)
    .join("\n\n");

export const toVitePressDialects = (body) => {
  const fences = parseFences(body)
    .map(({ lang, code }) => `\`\`\`${lang} [${labelFor(lang)}]\n${code}\n\`\`\``)
    .join("\n\n");
  return `::: code-group\n\n${fences}\n\n:::`;
};

export const transformDialects = (markdown, style) => {
  const replace = style === "vitepress" ? toVitePressDialects : toReadmeDialects;
  return markdown.replace(DIALECT_BLOCK, (_, body) => replace(body));
};

export const dialectsVitePlugin = () => ({
  name: "dialects",
  enforce: "pre",
  transform(code, id) {
    if (!id.split("?")[0].endsWith(".md")) return;
    return { code: transformDialects(code, "vitepress"), map: null };
  },
});
