import { describe, expect, it } from "vitest";
import { envFromAnswers, promptSetup, type Prompter } from "./setup.ts";

const script = (answers: (string | string[])[]): Prompter => {
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
  };
};

describe("setup", () => {
  it("asks each question and keeps sqlite checked from the url", async () => {
    const answers = await promptSetup(
      {
        select: async ({ default: def }) => def,
        checkbox: async ({ choices }) => choices.filter((c) => c.checked).map((c) => c.value),
        input: async ({ default: def }) => def,
      },
      { DATASOURCE_URL: "sqlite:///:memory:" },
      ["ts"],
    );
    expect(answers.providers).toContain("sqlite");
    expect(answers.url).toBe("sqlite:///:memory:");
    expect(answers.ecosystem).toBe("ts");
  });

  it("prompts for ecosystem when more than one project type exists", async () => {
    const answers = await promptSetup(
      script(["rust", ["json"], "url", "public"]),
      {},
      ["ts", "rust"],
    );
    expect(answers.ecosystem).toBe("rust");
    expect(answers.providers).toEqual(["json"]);
    expect(answers.url).toBe("url");
    expect(answers.schema).toBe("public");
  });

  it("ignores an invalid ecosystem choice and leaves found empty unset", async () => {
    expect(
      (await promptSetup(script(["nope", ["memory"]]), {}, ["ts", "csharp"])).ecosystem,
    ).toBeUndefined();
    expect((await promptSetup(script([["memory"]]), {}, [])).ecosystem).toBeUndefined();
  });

  it("defaults DATASOURCE_URL to sqlite when sqlite is selected", async () => {
    const answers = await promptSetup(script([["sqlite"]]), { INSTALLED_PROVIDERS: "mysql" }, ["ts"]);
    expect(answers.url).toBe("sqlite:///:memory:");
    expect(
      (await promptSetup(script([["json"]]), { INSTALLED_PROVIDERS: "mysql" }, ["ts"])).url,
    ).toBe("");
  });

  it("writes env keys from answers", () => {
    expect(
      envFromAnswers({
        providers: ["json"],
        url: "sqlite:///:memory:",
        schema: "",
      }),
    ).toEqual({
      INSTALLED_PROVIDERS: "json,sqlite",
      DATASOURCE_URL: "sqlite:///:memory:",
      DATABASE_SCHEMA: "",
    });
    expect(envFromAnswers({ providers: ["json"], url: "", schema: "" })).toEqual({
      INSTALLED_PROVIDERS: "json",
      DATASOURCE_URL: "",
      DATABASE_SCHEMA: "",
    });
    expect(envFromAnswers({ providers: [], url: "", schema: "" }).INSTALLED_PROVIDERS).toBe("sqlite");
  });
});
