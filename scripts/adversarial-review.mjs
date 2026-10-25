import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const RULE = ".cursor/rules/adversarial-software-engineer.mdc";
const OUT = "scripts/artifacts/adversarial";
const DEFAULT_REVIEWER = "claude -p";
const MAX_DIFF_BYTES = 200_000;

export const stripFrontmatter = (text) => text.replace(/^---\n[\s\S]*?\n---\n/, "").trim();

export const newBranches = (all, seen, base) => all.filter((branch) => branch !== base && !(branch in seen));

export const truncate = (diff, max) =>
  diff.length <= max ? diff : `${diff.slice(0, max)}\n\n[diff truncated at ${max} of ${diff.length} characters]`;

export const buildInput = (prompt, branch, diff) =>
  `${prompt}\n\n## Branch under review\n\n${branch}\n\n## Diff\n\n\`\`\`diff\n${diff}\n\`\`\`\n`;

export const feedbackPrompt = (branch, review) =>
  `Address these adversarial review findings on branch \`${branch}\`. Fix the underlying problem for each one. Do not change a test to hide a failure.\n\n${review.trim()}\n`;

export const parseSeen = (text) => (text ? JSON.parse(text) : {});

const git = (cwd, ...args) => execFileSync("git", args, { cwd, encoding: "utf8", maxBuffer: 1 << 30 });

const parseArgs = (argv) => {
  const options = { base: "main", reviewer: process.env.ADVERSARIAL_REVIEWER ?? DEFAULT_REVIEWER, maxDiff: MAX_DIFF_BYTES };
  const force = [];
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === "--dry-run") options.dryRun = true;
    else if (arg === "--all") options.all = true;
    else if (arg === "--base") options.base = argv[++i];
    else if (arg === "--reviewer") options.reviewer = argv[++i];
    else if (arg === "--max-diff-bytes") options.maxDiff = Number(argv[++i]);
    else if (arg === "--branch") options.branch = argv[++i];
    else if (arg === "--force") force.push(argv[++i]);
    else throw new Error(`unknown argument ${arg}`);
  }
  return { ...options, force };
};

const review = (options, prompt, branch, diff) => {
  const result = spawnSync(options.reviewer, {
    shell: true,
    input: buildInput(prompt, branch, truncate(diff, options.maxDiff)),
    encoding: "utf8",
    maxBuffer: 1 << 28,
  });
  if (result.status !== 0) throw new Error(result.stderr.trim() || `reviewer exited ${result.status}`);
  return result.stdout;
};

const reviewOnly = (options, cwd, log) => {
  try {
    git(cwd, "rev-parse", "--verify", "--quiet", `${options.branch}^{commit}`);
  } catch {
    throw new Error(`unknown ref ${options.branch}`);
  }
  const diff = git(cwd, "diff", `${options.base}...${options.branch}`);
  if (!diff.trim()) return log(`no changes in ${options.branch} against ${options.base}`);
  const prompt = stripFrontmatter(readFileSync(join(cwd, RULE), "utf8"));
  if (options.dryRun) {
    log(git(cwd, "diff", "--shortstat", `${options.base}...${options.branch}`).trim());
    return log(buildInput(prompt, options.branch, truncate(diff, options.maxDiff)));
  }
  log(feedbackPrompt(options.branch, review(options, prompt, options.branch, diff)));
};

export const main = (argv, cwd, log = console.log) => {
  const options = parseArgs(argv);
  if (options.branch) return reviewOnly(options, cwd, log);
  const statePath = join(git(cwd, "rev-parse", "--absolute-git-dir").trim(), "adversarial-reviewed.json");
  const firstRun = !existsSync(statePath);
  const seen = parseSeen(firstRun ? "" : readFileSync(statePath, "utf8"));
  for (const branch of options.force) delete seen[branch];

  const heads = Object.fromEntries(
    git(cwd, "for-each-ref", "--format=%(refname:short) %(objectname)", "refs/heads")
      .split("\n")
      .filter(Boolean)
      .map((line) => line.split(" ")),
  );
  const fresh = newBranches(Object.keys(heads), seen, options.base);
  const save = () => writeFileSync(statePath, `${JSON.stringify(seen, null, 2)}\n`);

  if (firstRun && !options.all && !options.dryRun) {
    for (const branch of fresh) seen[branch] = heads[branch];
    save();
    log(`baseline: recorded ${fresh.length} existing branches. Run with --all to review them.`);
    return;
  }
  if (!fresh.length) return log("no new branches");
  if (options.dryRun) return fresh.forEach((branch) => log(branch));

  const prompt = stripFrontmatter(readFileSync(join(cwd, RULE), "utf8"));
  mkdirSync(join(cwd, OUT), { recursive: true });
  for (const branch of fresh) {
    try {
      const diff = git(cwd, "diff", `${options.base}...${branch}`);
      if (diff.trim()) {
        const file = join(cwd, OUT, `${branch.replaceAll("/", "__")}.md`);
        writeFileSync(file, feedbackPrompt(branch, review(options, prompt, branch, diff)));
        log(`reviewed ${branch} -> ${file}`);
      } else log(`skipped ${branch}: no changes against ${options.base}`);
      seen[branch] = heads[branch];
      save();
    } catch (error) {
      log(`failed ${branch}: ${error.message}`);
    }
  }
};

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
if (import.meta.url === pathToFileURL(process.argv[1]).href) main(process.argv.slice(2), root);
