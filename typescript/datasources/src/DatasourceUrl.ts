export class DatasourceUrl {
  readonly provider: string;
  readonly username: string | undefined;
  readonly password: string | undefined;
  readonly host: string | undefined;
  readonly port: number | undefined;
  readonly path: string;
  readonly schema: string | undefined;
  readonly options: Readonly<Record<string, string>>;

  constructor(
    provider: string,
    username: string | undefined,
    password: string | undefined,
    host: string | undefined,
    port: number | undefined,
    path: string,
    schema: string | undefined,
    options: Readonly<Record<string, string>>,
  ) {
    this.provider = provider;
    this.username = username;
    this.password = password;
    this.host = host;
    this.port = port;
    this.path = path;
    this.schema = schema;
    this.options = options;
  }

  get datasource(): string | undefined {
    if (this.path === "") return undefined;
    if (!this.path.startsWith("/")) return this.path;
    const rest = this.path.slice(1);
    return rest === "" ? undefined : rest;
  }
}
