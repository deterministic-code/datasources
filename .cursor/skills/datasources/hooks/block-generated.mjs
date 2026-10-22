#!/usr/bin/env node
// Blocks agent writes under a directory named generated.
// Cursor reads { permission }. Claude reads hookSpecificOutput.permissionDecision.
// Copied into the datasources skill as hooks/block-generated.mjs.

import { pathToFileURL } from "node:url";

export const MESSAGE = "edit types.yaml or datasource.yaml and run generate all";

const EDIT_TOOLS = new Set([
  "Write",
  "StrReplace",
  "Delete",
  "Edit",
  "EditNotebook",
  "MultiEdit",
  "NotebookEdit",
  "ApplyPatch",
]);

const SHELL_TOOLS = new Set(["Shell", "shell", "Bash", "PowerShell"]);

const PATH_KEYS = new Set([
  "path",
  "file_path",
  "filePath",
  "target_file",
  "target_notebook",
  "notebook_path",
]);

const WRITE_ANY = new Set([
  "mv",
  "rm",
  "rmdir",
  "mkdir",
  "touch",
  "truncate",
  "del",
  "md",
  "ri",
  "remove-item",
  "new-item",
  "ni",
  "set-content",
  "sc",
  "add-content",
  "ac",
  "out-file",
  "clear-content",
  "move-item",
]);

const WRITE_DEST = new Set(["cp", "install", "rsync", "ln", "copy-item", "cpi"]);

const commandName = (token) => token?.split(/[/\\]/).pop()?.toLowerCase() ?? "";

export const isGeneratedPath = (value) =>
  typeof value === "string" &&
  value
    .replaceAll("\\", "/")
    .split("/")
    .some((part) => part.toLowerCase() === "generated");

const mentionsGenerated = (value) =>
  typeof value === "string" &&
  /(?:^|[\s'"`:=])generated(?:\/|$)/i.test(value.replaceAll("\\", "/"));

const pathValues = (value, found = []) => {
  if (Array.isArray(value)) {
    for (const item of value) pathValues(item, found);
    return found;
  }
  if (!value || typeof value !== "object") return found;
  for (const [key, item] of Object.entries(value)) {
    if (PATH_KEYS.has(key) && typeof item === "string") found.push(item);
    else pathValues(item, found);
  }
  return found;
};

const strings = (value, found = []) => {
  if (typeof value === "string") found.push(value);
  else if (Array.isArray(value)) for (const item of value) strings(item, found);
  else if (value && typeof value === "object") for (const item of Object.values(value)) strings(item, found);
  return found;
};

const patchTargets = (value) =>
  strings(value).flatMap((text) => [
    ...[...text.matchAll(/^\*\*\* (?:Update|Add|Delete) File: ?(.+)$/gm)].map((match) => match[1].trim()),
    ...[...text.matchAll(/^diff --git a\/(\S+) b\//gm)].map((match) => match[1]),
  ]);

const splitSegments = (command) => {
  const segments = [];
  let current = "";
  let quote = "";
  for (let i = 0; i < command.length; i++) {
    const char = command[i];
    if (quote) {
      current += char;
      if (char === quote && command[i - 1] !== "\\") quote = "";
      continue;
    }
    if (char === "'" || char === '"') {
      quote = char;
      current += char;
      continue;
    }
    const next = command[i + 1];
    if (char === "\n" || char === ";" || char === "|" || (char === "&" && next === "&")) {
      segments.push(current);
      current = "";
      if ((char === "|" && next === "|") || (char === "&" && next === "&")) i++;
      continue;
    }
    current += char;
  }
  if (current.trim()) segments.push(current);
  return segments.map((segment) => segment.trim()).filter(Boolean);
};

const shellTokens = (segment) => {
  const tokens = [];
  let current = "";
  let quote = "";
  const stripped = segment.replace(/(^|[^>&])>{1,2}\s*(?:"[^"]*"|'[^']*'|\S+)/g, "$1 ");
  for (const char of stripped) {
    if (quote) {
      if (char === quote) quote = "";
      else current += char;
      continue;
    }
    if (char === "'" || char === '"') {
      quote = char;
      continue;
    }
    if (/\s/.test(char)) {
      if (current) tokens.push(current);
      current = "";
      continue;
    }
    current += char;
  }
  if (current) tokens.push(current);
  return tokens;
};

const redirectTargets = (segment) =>
  [...segment.matchAll(/(^|[^>&])>{1,2}\s*(?:"([^"]*)"|'([^']*)'|(\S+))/g)].map(
    (match) => match[2] ?? match[3] ?? match[4],
  );

const isCli = (segment) =>
  (/\bdatasources-cli\b/.test(segment) &&
    /\b(generate|setup|add|remove|version|migrations)\b/.test(segment)) ||
  /\bnpm\s+run\s+(cli|setup)\b/.test(segment);

const inplace = (segment) =>
  (/\bsed\b/.test(segment) && /(^|\s)--in-place|(^|\s)-[A-Za-z]*i\b/.test(segment)) ||
  (/\bperl\b/.test(segment) && /(^|\s)-[A-Za-z]*i/.test(segment));

const segmentWrites = (segment) => {
  if (redirectTargets(segment).some(isGeneratedPath)) return true;
  const of = segment.match(/\bof=(?:"([^"]*)"|'([^']*)'|(\S+))/);
  if (of && isGeneratedPath(of[1] ?? of[2] ?? of[3] ?? "")) return true;
  const tokens = shellTokens(segment);
  if (/\btee\b/.test(segment) && tokens.some(isGeneratedPath)) return true;
  if (isCli(segment)) return false;
  if (inplace(segment) && mentionsGenerated(segment)) return true;
  if (/\bgit\s+(checkout|restore|clean|reset|apply|am)\b/.test(segment) && mentionsGenerated(segment))
    return true;
  if (
    /\b(python3?|node|ruby|deno|bun)\b/.test(segment) &&
    /writeFile|write_text|write_bytes|\.write\s*\(|['"]w['"]/.test(segment) &&
    mentionsGenerated(segment)
  )
    return true;
  if (/\bfind\b/.test(segment) && /-(delete|exec)\b/.test(segment) && mentionsGenerated(segment))
    return true;
  const start = tokens.findIndex((token) => !/^[A-Za-z_][A-Za-z0-9_]*=/.test(token));
  if (start < 0) return false;
  let name = commandName(tokens[start]);
  let args = tokens.slice(start + 1);
  if (name === "sudo") {
    const rest = args.filter((token) => !token.startsWith("-"));
    name = commandName(rest[0] ?? "");
    args = rest.slice(1);
  }
  const paths = args.filter((token) => token !== "--" && !token.startsWith("-"));
  if (WRITE_ANY.has(name)) return paths.some(isGeneratedPath);
  if (!WRITE_DEST.has(name)) return false;
  const targetFlag = args.findIndex(
    (token) => token === "-t" || token === "--target-directory" || token.startsWith("--target-directory="),
  );
  if (targetFlag >= 0) {
    const flag = args[targetFlag];
    const value = flag.includes("=") ? flag.slice(flag.indexOf("=") + 1) : args[targetFlag + 1];
    if (isGeneratedPath(value ?? "")) return true;
  }
  return isGeneratedPath(paths.at(-1) ?? "");
};

export const shellWritesGenerated = (command) =>
  typeof command === "string" && command.trim() !== "" && splitSegments(command).some(segmentWrites);

const inputOf = (data) => {
  const input = data.tool_input ?? {};
  if (typeof input !== "string") return input;
  return JSON.parse(input);
};

export const blocks = (data) => {
  if (!data || typeof data !== "object") return false;
  const tool = String(data.tool_name ?? "");
  const input = inputOf(data);
  if (EDIT_TOOLS.has(tool)) {
    if (pathValues(input).some(isGeneratedPath)) return true;
    return (tool === "ApplyPatch" || tool === "MultiEdit") && patchTargets(input).some(isGeneratedPath);
  }
  if (SHELL_TOOLS.has(tool) || (tool === "" && typeof data.command === "string")) {
    const command =
      (input && typeof input.command === "string" && input.command) ||
      (typeof data.command === "string" ? data.command : "");
    return shellWritesGenerated(command);
  }
  return false;
};

const respond = (data, deny) => {
  if (typeof data.hook_event_name === "string") {
    if (!deny) return;
    process.stdout.write(
      `${JSON.stringify({
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: MESSAGE,
        },
      })}\n`,
    );
    return;
  }
  process.stdout.write(
    `${JSON.stringify(
      deny
        ? { permission: "deny", user_message: MESSAGE, agent_message: MESSAGE }
        : { permission: "allow" },
    )}\n`,
  );
};

const run = async () => {
  const raw = await new Promise((resolve) => {
    let body = "";
    process.stdin.setEncoding("utf8");
    process.stdin.on("data", (chunk) => {
      body += chunk;
    });
    process.stdin.on("end", () => resolve(body));
  });
  if (!raw.trim()) {
    respond({}, false);
    return;
  }
  let data;
  try {
    data = JSON.parse(raw);
  } catch {
    respond({}, true);
    return;
  }
  try {
    respond(data, blocks(data));
  } catch {
    respond(data, true);
  }
};

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) await run();
