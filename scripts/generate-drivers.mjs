import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const parseOf = (pkg) => {
  try {
    return createRequire(pkg)("yaml").parse;
  } catch {
    return undefined;
  }
};
const parse =
  parseOf(join(root, "package.json")) ?? parseOf(join(root, "typescript/package.json"));
if (parse === undefined) throw new Error("Cannot find module 'yaml'");

const summaryPath = join(root, "docs/DriverSummary.yaml");
const outPath = join(root, "docs/drivers.md");
const banner =
  "<!-- Generated from docs/DriverSummary.yaml. Edit that file and run npm run docs:drivers. -->\n\n";

const API = {
  csharp: {
    driver: "`IDriver`",
    information: "Information",
    converters: "RowTypeConverters",
    changeLogs: "ChangeLogs",
  },
  rust: {
    driver: "`Driver`",
    information: "information",
    converters: "row_type_converters",
    changeLogs: "change_logs",
  },
  typescript: {
    driver: "`IDriver`",
    information: "information",
    converters: "rowTypeConverters",
    changeLogs: "changeLogs",
  },
};

const list = (items, empty = "_None._") =>
  items?.length ? items.map((item) => `- ${item}`).join("\n") : empty;

const tile = (href, logo, title) => `  <a class="driver-tile" href="${href}">
    <img src="${logo}" alt="" width="48" height="48" />
    <span>${title}</span>
  </a>`;

const driverSection = (language, driver) => {
  const api = API[language.id];
  const anchor = `${language.id}-${driver.id}`;
  const converters = list(
    (driver.rowTypeConverters ?? []).map((name) => `\`${name}\``),
    "_None — cells are already canonical._",
  );
  return `### ${driver.title} {#${anchor}}

<a href="#${language.id}" class="driver-back">← ${language.title} drivers</a>

#### ${api.information}

${list(driver.descriptions)}

#### Overrides

${list(driver.overrides, "_No statement or dialect overrides._")}

#### ${api.converters}

${converters}

#### ${api.changeLogs}

${list(driver.changeLogs, "_None._")}
`;
};

const languageSection = (language, drivers) => {
  const api = API[language.id];
  if (!api) throw new Error(`DriverSummary.yaml: unknown language "${language.id}"`);

  const header = drivers
    .map((driver) => tile(`#${language.id}-${driver.id}`, driver.logo, driver.title))
    .join("\n");

  return `## ${language.title} {#${language.id}}

<a href="#languages" class="driver-back">← Languages and Datasource Providers</a>

Drivers for ${language.title}. Click a tile for ${api.information}, Overrides, and ${api.converters} from that ${api.driver}.

<div class="driver-header">
${header}
</div>

${drivers.map((driver) => driverSection(language, driver)).join("\n")}`;
};

const generate = () => {
  const { languages, drivers } = parse(readFileSync(summaryPath, "utf8"));
  if (!Array.isArray(languages) || languages.length === 0) {
    throw new Error("DriverSummary.yaml: expected a non-empty languages list");
  }
  if (!Array.isArray(drivers) || drivers.length === 0) {
    throw new Error("DriverSummary.yaml: expected a non-empty drivers list");
  }

  const languageHeader = languages
    .map((language) => tile(`#${language.id}`, language.logo, language.title))
    .join("\n");

  const body = `# Languages and Datasource Providers {#languages}

Pick a language, then a driver. Each driver section lists {{Information|information|information}}, Overrides, and {{RowTypeConverters|rowTypeConverters|row_type_converters}} from that language's {{IDriver|IDriver|Driver}}.

<div class="driver-header">
${languageHeader}
</div>

${languages.map((language) => languageSection(language, drivers)).join("\n")}`;

  return `${banner}${body.trim()}\n`;
};

const next = generate();
if (process.argv.includes("--check")) {
  const current = readFileSync(outPath, "utf8");
  if (current !== next) {
    console.error("docs/drivers.md is stale. Run npm run docs:drivers.");
    process.exit(1);
  }
  process.exit(0);
}

writeFileSync(outPath, next);
console.log("Wrote docs/drivers.md");
