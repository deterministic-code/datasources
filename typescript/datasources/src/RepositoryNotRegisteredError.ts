export class RepositoryNotRegisteredError extends Error {
  readonly entityTypeName: string;

  constructor(entityTypeName: string) {
    super(`Repository "${entityTypeName}" is not registered`);
    this.name = "RepositoryNotRegisteredError";
    this.entityTypeName = entityTypeName;
  }
}
