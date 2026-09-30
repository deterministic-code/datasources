import { blobUrl, sourceHref, SOURCE_ICON } from "./example.mjs";
import { renderClassReference } from "./reference.mjs";
import { schemaText } from "./schema-sql.mjs";
import { readSnippet } from "./sync-examples.mjs";

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
const PROSE_BLOCK = /^::: prose\r?\n([\s\S]*?)^:::[ \t]*$/gm;
const CODE_FENCE = /(^```[\s\S]*?^```)/m;
const NAME_MARKER = /\{\{([^{}|]+)\|([^{}|]+)\|([^{}|]+)\}\}/g;
const FENCE = /^```([a-zA-Z0-9_+-]+)(?:[ \t]+\[([^\]]+)\])?[^\n]*\r?\n([\s\S]*?)^```\s*$/gm;
const EXAMPLE_FENCE = /^```([a-zA-Z0-9_+-]+)[ \t]+(\S+#\S+)\s*\r?\n([\s\S]*?)^```\s*$/gm;
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
  if (!fences.length) throw new Error("::: example has no sample sources");
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

const schemaOf = (resolved, heading) => {
  const names = [...new Set(resolved.map((item) => item.schema).filter(Boolean))];
  if (names.length > 1) throw new Error(`${heading} schemas differ: ${names.join(", ")}`);
  return names[0] ?? null;
};

const renderSchema = (name) =>
  name
    ? `<ExampleSchema items="${escapeAttr(JSON.stringify([{ name, href: `/examples#${name}`, sql: schemaText(name) }]))}" />\n\n`
    : "";

const readmeSchema = (name) =>
  name ? `Schema: [${name}](../examples.md#${name})\n\n\`\`\`sql\n${schemaText(name)}\n\`\`\`\n\n` : "";

const lastHeading = (prefix) => {
  let heading = "";
  for (const line of prefix.split("\n")) {
    const match = line.match(/^###\s+(.+?)\s*$/);
    if (match) heading = match[1];
  }
  return heading;
};

const resolveExamples = (body, heading, files) =>
  parseExampleFences(body).map(({ lang, spec }) => {
    const snippet = readSnippet(heading, lang, spec);
    if (files) {
      files.push(snippet.file);
      if (!files.includes(snippet.linesFile)) files.push(snippet.linesFile);
    }
    return { lang, code: snippet.body, href: blobUrl(snippet), source: sourceHref(snippet), schema: snippet.schema };
  });

const sourceLink = (href) => `<ExampleSource href="${href.replaceAll("&", "&amp;")}" />`;

const groupedExamples = (resolved) => {
  const groups = [];
  for (const item of resolved) {
    const lang = slotName(item.lang);
    const last = groups.at(-1);
    if (last?.lang === lang) {
      last.code = `${last.code}\n\n${item.code}`;
      last.hrefs.push(item.href);
      last.sources.push(item.source);
    } else groups.push({ lang, code: item.code, hrefs: [item.href], sources: [item.source] });
  }
  return groups;
};

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

export const toVitePressProse = (body) =>
  orderedPanels(body)
    .map(({ lang, content }) => toPanel(lang, content))
    .join("\n\n");

export const toReadmeProse = (body) =>
  orderedPanels(body)
    .map(({ lang, content }) => {
      const quoted = content
        .split("\n")
        .map((line) => (line.length ? `> ${line}` : ">"))
        .join("\n");
      return `> **${labelFor(lang)}**\n>\n${quoted}`;
    })
    .join("\n\n");

const applyNames = (text, style) => {
  NAME_MARKER.lastIndex = 0;
  return text.replace(NAME_MARKER, (_, csharp, typescript, rust) => {
    const slots = [csharp, typescript, rust].map((slot) => slot.trim());
    return style === "vitepress"
      ? `<DialectName csharp="${escapeAttr(slots[0])}" typescript="${escapeAttr(slots[1])}" rust="${escapeAttr(slots[2])}"></DialectName>`
      : slots.map((slot) => `\`${slot}\``).join(" / ");
  });
};

const transformOutsideCodeFences = (markdown, fn) =>
  markdown.split(CODE_FENCE).map((chunk) => (chunk.startsWith("```") ? chunk : fn(chunk))).join("");

const outsideFences = (markdown) =>
  markdown
    .split(DOC_FENCE)
    .filter((chunk) => !chunk.startsWith("~~~~"))
    .join("\n")
    .split(CODE_FENCE)
    .filter((chunk) => !chunk.startsWith("```"))
    .join("\n");

const hasReference = (markdown) => /^::: reference\r?\n/m.test(outsideFences(markdown));
const hasProse = (markdown) => /^::: prose\r?\n/m.test(outsideFences(markdown));
const hasNames = (markdown) => {
  NAME_MARKER.lastIndex = 0;
  return NAME_MARKER.test(outsideFences(markdown));
};

const withSelector = (markdown) =>
  transformOutsideDocFences(markdown, (chunk) => {
    if (chunk.includes("<DialectSelector")) return chunk;
    if (/^## Classes/m.test(chunk))
      return chunk.replace(/^## Classes/m, "<DialectSelector />\n\n## Classes");
    return chunk.replace(/^(# [^\n]+)\r?\n/, "$1\n\n<DialectSelector />\n");
  });

const readmeOutput = (outputs) =>
  outputs.map(({ title, value }) => `**${title}**\n\n\`\`\`\n${value}\n\`\`\``).join("\n\n");

const toReadmeExamples = (body, heading, files) => {
  const outputs = parseExampleOutputs(body);
  const resolved = resolveExamples(body, heading, files);
  const schema = readmeSchema(schemaOf(resolved, heading));
  const samples = groupedExamples(resolved)
    .map(({ lang, code, hrefs }) => {
      const boxes = readmeOutput(outputsFor(outputs, lang));
      const links = hrefs.map((href) => `[${SOURCE_ICON}](${href})`).join(" ");
      return `#### ${labelFor(lang)} ${links}\n\n\`\`\`${lang}\n${code}\n\`\`\`${boxes ? `\n\n${boxes}` : ""}`;
    })
    .join("\n\n");
  return `${samples}${schema ? `\n\n${schema.trimEnd()}` : ""}`;
};

const toVitePressExamplePanels = (body, heading, files) => {
  const outputs = parseExampleOutputs(body);
  const resolved = resolveExamples(body, heading, files);
  const panels = groupedExamples(resolved)
    .map(({ lang, code, sources }) => {
      const boxes = renderOutputs(outputsFor(outputs, lang));
      const links = sources.map(sourceLink).join("\n");
      return toPanel(lang, `${links}\n\n\`\`\`${lang}\n${code}\n\`\`\`${boxes ? `\n\n${boxes}` : ""}`);
    })
    .join("\n\n");
  const schema = renderSchema(schemaOf(resolved, heading));
  return `${panels}${schema ? `\n\n${schema.trimEnd()}` : ""}`;
};

const toVitePressExamples = (body, heading, files) => {
  const loaded = resolveExamples(body, heading, files);
  const resolved = groupedExamples(loaded);
  const boxes = renderOutputs(labeledOutputs(parseExampleOutputs(body)));
  const group = resolved
    .map(({ lang, code }) => `\`\`\`${lang} [${labelFor(lang)}]\n${code}\n\`\`\``)
    .join("\n\n");
  const links = resolved.flatMap(({ sources }) => sources.map(sourceLink)).join("\n");
  const schema = renderSchema(schemaOf(loaded, heading));
  return `${links}\n\n::: code-group\n\n${group}\n\n:::${schema ? `\n\n${schema.trimEnd()}` : ""}${boxes ? `\n\n${boxes}` : ""}`;
};

export const transformDialects = (markdown, style, files) => {
  const page = hasReference(markdown);
  const dialects =
    style === "vitepress" ? (page ? toVitePressDialectPanels : toVitePressDialects) : toReadmeDialects;
  const examples =
    style === "vitepress" ? (page ? toVitePressExamplePanels : toVitePressExamples) : toReadmeExamples;
  const packages = style === "vitepress" ? toVitePressPackages : toReadmePackages;
  const reference = style === "vitepress" ? toVitePressReference : toReadmeReference;
  const prose = style === "vitepress" ? toVitePressProse : toReadmeProse;
  const showSelector = page || hasProse(markdown) || hasNames(markdown);
  const next = transformOutsideDocFences(markdown, (chunk) =>
    transformOutsideCodeFences(
      chunk
        .replace(PROSE_BLOCK, (_, body) => prose(body))
        .replace(EXAMPLE_BLOCK, (match, body, offset, text) =>
          examples(body, lastHeading(text.slice(0, offset)), files),
        )
        .replace(DIALECT_BLOCK, (_, body) => dialects(body))
        .replace(PACKAGES_BLOCK, (_, body) => packages(body))
        .replace(REFERENCE_BLOCK, (_, body) => reference(body)),
      (text) => applyNames(text, style),
    ),
  );
  return style === "vitepress" && showSelector ? withSelector(next) : next;
};

export const dialectsVitePlugin = () => ({
  name: "dialects",
  enforce: "pre",
  transform(code, id) {
    if (!id.split("?")[0].endsWith(".md")) return;
    const files = [];
    const next = transformDialects(code, "vitepress", files);
    for (const file of files) this.addWatchFile(file);
    return { code: next, map: null };
  },
});
