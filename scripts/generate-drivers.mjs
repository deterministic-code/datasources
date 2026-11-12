import { createRequire } from "node:module";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
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
const conversionsPath = join(root, "docs/DriverConversions.yaml");
const outPath = join(root, "docs/drivers.md");
const banner =
  "<!-- Generated from docs/DriverSummary.yaml and docs/DriverConversions.yaml. Run npm run docs:drivers. -->\n\n";

const API = {
  csharp: {
    driver: "`IDriver`",
    information: "Information",
    typeConverter: "TypeConverter",
    converters: "RowTypeConverters",
    defaultValueConverter: "DefaultValueConverter",
    changeLogs: "ChangeLogs",
  },
  rust: {
    driver: "`Driver`",
    information: "information",
    typeConverter: "type_converter",
    converters: "row_type_converters",
    defaultValueConverter: "default_value_converter",
    changeLogs: "change_logs",
  },
  typescript: {
    driver: "`IDriver`",
    information: "information",
    typeConverter: "typeConverter",
    converters: "rowTypeConverters",
    defaultValueConverter: "defaultValueConverter",
    changeLogs: "changeLogs",
  },
};

const list = (items, empty = "_None._") =>
  items?.length ? items.map((item) => `- ${item}`).join("\n") : empty;

const fence = (text) => `\`${String(text).replaceAll("|", "\\|")}\``;

const typeFromToTable = (rows, typeHeader, empty) => {
  if (!rows?.length) return empty;
  return [
    `| ${typeHeader} | From | To |`,
    "|---|---|---|",
    ...rows.map((row) => `| ${fence(row.type)} | ${fence(row.from)} | ${fence(row.to)} |`),
  ].join("\n");
};

const tile = (href, logo, title) => `  <a class="driver-tile" href="${href}">
    <img src="${logo}" alt="" width="48" height="48" />
    <span>${title}</span>
  </a>`;

const driverSection = (language, driver, conversions) => {
  const api = API[language.id];
  const anchor = `${language.id}-${driver.id}`;
  const tables = conversions[driver.id] ?? {};
  return `### ${driver.title} {#${anchor}}

<a href="#${language.id}" class="driver-back">← ${language.title} drivers</a>

#### ${api.information}

${list(driver.descriptions)}

#### Overrides

${list(driver.overrides, "_No statement or dialect overrides._")}

#### ${api.typeConverter}

${typeFromToTable(tables.typeConverter, "Type", "_None._")}

#### ${api.converters}

${typeFromToTable(tables.rowTypeConverters, "Type", "_None — cells are already canonical._")}

#### ${api.defaultValueConverter}

${typeFromToTable(tables.defaultValueConverter, "Column type", "_None._")}

#### ${api.changeLogs}

${list(driver.changeLogs, "_None._")}
`;
};

const languageSection = (language, drivers, conversions) => {
  const api = API[language.id];
  if (!api) throw new Error(`DriverSummary.yaml: unknown language "${language.id}"`);

  const header = drivers
    .map((driver) => tile(`#${language.id}-${driver.id}`, driver.logo, driver.title))
    .join("\n");

  return `## ${language.title} {#${language.id}}

<a href="#languages" class="driver-back">← Languages and Datasource Providers</a>

Drivers for ${language.title}. Click a tile for ${api.information}, Overrides, ${api.typeConverter}, ${api.converters}, and ${api.defaultValueConverter} from that ${api.driver}.

<div class="driver-header">
${header}
</div>

${drivers.map((driver) => driverSection(language, driver, conversions)).join("\n")}`;
};

const generate = () => {
  const { languages, drivers } = parse(readFileSync(summaryPath, "utf8"));
  if (!Array.isArray(languages) || languages.length === 0) {
    throw new Error("DriverSummary.yaml: expected a non-empty languages list");
  }
  if (!Array.isArray(drivers) || drivers.length === 0) {
    throw new Error("DriverSummary.yaml: expected a non-empty drivers list");
  }
  if (!existsSync(conversionsPath)) {
    throw new Error("DriverConversions.yaml is missing. Run npm run docs:drivers.");
  }
  const conversions = parse(readFileSync(conversionsPath, "utf8")) ?? {};

  const languageHeader = languages
    .map((language) => tile(`#${language.id}`, language.logo, language.title))
    .join("\n");

  const body = `# Languages and Datasource Providers {#languages}

Pick a language, then a driver. Each driver section lists {{Information|information|information}}, Overrides, {{TypeConverter|typeConverter|type_converter}}, {{RowTypeConverters|rowTypeConverters|row_type_converters}}, and {{DefaultValueConverter|defaultValueConverter|default_value_converter}} from that language's {{IDriver|IDriver|Driver}}.

<div class="driver-header">
${languageHeader}
</div>

${languages.map((language) => languageSection(language, drivers, conversions)).join("\n")}`;

  return `${banner}${body.trim()}\n`;
};

const next = generate();
if (process.argv.includes("--check")) {
  const current = existsSync(outPath) ? readFileSync(outPath, "utf8") : "";
  if (current !== next) {
    console.error("docs/drivers.md is stale. Run npm run docs:drivers.");
    process.exit(1);
  }
  process.exit(0);
}

writeFileSync(outPath, next);
console.log("Wrote docs/drivers.md");
