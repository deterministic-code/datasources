import { DatasourceUrl } from "./DatasourceUrl.ts";

const empty: Readonly<Record<string, string>> = {};

const schemeStart = (c: string): boolean =>
  (c >= "A" && c <= "Z") || (c >= "a" && c <= "z");

const schemeChar = (c: string): boolean =>
  schemeStart(c) || (c >= "0" && c <= "9") || c === "+" || c === "." || c === "-";

const schemeColon = (url: string): number | undefined => {
  if (!schemeStart(url.charAt(0))) return undefined;
  let i = 1;
  while (i < url.length && schemeChar(url[i])) i++;
  return i < url.length && url[i] === ":" ? i : undefined;
};

const hex = (text: string): number | undefined =>
  /^[0-9a-fA-F]{2}$/.test(text) ? Number.parseInt(text, 16) : undefined;

const decode = (text: string): string | undefined => {
  const bytes: number[] = [];
  let decoded = "";
  const flush = (): void => {
    if (bytes.length === 0) return;
    decoded += new TextDecoder().decode(new Uint8Array(bytes));
    bytes.length = 0;
  };
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (c !== "%") {
      const code = text.charCodeAt(i);
      if (code <= 127) bytes.push(code);
      else {
        flush();
        decoded += c;
      }
      continue;
    }
    const byte = i + 2 < text.length ? hex(text.slice(i + 1, i + 3)) : undefined;
    if (byte === undefined) return undefined;
    bytes.push(byte);
    i += 2;
  }
  flush();
  return decoded;
};

const portOf = (text: string): number | undefined =>
  /^[0-9]+$/.test(text) ? Number.parseInt(text, 10) : undefined;

type Authority = {
  username: string | undefined;
  password: string | undefined;
  host: string | undefined;
  port: number | undefined;
  file: string | undefined;
};

const userOf = (user: string): { username: string | undefined; password: string | undefined } | undefined => {
  const colon = user.indexOf(":");
  const username = decode(colon < 0 ? user : user.slice(0, colon));
  if (username === undefined) return undefined;
  if (colon < 0) return { username: username === "" ? undefined : username, password: undefined };
  const password = decode(user.slice(colon + 1));
  if (password === undefined) return undefined;
  return { username: username === "" ? undefined : username, password };
};

const hostLiteral = (hostport: string): { host: string | undefined; port: number | undefined } | undefined => {
  const end = hostport.indexOf("]");
  if (end < 1) return undefined;
  const host = decode(hostport.slice(1, end));
  if (host === undefined) return undefined;
  if (end + 1 === hostport.length) return { host, port: undefined };
  if (hostport[end + 1] !== ":") return undefined;
  const port = portOf(hostport.slice(end + 2));
  return port === undefined ? undefined : { host, port };
};

const authorityOf = (authority: string, hasPath: boolean): Authority | undefined => {
  if (authority.length === 0)
    return { username: undefined, password: undefined, host: undefined, port: undefined, file: undefined };
  const at = authority.lastIndexOf("@");
  let hostport = authority;
  let username: string | undefined;
  let password: string | undefined;
  if (at >= 0) {
    const user = userOf(authority.slice(0, at));
    if (!user) return undefined;
    username = user.username;
    password = user.password;
    hostport = authority.slice(at + 1);
  }
  const base = { username, password, file: undefined };
  if (hostport.length === 0) return { ...base, host: undefined, port: undefined };
  if (hostport[0] === "[") {
    const literal = hostLiteral(hostport);
    return literal ? { ...base, host: literal.host, port: literal.port } : undefined;
  }
  const colon = hostport.lastIndexOf(":");
  if (colon >= 0) {
    const port = portOf(hostport.slice(colon + 1));
    if (port !== undefined) {
      const name = hostport.slice(0, colon);
      if (name.length === 0) return { ...base, host: undefined, port };
      const host = decode(name);
      return host === undefined ? undefined : { ...base, host, port };
    }
  }
  if (!hasPath && at < 0 && hostport[0] !== ":") {
    const file = decode(hostport);
    return file === undefined ? undefined : { ...base, host: undefined, port: undefined, file };
  }
  const host = decode(hostport);
  return host === undefined ? undefined : { ...base, host, port: undefined };
};

const queryOf = (
  query: string,
): { schema: string | undefined; options: Readonly<Record<string, string>> } | undefined => {
  if (query.length === 0) return { schema: undefined, options: empty };
  const options: Record<string, string> = {};
  let schema: string | undefined;
  for (const part of query.split("&")) {
    if (part.length === 0) continue;
    const eq = part.indexOf("=");
    const key = decode(eq < 0 ? part : part.slice(0, eq));
    const value = decode(eq < 0 ? "" : part.slice(eq + 1));
    if (key === undefined || value === undefined) return undefined;
    if (key === "schema") schema = value;
    else options[key] = value;
  }
  return { schema, options };
};

const partsOf = (
  rest: string,
):
  | {
      username: string | undefined;
      password: string | undefined;
      host: string | undefined;
      port: number | undefined;
      path: string;
    }
  | undefined => {
  if (!rest.startsWith("//"))
    return { username: undefined, password: undefined, host: undefined, port: undefined, path: rest };
  const body = rest.slice(2);
  const slash = body.indexOf("/");
  const authority = authorityOf(slash >= 0 ? body.slice(0, slash) : body, slash >= 0);
  if (!authority) return undefined;
  return {
    username: authority.username,
    password: authority.password,
    host: authority.host,
    port: authority.port,
    path: authority.file ?? (slash >= 0 ? body.slice(slash) : ""),
  };
};

export class DatasourceUrlParser {
  static parse(url: string | undefined): DatasourceUrl | undefined {
    if (url === undefined || url.trim() === "") return undefined;
    const colon = schemeColon(url);
    if (colon === undefined) return undefined;
    let rest = url.slice(colon + 1);
    const hash = rest.indexOf("#");
    if (hash >= 0) rest = rest.slice(0, hash);
    const queryAt = rest.indexOf("?");
    const query = queryAt >= 0 ? rest.slice(queryAt + 1) : "";
    if (queryAt >= 0) rest = rest.slice(0, queryAt);
    const parts = partsOf(rest);
    const parsed = queryOf(query);
    if (!parts || !parsed) return undefined;
    return new DatasourceUrl(
      url.slice(0, colon).toLowerCase(),
      parts.username,
      parts.password,
      parts.host,
      parts.port,
      parts.path,
      parsed.schema,
      parsed.options,
    );
  }
}
