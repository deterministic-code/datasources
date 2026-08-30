import { spawn } from "node:child_process";
import { readdir, readFile, writeFile } from "node:fs/promises";
import { checkbox, confirm, input, select } from "@inquirer/prompts";
import type { Prompter } from "./setup.ts";

export type Fs = {
  readFile: (path: string) => Promise<string>;
  writeFile: (path: string, text: string) => Promise<void>;
  readdir: (path: string) => Promise<string[]>;
};

export type Runner = (command: string, args: string[], cwd: string) => Promise<void>;

export const nodeFs: Fs = {
  readFile: (path) => readFile(path, "utf8"),
  writeFile: (path, text) => writeFile(path, text),
  readdir,
};

export const spawnRunner: Runner = (command, args, cwd) =>
  new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      stdio: "inherit",
      shell: process.platform === "win32",
    });
    child.on("error", reject);
    child.on("exit", (code) =>
      code === 0 ? resolve() : reject(new Error(`${command} exited ${code}`)),
    );
  });

export const inquirerPrompt: Prompter = {
  select: ({ message, choices, default: def }) => select({ message, choices, default: def }),
  checkbox: ({ message, choices }) => checkbox({ message, choices, required: true }),
  input: ({ message, default: def }) => input({ message, default: def }),
  confirm: ({ message, default: def }) => confirm({ message, default: def }),
};
