import { renderClassReference } from "./reference.mjs";

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

const DIALECT_BLOCK = /^::: dialects\r?\n([\s\S]*?)^:::[ \t]*$/gm;
const PACKAGES_BLOCK = /^::: packages\r?\n([\s\S]*?)^:::[ \t]*$/gm;
const REFERENCE_BLOCK = /^::: reference\r?\n([\s\S]*?)^:::[ \t]*$/gm;
const FENCE = /^```([a-zA-Z0-9_+-]+)(?:[ \t]+\[([^\]]+)\])?[^\n]*\r?\n([\s\S]*?)^```\s*$/gm;
const PANEL_ORDER = ["typescript", "csharp", "rust"];

export const labelFor = (lang) => LABELS[lang.toLowerCase()] ?? lang;

export const parseFences = (body) => {
  const fences = [];
  for (const match of body.matchAll(FENCE)) {
    fences.push({ lang: match[1], label: match[2], code: match[3].replace(/\n$/, "") });
  }
  return fences;
};

const BOX_SEP = /\n---\n/;
const slotName = (lang) =>
  ({ cs: "csharp", csharp: "csharp", ts: "typescript", typescript: "typescript", rs: "rust", rust: "rust" })[
    lang.toLowerCase()
  ] ?? lang.toLowerCase();

const BOX_TITLES = ["Install Core", "Install Providers"];
const boxesOf = (code) => code.split(BOX_SEP).map((box) => box.trim());
const titleOf = (index) => BOX_TITLES[index] ?? `Box ${index + 1}`;

const toGroup = (body, title) => {
  const fences = parseFences(body)
    .map(({ lang, label, code }) => `\`\`\`${lang} [${title({ lang, label })}]\n${code}\n\`\`\``)
    .join("\n\n");
  return `::: code-group\n\n${fences}\n\n:::`;
};

const toReadme = (body, title) =>
  parseFences(body)
    .map(({ lang, label, code }) => {
      const fences = boxesOf(code).map(
        (box, i) =>
          `${BOX_SEP.test(code) ? `${titleOf(i)}\n\n` : ""}\`\`\`${lang}\n${box}\n\`\`\``,
      );
      return `#### ${title({ lang, label })}\n\n${fences.join("\n\n")}`;
    })
    .join("\n\n");

export const toReadmeDialects = (body) => toReadme(body, ({ lang }) => labelFor(lang));
export const toVitePressDialects = (body) => {
  const fences = parseFences(body);
  if (!fences.some(({ code }) => BOX_SEP.test(code))) return toGroup(body, ({ lang }) => labelFor(lang));
  const slots = fences.map(({ lang, code }) => {
    const fences = boxesOf(code).map(
      (box, i) =>
        `<div class="dialect-box">\n\n<p class="dialect-box-title">${titleOf(i)}</p>\n\n\`\`\`${lang}\n${box}\n\`\`\`\n\n</div>`,
    );
    return `<template v-slot:${slotName(lang)}>\n\n${fences.join("\n\n")}\n\n</template>`;
  });
  return `<DialectBoxes>\n\n${slots.join("\n\n")}\n\n</DialectBoxes>`;
};
export const toReadmePackages = (body) => toReadme(body, ({ lang, label }) => label ?? lang);
export const toVitePressPackages = (body) => toGroup(body, ({ lang, label }) => label ?? lang);

export const parseReferencePanels = (body) => {
  const marker = /^#{3,4}[ \t]+(typescript|ts|csharp|cs|rust|rs)[ \t]*$/gim;
  return [...body.matchAll(marker)].map((match, i, matches) => {
    const start = match.index + match[0].length;
    const end = i + 1 < matches.length ? matches[i + 1].index : body.length;
    return { lang: slotName(match[1]), content: body.slice(start, end).trim() };
  });
};

const orderedPanels = (body) => {
  const panels = parseReferencePanels(body);
  return PANEL_ORDER.map((id) => panels.find((panel) => panel.lang === id)).filter(Boolean);
};

const toPanel = (name, content) => `<DialectPanel name="${name}">\n\n${content}\n\n</DialectPanel>`;

export const toVitePressReference = (body) =>
  orderedPanels(body)
    .map(({ lang, content }) => toPanel(lang, renderClassReference(content, lang)))
    .join("\n\n");

export const toVitePressDialectPanels = (body) => {
  const fences = parseFences(body);
  return fences
    .map(({ lang, code }) => {
      const boxes = boxesOf(code).map(
        (box, i) =>
          `${BOX_SEP.test(code) ? `<p class="dialect-box-title">${titleOf(i)}</p>\n\n` : ""}\`\`\`${lang}\n${box}\n\`\`\``,
      );
      return toPanel(slotName(lang), boxes.join("\n\n"));
    })
    .join("\n\n");
};

export const toReadmeReference = (body) =>
  orderedPanels(body)
    .map(({ lang, content }) => `#### ${labelFor(lang)}\n\n${renderClassReference(content, lang)}`)
    .join("\n\n");

const hasReference = (markdown) => /^::: reference\r?\n/m.test(markdown);

export const transformDialects = (markdown, style) => {
  const page = hasReference(markdown);
  const dialects =
    style === "vitepress" ? (page ? toVitePressDialectPanels : toVitePressDialects) : toReadmeDialects;
  const packages = style === "vitepress" ? toVitePressPackages : toReadmePackages;
  const reference = style === "vitepress" ? toVitePressReference : toReadmeReference;
  const next = markdown
    .replace(DIALECT_BLOCK, (_, body) => dialects(body))
    .replace(PACKAGES_BLOCK, (_, body) => packages(body))
    .replace(REFERENCE_BLOCK, (_, body) => reference(body));
  return style === "vitepress" && page
    ? next.replace(/^## Classes/m, "<DialectSelector />\n\n## Classes")
    : next;
};

export const dialectsVitePlugin = () => ({
  name: "dialects",
  enforce: "pre",
  transform(code, id) {
    if (!id.split("?")[0].endsWith(".md")) return;
    return { code: transformDialects(code, "vitepress"), map: null };
  },
});
