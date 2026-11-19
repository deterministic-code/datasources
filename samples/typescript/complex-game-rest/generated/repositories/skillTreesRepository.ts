import { SkillTree } from "../types/view/skillTree";
import { CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, Schema, TreeReadonlyEnrichmentStrategy, type IConnection } from "@deterministic-code/datasources";
const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<SkillTree>()
    .add({ type: SkillTree, entity: "SkillTree", entityMapper: new DictionaryMapper([["SkillTree", "skills"]]), propertyMappings: new DictionaryMapper([["parentSkillId", "parent_skill_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "parentSkillId", "name"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, parentSkillId: { type: "biginteger", nullable: true, hasDefault: false }, name: { type: "string", nullable: false, hasDefault: false } }) })
    .add("sqlite", { type: SkillTree, entity: "SkillTree", entityMapper: new DictionaryMapper([["SkillTree", "skills"]]), propertyMappings: new DictionaryMapper([["parentSkillId", "parent_skill_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "parentSkillId", "name"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, parentSkillId: { type: "biginteger", nullable: true, hasDefault: false }, name: { type: "string", nullable: false, hasDefault: false } }) })
    .for(connection);

export class SkillTreesRepository extends ReadonlyRepository<SkillTree> {
  constructor(connection: IConnection) {
    super(connection, readonlyOptions(connection));
    this.addEnriched({ destinationProperty: "children", sourceProperty: "id", relatedKey: "parentSkillId", type: "list" }, new ReadonlyRepository<SkillTree>(connection, readonlyOptions(connection)), new TreeReadonlyEnrichmentStrategy());
  }
  static createReadonly = (connection: IConnection) => new SkillTreesRepository(connection);
}
