import { CategoryTree } from "../types/view/categoryTree";
import { DictionaryMapper, ReadonlyRepository, Schema, TreeReadonlyEnrichmentStrategy, type IConnection } from "@deterministic-code/datasources";
export class CategoryTreesRepository extends ReadonlyRepository<CategoryTree> {
  constructor(connection: IConnection) {
    super(connection, { type: CategoryTree, entity: "CategoryTree", entityMapper: new DictionaryMapper([["CategoryTree", "categories"]]), propertyMappings: new DictionaryMapper([["parentCategoryId", "parent_category_id"]]), readProperties: ["id", "parentCategoryId", "name"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, parentCategoryId: { type: "integer", nullable: true, hasDefault: false }, name: { type: "string", nullable: false, hasDefault: false } }) });
    this.addEnriched({ destinationProperty: "children", sourceProperty: "id", relatedKey: "parentCategoryId", type: "list" }, new ReadonlyRepository<CategoryTree>(connection, { type: CategoryTree, entity: "CategoryTree", entityMapper: new DictionaryMapper([["CategoryTree", "categories"]]), propertyMappings: new DictionaryMapper([["parentCategoryId", "parent_category_id"]]), readProperties: ["id", "parentCategoryId", "name"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, parentCategoryId: { type: "integer", nullable: true, hasDefault: false }, name: { type: "string", nullable: false, hasDefault: false } }) }), new TreeReadonlyEnrichmentStrategy());
  }
  static createReadonly = (connection: IConnection) => new CategoryTreesRepository(connection);
}
