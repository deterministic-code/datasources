import { spawnSync } from "node:child_process";

const [script = "test:e2e", ...rest] = process.argv.slice(2);
const forwarded = rest.filter((arg) => {
  if (arg !== "--verbose") return true;
  process.env.VERBOSE = "1";
  return false;
});

const result = spawnSync(
  "npm",
  [
    "run",
    script,
    "--workspaces",
    "--if-present",
    ...(forwarded.length ? ["--", ...forwarded] : []),
  ],
  { stdio: "inherit", env: process.env },
);

process.exit(result.status ?? 1);
