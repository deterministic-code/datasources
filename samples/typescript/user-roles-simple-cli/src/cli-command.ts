export type CliCommand =
  | { kind: "add-user"; username: string; password: string }
  | { kind: "add-role"; name: string }
  | { kind: "add-group"; name: string }
  | { kind: "add-user-role"; username: string; role: string }
  | { kind: "add-user-group"; username: string; group: string }
  | { kind: "enable-user"; username: string }
  | { kind: "disable-user"; username: string }
  | { kind: "list-users-roles-groups" };
