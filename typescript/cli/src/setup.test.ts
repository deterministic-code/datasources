import { describe, expect, it } from "vitest";
import { envFromAnswers, promptSetup, type Prompter } from "./setup.ts";

const script = (answers: (string | string[] | boolean)[]): Prompter => {
  const values = [...answers];
  const next = () => values.shift()!;
  return {
    select: async () => String(next()),
    checkbox: async () => {
      const value = next();
      return Array.isArray(value) ? value : [String(value)];
    },
    input: async ({ default: def }) => {
      const value = next();
      return value === undefined ? def : String(value);
    },
    confirm: async () => Boolean(next()),
  };
};

describe("setup", () => {
  it("asks each question and keeps the default provider selected", async () => {
    const answers = await promptSetup(
      {
        select: async ({ default: def }) => def,
        checkbox: async ({ choices }) => choices.filter((c) => c.checked).map((c) => c.value),
        input: async ({ default: def }) => def,
        confirm: async ({ default: def }) => def,
      },
      { DATASOURCE_PROVIDER: "sqlite" },
      ["ts"],
    );
    expect(answers.provider).toBe("sqlite");
    expect(answers.providers).toContain("sqlite");
    expect(answers.filename).toBe(":memory:");
    expect(answers.engine).toBe(true);
    expect(answers.ecosystem).toBe("ts");
  });

  it("prompts for ecosystem and skips engine when only stubs are chosen", async () => {
    const answers = await promptSetup(
      script(["rust", "json", ["json"], "url", "public"]),
      {},
      ["ts", "rust"],
    );
    expect(answers.ecosystem).toBe("rust");
    expect(answers.providers).toEqual(["json"]);
    expect(answers.url).toBe("url");
    expect(answers.schema).toBe("public");
    expect(answers.engine).toBe(false);
  });

  it("ignores an invalid ecosystem choice and leaves found empty unset", async () => {
    expect(
      (await promptSetup(script(["nope", "memory", ["memory"]]), {}, ["ts", "csharp"]))
        .ecosystem,
    ).toBeUndefined();
    expect((await promptSetup(script(["memory", ["memory"]]), {}, [])).ecosystem).toBeUndefined();
  });

  it("writes env keys from answers", () => {
    expect(
      envFromAnswers({
        provider: "sqlite",
        providers: ["json"],
        filename: "db",
        url: "",
        schema: "",
        engine: true,
      }),
    ).toEqual({
      DATASOURCE_PROVIDER: "sqlite",
      DATASOURCE_PROVIDERS: "json,sqlite",
      SQLITE_FILENAME: "db",
      DATABASE_URL: "",
      DATABASE_SCHEMA: "",
    });
  });
});
