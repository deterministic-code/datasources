/** @typedef {{ lang: string, label?: string, code: string }} DialectFence */

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
const PACKAGES_BLOCK = /^::: packages\r?\n([\s\S]*?)^:::\s*$/gm;
const FENCE = /^```([a-zA-Z0-9_+-]+)(?:[ \t]+\[([^\]]+)\])?[^\n]*\r?\n([\s\S]*?)^```\s*$/gm;

export const labelFor = (lang) => LABELS[lang.toLowerCase()] ?? lang;

export const parseFences = (body) => {
  const fences = [];
  for (const match of body.matchAll(FENCE)) {
    fences.push({ lang: match[1], label: match[2], code: match[3].replace(/\n$/, "") });
  }
  return fences;
};

const toGroup = (body, title) => {
  const fences = parseFences(body)
    .map(({ lang, label, code }) => `\`\`\`${lang} [${title({ lang, label })}]\n${code}\n\`\`\``)
    .join("\n\n");
  return `::: code-group\n\n${fences}\n\n:::`;
};

const toReadme = (body, title) =>
  parseFences(body)
    .map(({ lang, label, code }) => `#### ${title({ lang, label })}\n\n\`\`\`${lang}\n${code}\n\`\`\``)
    .join("\n\n");

export const toReadmeDialects = (body) => toReadme(body, ({ lang }) => labelFor(lang));
export const toVitePressDialects = (body) => toGroup(body, ({ lang }) => labelFor(lang));
export const toReadmePackages = (body) => toReadme(body, ({ lang, label }) => label ?? lang);
export const toVitePressPackages = (body) => toGroup(body, ({ lang, label }) => label ?? lang);

export const transformDialects = (markdown, style) => {
  const dialects = style === "vitepress" ? toVitePressDialects : toReadmeDialects;
  const packages = style === "vitepress" ? toVitePressPackages : toReadmePackages;
  return markdown
    .replace(DIALECT_BLOCK, (_, body) => dialects(body))
    .replace(PACKAGES_BLOCK, (_, body) => packages(body));
};

export const dialectsVitePlugin = () => ({
  name: "dialects",
  enforce: "pre",
  transform(code, id) {
    if (!id.split("?")[0].endsWith(".md")) return;
    return { code: transformDialects(code, "vitepress"), map: null };
  },
});
