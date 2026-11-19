import { Category } from "../types/datasource/category";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
export class CategoriesRepository extends CRUDRepository<Category> {
  constructor(connection: IConnection) {
    super(connection, { type: Category, entity: "Category", entityMapper: new DictionaryMapper([["Category", "categories"]]), propertyMappings: new DictionaryMapper([["parentCategoryId", "parent_category_id"]]), ids: ["id"], createProperties: ["parentCategoryId", "name"], updateProperties: ["parentCategoryId", "name"], readProperties: ["id", "parentCategoryId", "name"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, parentCategoryId: { type: "integer", nullable: true, hasDefault: false }, name: { type: "string", nullable: false, hasDefault: false } }) });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<Category>(connection, { type: Category, entity: "Category", entityMapper: new DictionaryMapper([["Category", "categories"]]), propertyMappings: new DictionaryMapper([["parentCategoryId", "parent_category_id"]]), readProperties: ["id", "parentCategoryId", "name"], schema: new Schema({ id: { type: "integer", nullable: false, hasDefault: false, isId: true }, parentCategoryId: { type: "integer", nullable: true, hasDefault: false }, name: { type: "string", nullable: false, hasDefault: false } }) });
}
