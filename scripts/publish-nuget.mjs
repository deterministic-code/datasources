import { mkdtemp, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const repo = resolve(import.meta.dirname, "..");
const csharp = join(repo, "csharp");
const artifacts = join(csharp, "artifacts");
const argv = process.argv.slice(2);
const doPublish = argv.includes("--publish");
const versionAt = argv.indexOf("--version");
const stamp = versionAt >= 0 ? argv[versionAt + 1] : undefined;
if (versionAt >= 0 && !stamp) {
  console.error("usage: npm run publish:nuget -- [--publish] [--version x.y.z]");
  process.exit(1);
}

const libraries = [
  { id: "Datasources", project: "Datasources/Datasources.csproj" },
  { id: "Datasources.Json", project: "Providers/Json/Datasources.Json.csproj" },
  { id: "Datasources.Memory", project: "Providers/Memory/Datasources.Memory.csproj" },
  { id: "Datasources.Sqlite", project: "Providers/Sqlite/Datasources.Sqlite.csproj" },
  { id: "Datasources.Xml", project: "Providers/Xml/Datasources.Xml.csproj" },
  { id: "Datasources.Yaml", project: "Providers/Yaml/Datasources.Yaml.csproj" },
  { id: "Datasources.Mysql", project: "Providers/Mysql/Datasources.Mysql.csproj" },
  { id: "Datasources.SqlServer", project: "Providers/SqlServer/Datasources.SqlServer.csproj" },
  { id: "Datasources.Postgres", project: "Providers/Postgres/Datasources.Postgres.csproj" },
  { id: "Datasources.Oracle", project: "Providers/Oracle/Datasources.Oracle.csproj" },
  { id: "Datasources.MongoDb", project: "Providers/MongoDb/Datasources.MongoDb.csproj" },
  { id: "Datasources.DynamoDb", project: "Providers/DynamoDb/Datasources.DynamoDb.csproj" },
];

const ours = "https://github.com/deterministic-code/datasources";

const run = (command, args, cwd, opts = {}) => {
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    stdio: opts.capture ? ["ignore", "pipe", "pipe"] : "inherit",
  });
  if (result.status !== 0 && !opts.allowFail) process.exit(result.status ?? 1);
  return result;
};

const version = () => {
  if (stamp) return stamp;
  const props = spawnSync(
    "dotnet",
    ["msbuild", "Datasources/Datasources.csproj", "-getProperty:Version", "-nologo"],
    { cwd: csharp, encoding: "utf8" },
  );
  if (props.status !== 0) process.exit(props.status ?? 1);
  return props.stdout.trim();
};

const nugetJson = async (url) => {
  const response = await fetch(url);
  if (response.status === 404) return null;
  if (!response.ok) throw new Error(`${url}: ${response.status}`);
  return response.json();
};

const published = async (id, ver) => {
  const lower = id.toLowerCase();
  const index = await nugetJson(`https://api.nuget.org/v3-flatcontainer/${lower}/index.json`);
  return Boolean(index?.versions?.includes(ver));
};

const foreignOwner = async () => {
  const catalog = await nugetJson(
    "https://api.nuget.org/v3/registration5-semver1/datasources/index.json",
  );
  if (!catalog) return false;
  const items = catalog.items ?? [];
  const pages = items.flatMap((page) => page.items ?? [page]);
  const urls = pages.flatMap((item) => {
    const catalogEntry = item.catalogEntry ?? item;
    const repoUrl = catalogEntry.repository ?? catalogEntry.projectUrl ?? "";
    return [String(repoUrl)];
  });
  if (urls.length === 0) return true;
  return !urls.some((url) => url.toLowerCase().includes("deterministic-code/datasources"));
};

const pack = (ver) => {
  for (const library of libraries) {
    const args = ["pack", library.project, "-c", "Release", "--no-restore"];
    if (stamp) args.push(`-p:Version=${stamp}`);
    console.log(`pack ${library.id} ${ver}`);
    run("dotnet", ["restore", library.project], csharp);
    run("dotnet", args, csharp);
  }
};

const nupkg = (id, ver) => join(artifacts, `${id}.${ver}.nupkg`);

const smoke = async (ver) => {
  const dir = await mkdtemp(join(tmpdir(), "datasources-nuget-"));
  try {
    run("dotnet", ["new", "classlib", "-n", "Smoke", "-o", ".", "--framework", "net10.0", "--force"], dir);
    await writeFile(
      join(dir, "nuget.config"),
      `<?xml version="1.0" encoding="utf-8"?>
<configuration>
  <packageSources>
    <clear />
    <add key="datasources-local" value="${artifacts}" />
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" />
  </packageSources>
</configuration>
`,
    );
    for (const id of ["Datasources", "Datasources.Sqlite"]) {
      run("dotnet", ["add", "package", id, "--version", ver, "--source", artifacts], dir);
    }
    run("dotnet", ["build"], dir);
    console.log(`smoke ok (${dir})`);
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
};

const push = async (ver) => {
  const key = process.env.NUGET_API_KEY;
  if (!key) {
    console.error("NUGET_API_KEY is not set; stop before push");
    process.exit(1);
  }
  for (const library of libraries) {
    if (await published(library.id, ver)) {
      console.log(`skip ${library.id}@${ver} (already on nuget.org)`);
      continue;
    }
    const file = nupkg(library.id, ver);
    console.log(`push ${library.id}@${ver}`);
    const result = run(
      "dotnet",
      [
        "nuget",
        "push",
        file,
        "--source",
        "https://api.nuget.org/v3/index.json",
        "--api-key",
        key,
        "--skip-duplicate",
      ],
      repo,
      { capture: true, allowFail: true },
    );
    if (result.status !== 0) {
      process.stdout.write(result.stdout);
      process.stderr.write(result.stderr);
      process.exit(result.status ?? 1);
    }
    process.stdout.write(result.stdout);
  }
};

const ver = version();
if (await foreignOwner()) {
  console.error(
    `nuget.org already has Datasources from another owner. Do not rename. Stop and decide.`,
  );
  process.exit(1);
}

run("dotnet", ["restore"], csharp);
pack(ver);
const files = (await readdir(artifacts)).filter((name) => name.endsWith(`${ver}.nupkg`));
if (files.length < libraries.length) {
  console.error(`expected ${libraries.length} nupkgs, found ${files.join(", ")}`);
  process.exit(1);
}
await smoke(ver);
if (doPublish) await push(ver);
else console.log(`dry-run packed ${ver}; pass --publish to push to nuget.org`);
