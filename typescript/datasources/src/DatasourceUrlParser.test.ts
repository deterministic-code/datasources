import { describe, expect, it } from "vitest";
import { DatasourceUrlParser } from "./index.ts";

describe("DatasourceUrlParser", () => {
  it("postgres url", () => {
    const url = DatasourceUrlParser.parse(
      "postgres://ds:ds@127.0.0.1:5432/datasources?schema=public&sslmode=require",
    );
    expect(url?.provider).toBe("postgres");
    expect(url?.username).toBe("ds");
    expect(url?.password).toBe("ds");
    expect(url?.host).toBe("127.0.0.1");
    expect(url?.port).toBe(5432);
    expect(url?.path).toBe("/datasources");
    expect(url?.datasource).toBe("datasources");
    expect(url?.schema).toBe("public");
    expect(url?.options.sslmode).toBe("require");
  });

  it("postgres user without password", () => {
    const url = DatasourceUrlParser.parse("postgres://ds@127.0.0.1/datasources");
    expect(url?.username).toBe("ds");
    expect(url?.password).toBeUndefined();
    expect(url?.port).toBeUndefined();
    expect(url?.schema).toBeUndefined();
    expect(url?.options).toEqual({});
  });

  it("schema only", () => {
    const url = DatasourceUrlParser.parse("postgres://127.0.0.1/datasources?schema=public");
    expect(url?.schema).toBe("public");
    expect(url?.options).toEqual({});
  });

  it("options only", () => {
    const url = DatasourceUrlParser.parse("postgres://127.0.0.1/datasources?sslmode=require");
    expect(url?.schema).toBeUndefined();
    expect(url?.options.sslmode).toBe("require");
  });

  it("repeated option keeps the last value", () => {
    const url = DatasourceUrlParser.parse(
      "postgres://127.0.0.1/datasources?sslmode=require&sslmode=disable",
    );
    expect(url?.options.sslmode).toBe("disable");
  });

  it("flag option is empty", () => {
    const url = DatasourceUrlParser.parse("postgres://127.0.0.1/datasources?readonly");
    expect(url?.options.readonly).toBe("");
  });

  it("mysql url", () => {
    const url = DatasourceUrlParser.parse("mysql://ds:secret@127.0.0.1:3306/app");
    expect(url?.provider).toBe("mysql");
    expect(url?.username).toBe("ds");
    expect(url?.password).toBe("secret");
    expect(url?.host).toBe("127.0.0.1");
    expect(url?.port).toBe(3306);
    expect(url?.datasource).toBe("app");
  });

  it("mysql host and datasource", () => {
    const url = DatasourceUrlParser.parse("mysql://db/app");
    expect(url?.host).toBe("db");
    expect(url?.datasource).toBe("app");
    expect(url?.username).toBeUndefined();
    expect(url?.port).toBeUndefined();
  });

  it("sqlite relative file", () => {
    const url = DatasourceUrlParser.parse("sqlite:///app.db");
    expect(url?.provider).toBe("sqlite");
    expect(url?.path).toBe("/app.db");
    expect(url?.datasource).toBe("app.db");
    expect(url?.host).toBeUndefined();
  });

  it("sqlite absolute file", () => {
    const url = DatasourceUrlParser.parse("sqlite:////tmp/app.db");
    expect(url?.path).toBe("//tmp/app.db");
    expect(url?.datasource).toBe("/tmp/app.db");
  });

  it("sqlite memory", () => {
    const url = DatasourceUrlParser.parse("sqlite:///:memory:");
    expect(url?.provider).toBe("sqlite");
    expect(url?.path).toBe("/:memory:");
    expect(url?.datasource).toBe(":memory:");
  });

  it("sqlite memory without path", () => {
    const url = DatasourceUrlParser.parse("sqlite://:memory:");
    expect(url?.host).toBe(":memory:");
    expect(url?.datasource).toBeUndefined();
  });

  it("json file", () => {
    const url = DatasourceUrlParser.parse("json://contacts.json");
    expect(url?.provider).toBe("json");
    expect(url?.datasource).toBe("contacts.json");
    expect(url?.host).toBeUndefined();
  });

  it("percent encoded password", () => {
    const url = DatasourceUrlParser.parse("postgres://ds:p%3Ax@127.0.0.1/datasources");
    expect(url?.password).toBe("p:x");
  });

  it("password contains colon", () => {
    const url = DatasourceUrlParser.parse("postgres://ds:a:b@127.0.0.1/datasources");
    expect(url?.username).toBe("ds");
    expect(url?.password).toBe("a:b");
  });

  it("edges", () => {
    const upper = DatasourceUrlParser.parse("Postgres://127.0.0.1/db");
    expect(upper?.provider).toBe("postgres");
    const opaque = DatasourceUrlParser.parse("a.b+c1:file");
    expect(opaque?.provider).toBe("a.b+c1");
    expect(opaque?.datasource).toBe("file");
    const slash = DatasourceUrlParser.parse("postgres://127.0.0.1/");
    expect(slash?.path).toBe("/");
    expect(slash?.datasource).toBeUndefined();
    const ipv6 = DatasourceUrlParser.parse("postgres://[::1]:5432/db");
    expect(ipv6?.host).toBe("::1");
    expect(ipv6?.port).toBe(5432);
    const hostOnly = DatasourceUrlParser.parse("postgres://[::1]/db");
    expect(hostOnly?.host).toBe("::1");
    expect(hostOnly?.port).toBeUndefined();
    const noHost = DatasourceUrlParser.parse("postgres://ds@/db");
    expect(noHost?.host).toBeUndefined();
    const portOnly = DatasourceUrlParser.parse("postgres://:5432/db");
    expect(portOnly?.host).toBeUndefined();
    expect(portOnly?.port).toBe(5432);
    const passwordOnly = DatasourceUrlParser.parse("postgres://:secret@h/db");
    expect(passwordOnly?.username).toBeUndefined();
    expect(passwordOnly?.password).toBe("secret");
    const fragment = DatasourceUrlParser.parse("postgres://h/db#frag");
    expect(fragment?.datasource).toBe("db");
    const accented = DatasourceUrlParser.parse("postgres://dé@h/db");
    expect(accented?.username).toBe("dé");
    const encoded = DatasourceUrlParser.parse("postgres://d%C3%A9@h/db");
    expect(encoded?.username).toBe("dé");
    expect(DatasourceUrlParser.parse(undefined)).toBeUndefined();
    expect(DatasourceUrlParser.parse("   ")).toBeUndefined();
    expect(DatasourceUrlParser.parse("9abc:file")).toBeUndefined();
    expect(DatasourceUrlParser.parse("postgres://ds:a%zz@h/db")).toBeUndefined();
    expect(DatasourceUrlParser.parse("postgres://[/db")).toBeUndefined();
    expect(DatasourceUrlParser.parse("postgres://[::1]x/db")).toBeUndefined();
    expect(DatasourceUrlParser.parse("postgres://[::1]:no/db")).toBeUndefined();
    expect(DatasourceUrlParser.parse("postgres://[%zz]/db")).toBeUndefined();
    const emptyUser = DatasourceUrlParser.parse("postgres://@h/db");
    expect(emptyUser?.username).toBeUndefined();
    expect(emptyUser?.host).toBe("h");
    expect(DatasourceUrlParser.parse("postgres://ds:a%@h/db")).toBeUndefined();
    expect(DatasourceUrlParser.parse("postgres://%zz@h/db")).toBeUndefined();
    expect(DatasourceUrlParser.parse("postgres://%zz:5432/db")).toBeUndefined();
    expect(DatasourceUrlParser.parse("json://%zz")).toBeUndefined();
    expect(DatasourceUrlParser.parse("postgres://%zz/db")).toBeUndefined();
    expect(DatasourceUrlParser.parse("postgres://h/db?sslmode=%zz")).toBeUndefined();
    expect(DatasourceUrlParser.parse("postgres://h/db?&sslmode=on")?.options.sslmode).toBe("on");
  });

  it("not a url", () => {
    const blank = DatasourceUrlParser.parse("");
    const text = DatasourceUrlParser.parse("not-a-url");
    const connectionString = DatasourceUrlParser.parse("Host=db;Port=1");
    expect(blank).toBeUndefined();
    expect(text).toBeUndefined();
    expect(connectionString).toBeUndefined();
  });
});
