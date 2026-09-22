import type { CliCommand } from "./commands/cli-command.ts";

export const usage = `add-user <username> <password>
add-role <name>
add-group <name>
add-user-role <username> <role>
add-user-group <username> <group>
enable-user <username>
disable-user <username>
list-users-roles-groups`;

export const parse = (args: string[]): CliCommand =>
  ({
    "add-user": () =>
      args.length === 3
        ? { kind: "add-user" as const, username: args[1]!, password: args[2]! }
        : (() => {
            throw new Error("Usage: add-user <username> <password>");
          })(),
    "add-role": () =>
      args.length === 2
        ? { kind: "add-role" as const, name: args[1]! }
        : (() => {
            throw new Error("Usage: add-role <name>");
          })(),
    "add-group": () =>
      args.length === 2
        ? { kind: "add-group" as const, name: args[1]! }
        : (() => {
            throw new Error("Usage: add-group <name>");
          })(),
    "add-user-role": () =>
      args.length === 3
        ? { kind: "add-user-role" as const, username: args[1]!, role: args[2]! }
        : (() => {
            throw new Error("Usage: add-user-role <username> <role>");
          })(),
    "add-user-group": () =>
      args.length === 3
        ? { kind: "add-user-group" as const, username: args[1]!, group: args[2]! }
        : (() => {
            throw new Error("Usage: add-user-group <username> <group>");
          })(),
    "enable-user": () =>
      args.length === 2
        ? { kind: "enable-user" as const, username: args[1]! }
        : (() => {
            throw new Error("Usage: enable-user <username>");
          })(),
    "disable-user": () =>
      args.length === 2
        ? { kind: "disable-user" as const, username: args[1]! }
        : (() => {
            throw new Error("Usage: disable-user <username>");
          })(),
    "list-users-roles-groups": () =>
      args.length === 1
        ? { kind: "list-users-roles-groups" as const }
        : (() => {
            throw new Error(`Unknown command: ${args[0]}\n${usage}`);
          })(),
  }[args[0] ?? ""] ??
    (() => {
      throw new Error(args[0] ? `Unknown command: ${args[0]}\n${usage}` : usage);
    }))();
