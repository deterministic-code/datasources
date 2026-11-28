import { ItemGraph } from "../types/view/itemGraph";
import { CRUDRepository, CRUDRepositoryBuilder, DefaultWriteEnrichmentStrategy, DictionaryMapper, GraphCreateEnrichmentStrategy, GraphReadonlyEnrichmentStrategy, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { ItemLinksRepository } from "./itemLinksRepository";
export const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ItemGraph>()
    .add({ type: ItemGraph, entity: "ItemGraph", entityMapper: new DictionaryMapper([["ItemGraph", "items"]]), propertyMappings: new DictionaryMapper([]), ids: ["id"], createProperties: ["name"], updateProperties: ["uuid", "name"], readProperties: ["id", "uuid", "created", "updated", "version", "name"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false } }) })
    .add("sqlite", { type: ItemGraph, entity: "ItemGraph", entityMapper: new DictionaryMapper([["ItemGraph", "items"]]), propertyMappings: new DictionaryMapper([]), ids: ["id"], createProperties: ["uuid", "created", "version", "name"], updateProperties: ["uuid", "created", "version", "name"], readProperties: ["id", "uuid", "created", "updated", "version", "name"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false } }) })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ItemGraph>()
    .add({ type: ItemGraph, entity: "ItemGraph", entityMapper: new DictionaryMapper([["ItemGraph", "items"]]), propertyMappings: new DictionaryMapper([]), readProperties: ["id", "uuid", "created", "updated", "version", "name"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false } }) })
    .add("sqlite", { type: ItemGraph, entity: "ItemGraph", entityMapper: new DictionaryMapper([["ItemGraph", "items"]]), propertyMappings: new DictionaryMapper([]), readProperties: ["id", "uuid", "created", "updated", "version", "name"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, name: { type: "string", nullable: false, hasDefault: false } }) })
    .for(connection);

export class ItemGraphsRepository extends CRUDRepository<ItemGraph> {
  readonly itemLinks: ItemLinksRepository;
  constructor(connection: IConnection) {
    super(connection, { ...options(connection), write: new DefaultWriteEnrichmentStrategy(undefined, undefined, new GraphCreateEnrichmentStrategy(new ItemLinksRepository(connection))) });
    this.itemLinks = new ItemLinksRepository(connection);
    this.addEnriched({ destinationProperty: "edges", sourceProperty: "id", relatedKey: "id", association: { parent: "fromItemId", related: "toItemId" }, type: "list" }, new CRUDRepository<ItemGraph>(connection, options(connection)), new GraphReadonlyEnrichmentStrategy(this.itemLinks));
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<ItemGraph>(connection, readonlyOptions(connection));
    const itemLinks = new ItemLinksRepository(connection);
    repo.addEnriched({ destinationProperty: "edges", sourceProperty: "id", relatedKey: "id", association: { parent: "fromItemId", related: "toItemId" }, type: "list" }, new ReadonlyRepository<ItemGraph>(connection, readonlyOptions(connection)), new GraphReadonlyEnrichmentStrategy(itemLinks));
    return repo;
  }
}
