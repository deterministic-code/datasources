export class DatasourceError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "DatasourceError";
  }
}
