import { describe, expect, it } from "vitest";
import { answersForSetup, envFromAnswers, promptSetup, type Prompter } from "./setup.ts";

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
    confirm: async ({ default: def }) => {
      const value = next();
      return value === undefined ? def : Boolean(value);
    },
  };
};

const silent: Prompter = {
  select: async ({ default: def }) => def,
  checkbox: async ({ choices }) => choices.filter((c) => c.checked).map((c) => c.value),
  input: async ({ default: def }) => def,
  confirm: async ({ default: def }) => def,
};

describe("setup", () => {
  it("asks each question and keeps sqlite checked from the url", async () => {
    const answers = await promptSetup(silent, { DATASOURCE_URL: "sqlite:///:memory:" }, ["ts"]);
    expect(answers.providers).toContain("sqlite");
    expect(answers.url).toBe("sqlite:///:memory:");
    expect(answers.languages).toEqual(["ts"]);
    expect(answers.scaffold).toBe(false);
  });

  it("asks to create the deterministic directory when the spec is missing", async () => {
    expect((await promptSetup(silent, {}, ["ts"], undefined, true)).scaffold).toBe(true);
    expect((await promptSetup(script([["json"], "", "", false]), {}, ["ts"], undefined, true)).scaffold).toBe(
      false,
    );
  });

  it("uses explicit languages without prompting", async () => {
    expect((await promptSetup(silent, {}, ["ts", "rust"], ["csharp"])).languages).toEqual(["csharp"]);
  });

  it("prompts for languages when more than one project type exists", async () => {
    const answers = await promptSetup(
      script([["rust"], ["json"], "url", "public"]),
      {},
      ["ts", "rust"],
    );
    expect(answers.languages).toEqual(["rust"]);
    expect(answers.providers).toEqual(["json"]);
    expect(answers.url).toBe("url");
    expect(answers.schema).toBe("public");
  });

  it("ignores an invalid language choice and leaves found empty unset", async () => {
    expect(
      (await promptSetup(script(["nope", ["memory"]]), {}, ["ts", "csharp"])).languages,
    ).toBeUndefined();
    expect((await promptSetup(script([["memory"]]), {}, [])).languages).toBeUndefined();
  });

  it("defaults DATASOURCE_URL to sqlite when sqlite is selected", async () => {
    const answers = await promptSetup(script([["sqlite"]]), { INSTALLED_PROVIDERS: "mysql" }, ["ts"]);
    expect(answers.url).toBe("sqlite:///:memory:");
    expect(
      (await promptSetup(script([["json"]]), { INSTALLED_PROVIDERS: "mysql" }, ["ts"])).url,
    ).toBe("");
  });

  it("uses requested providers for --yes and pre-checks them interactively", async () => {
    expect(answersForSetup({ INSTALLED_PROVIDERS: "mysql", DATASOURCE_URL: "mysql://db" })).toEqual({
      providers: ["mysql"],
      url: "mysql://db",
      schema: "",
    });
    expect(
      answersForSetup({ INSTALLED_PROVIDERS: "mysql", DATASOURCE_URL: "mysql://db" }, ["sqlite", "postgres"]),
    ).toEqual({
      providers: ["sqlite", "postgres"],
      url: "mysql://db",
      schema: "",
    });
    const answers = await promptSetup(
      silent,
      { INSTALLED_PROVIDERS: "mysql" },
      ["ts"],
      undefined,
      false,
      ["sqlite", "postgres"],
    );
    expect(answers.providers).toEqual(["sqlite", "postgres"]);
  });

  it("writes env keys from answers", () => {
    expect(
      envFromAnswers({
        providers: ["json"],
        url: "sqlite:///:memory:",
        schema: "",
        scaffold: false,
      }),
    ).toEqual({
      INSTALLED_PROVIDERS: "json,sqlite",
      PROVIDER: "sqlite",
      DATASOURCE_URL: "sqlite:///:memory:",
    });
    expect(envFromAnswers({ providers: ["json"], url: "", schema: "", scaffold: false })).toEqual({
      INSTALLED_PROVIDERS: "json",
      PROVIDER: "json",
      DATASOURCE_URL: "",
    });
    expect(
      envFromAnswers({
        providers: ["postgres"],
        url: "postgres://localhost/db",
        schema: "public",
        scaffold: false,
      }),
    ).toEqual({
      INSTALLED_PROVIDERS: "postgres",
      PROVIDER: "postgres",
      DATASOURCE_URL: "postgres://localhost/db?schema=public",
    });
    expect(envFromAnswers({ providers: [], url: "", schema: "", scaffold: false }).INSTALLED_PROVIDERS).toBe(
      "sqlite",
    );
  });
});
