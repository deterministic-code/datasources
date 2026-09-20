import { blobUrl, readRange, SOURCE_ICON } from "./example.mjs";
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
const EXAMPLE_BLOCK = /^::: example\r?\n([\s\S]*?)^:::[ \t]*$/gm;
const PACKAGES_BLOCK = /^::: packages\r?\n([\s\S]*?)^:::[ \t]*$/gm;
const REFERENCE_BLOCK = /^::: reference\r?\n([\s\S]*?)^:::[ \t]*$/gm;
const FENCE = /^```([a-zA-Z0-9_+-]+)(?:[ \t]+\[([^\]]+)\])?[^\n]*\r?\n([\s\S]*?)^```\s*$/gm;
const EXAMPLE_FENCE = /^```([a-zA-Z0-9_+-]+)[ \t]+(\S+#L\d+-L\d+)\s*\r?\n([\s\S]*?)^```\s*$/gm;
const OUTPUT_FENCE = /^```output(?::([a-zA-Z0-9_+-]+))?[ \t]+\[([^\]]+)\]\s*\r?\n([\s\S]*?)^```\s*$/gm;
const DOC_FENCE = /(^~~~~[\s\S]*?^~~~~)/m;
const PANEL_ORDER = ["typescript", "csharp", "rust"];

export const labelFor = (lang) => LABELS[lang.toLowerCase()] ?? lang;

export const parseFences = (body) => {
  const fences = [];
  for (const match of body.matchAll(FENCE)) {
    fences.push({ lang: match[1], label: match[2], code: match[3].replace(/\n$/, "") });
  }
  return fences;
};

export const parseExampleFences = (body) => {
  EXAMPLE_FENCE.lastIndex = 0;
  const fences = [...body.matchAll(EXAMPLE_FENCE)].map((match) => {
    if (match[3].trim()) throw new Error(`Example fence for ${match[2]} must not contain inline code`);
    return { lang: match[1], spec: match[2] };
  });
  if (!fences.length) throw new Error("::: example has no test ranges");
  return fences;
};

export const parseExampleOutputs = (body) => {
  OUTPUT_FENCE.lastIndex = 0;
  return [...body.matchAll(OUTPUT_FENCE)].map((match) => ({
    lang: match[1] ? slotName(match[1]) : null,
    title: match[2],
    value: match[3].replace(/\n$/, ""),
  }));
};

const outputsFor = (outputs, lang) => {
  const override = new Set(outputs.filter((output) => output.lang === lang).map((output) => output.title));
  return outputs.filter((output) => output.lang === lang || (!output.lang && !override.has(output.title)));
};

const labeledOutputs = (outputs) =>
  outputs.map((output) =>
    output.lang ? { ...output, title: `${output.title} (${labelFor(output.lang)})` } : output,
  );

const escapeAttr = (value) =>
  String(value).replaceAll("&", "&amp;").replaceAll('"', "&quot;").replaceAll("<", "&lt;");

const renderOutputs = (outputs) =>
  outputs.length
    ? `<ExampleOutputs items="${escapeAttr(JSON.stringify(outputs.map(({ title, value }) => ({ title, value }))))}" />`
    : "";

const resolveExamples = (body) =>
  parseExampleFences(body).map(({ lang, spec }) => {
    const snippet = readRange(spec);
    return { lang, code: snippet.code, href: blobUrl(snippet) };
  });

const sourceLink = (href) => `<ExampleSource href="${href}" />`;

const transformOutsideDocFences = (markdown, fn) =>
  markdown.split(DOC_FENCE).map((chunk) => (chunk.startsWith("~~~~") ? chunk : fn(chunk))).join("");

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

const readmeOutput = (outputs) =>
  outputs.map(({ title, value }) => `**${title}**\n\n\`\`\`\n${value}\n\`\`\``).join("\n\n");

const toReadmeExamples = (body) => {
  const outputs = parseExampleOutputs(body);
  return resolveExamples(body)
    .map(({ lang, code, href }) => {
      const boxes = readmeOutput(outputsFor(outputs, slotName(lang)));
      return `#### ${labelFor(lang)} [${SOURCE_ICON}](${href})\n\n\`\`\`${lang}\n${code}\n\`\`\`${boxes ? `\n\n${boxes}` : ""}`;
    })
    .join("\n\n");
};

const toVitePressExamplePanels = (body) => {
  const outputs = parseExampleOutputs(body);
  return resolveExamples(body)
    .map(({ lang, code, href }) => {
      const boxes = renderOutputs(outputsFor(outputs, slotName(lang)));
      return toPanel(slotName(lang), `${sourceLink(href)}\n\n\`\`\`${lang}\n${code}\n\`\`\`${boxes ? `\n\n${boxes}` : ""}`);
    })
    .join("\n\n");
};

const toVitePressExamples = (body) => {
  const resolved = resolveExamples(body);
  const boxes = renderOutputs(labeledOutputs(parseExampleOutputs(body)));
  const group = resolved
    .map(({ lang, code }) => `\`\`\`${lang} [${labelFor(lang)}]\n${code}\n\`\`\``)
    .join("\n\n");
  const links = resolved.map(({ href }) => sourceLink(href)).join("\n");
  return `${links}\n\n::: code-group\n\n${group}\n\n:::${boxes ? `\n\n${boxes}` : ""}`;
};

export const transformDialects = (markdown, style) => {
  const page = hasReference(markdown);
  const dialects =
    style === "vitepress" ? (page ? toVitePressDialectPanels : toVitePressDialects) : toReadmeDialects;
  const examples =
    style === "vitepress" ? (page ? toVitePressExamplePanels : toVitePressExamples) : toReadmeExamples;
  const packages = style === "vitepress" ? toVitePressPackages : toReadmePackages;
  const reference = style === "vitepress" ? toVitePressReference : toReadmeReference;
  const next = transformOutsideDocFences(markdown, (chunk) =>
    chunk
      .replace(EXAMPLE_BLOCK, (_, body) => examples(body))
      .replace(DIALECT_BLOCK, (_, body) => dialects(body))
      .replace(PACKAGES_BLOCK, (_, body) => packages(body))
      .replace(REFERENCE_BLOCK, (_, body) => reference(body)),
  );
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
