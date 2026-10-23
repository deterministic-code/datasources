import type { RepositoryModel } from "./RepositoryModel.ts";

const quote = (value: string): string => value.replaceAll("\\", "\\\\").replaceAll("\"", "\\\"");

const namesOf = (model: RepositoryModel, kind: "create" | "update"): string[] =>
  kind === "create" ? model.createProperties : model.updateProperties;

const same = (left: string[], right: string[]): boolean =>
  left.length === right.length && left.every((name, index) => name === right[index]);

const csharpList = (names: string[]): string =>
  `[${names.map((name) => `"${quote(name)}"`).join(", ")}]`;

const tsList = (names: string[]): string =>
  `[${names.map((name) => `"${quote(name)}"`).join(", ")}]`;

const rustList = (names: string[]): string =>
  `vec![${names.map((name) => `"${quote(name)}".into()`).join(", ")}]`;

const fallback = (model: RepositoryModel, kind: "create" | "update"): string[] =>
  model.writesByProvider?.sqlite?.[kind] ?? namesOf(model, kind);

export const writeList = (dialect: "csharp" | "typescript" | "rust", model: RepositoryModel, kind: "create" | "update"): string => {
  const by = model.writesByProvider;
  const listed = namesOf(model, kind);
  if (by === undefined || Object.values(by).every((item) => same(item[kind], listed)))
    return dialect === "csharp" ? csharpList(listed) : dialect === "typescript" ? tsList(listed) : rustList(listed);
  const fallbackList = fallback(model, kind);
  return dialect === "csharp"
    ? `connection.Provider switch\n        {\n${Object.entries(by)
        .filter(([engine]) => engine !== "sqlite")
        .map(([engine, item]) => `            "${quote(engine)}" => ${csharpList(item[kind])},`)
        .join("\n")}\n            _ => ${csharpList(fallbackList)},\n        }`
    : dialect === "typescript"
      ? `({ ${Object.entries(by)
          .filter(([engine]) => engine !== "sqlite")
          .map(([engine, item]) => `${engine}: ${tsList(item[kind])}`)
          .join(", ")} }[connection.provider] ?? ${tsList(fallbackList)})`
      : `match connection.provider() {\n        ${Object.entries(by)
          .filter(([engine]) => engine !== "sqlite")
          .map(([engine, item]) => `${engine === "mysql" || engine === "postgres" ? `"${engine}"` : `"${quote(engine)}"`} => ${rustList(item[kind])},`)
          .join("\n        ")}\n        _ => ${rustList(fallbackList)},\n    }`;
};
