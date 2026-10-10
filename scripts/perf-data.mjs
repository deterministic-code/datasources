import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const stamp = "2024-01-01T00:00:00.000Z";

export const users = (n) =>
  Array.from({ length: n }, (_, i) => ({
    id: i + 1,
    username: `user${i + 1}`,
    password: "hash",
    is_enabled: true,
    created: stamp,
    updated: [1, 2, 3],
  }));

export const related = Number(process.env.PERF_RELATED ?? 10);

const childId = (contactId, index, each) => (contactId - 1) * each + index + 1;

export const contacts = (n, each = related) =>
  Array.from({ length: n }, (_, i) => {
    const id = i + 1;
    return {
      id,
      uuid: `00000000-0000-0000-0000-${id.toString(16).padStart(12, "0")}`,
      created: stamp,
      updated: stamp,
      version: [1],
      contactSourceId: 1,
      firstName: `First${id}`,
      lastName: `Last${id}`,
      email: `c${id}@ex.test`,
      notes: "",
      addresses: Array.from({ length: each }, (_, n) => {
        const child = childId(id, n, each);
        return {
          id: child,
          uuid: `a${child}`,
          created: stamp,
          updated: stamp,
          version: [1],
          addressTypeId: 1,
          line1: "1 Main",
          city: "Town",
        };
      }),
      phones: Array.from({ length: each }, (_, n) => {
        const child = childId(id, n, each);
        return {
          id: child,
          uuid: `p${child}`,
          created: stamp,
          updated: stamp,
          version: [1],
          phoneTypeId: 1,
          number: "5550000",
          label: "m",
        };
      }),
      settings: Object.fromEntries(Array.from({ length: each }, (_, n) => [`k${n}`, "light"])),
    };
  });

export const writePerfData = (dir, rows, each = related) => {
  mkdirSync(dir, { recursive: true });
  const paths = {
    "user-roles-simple-cli": join(dir, "user-roles-simple-cli.json"),
    "contacts-crud-enriched-medium-rest": join(dir, "contacts-crud-enriched-medium-rest.json"),
  };
  writeFileSync(paths["user-roles-simple-cli"], `${JSON.stringify(users(rows))}\n`);
  writeFileSync(paths["contacts-crud-enriched-medium-rest"], `${JSON.stringify(contacts(rows, each))}\n`);
  return paths;
};
