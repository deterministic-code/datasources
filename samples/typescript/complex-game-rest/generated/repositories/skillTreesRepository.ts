import { SkillTree } from "../types/view/skillTree";
import { CRUDRepository, CRUDRepositoryBuilder, DefaultWriteEnrichmentStrategy, DictionaryMapper, ReadonlyRepository, Schema, TreeCreateEnrichmentStrategy, TreeReadonlyEnrichmentStrategy, type IConnection } from "@deterministic-code/datasources";
export const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<SkillTree>()
    .add({ type: SkillTree, entity: "SkillTree", entityMapper: new DictionaryMapper([["SkillTree", "skills"]]), propertyMappings: new DictionaryMapper([["parentSkillId", "parent_skill_id"]]), ids: ["id"], createProperties: ["parentSkillId", "name"], updateProperties: ["uuid", "parentSkillId", "name"], readProperties: ["id", "uuid", "created", "updated", "version", "parentSkillId", "name"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, parentSkillId: { type: "biginteger", nullable: true, hasDefault: false }, name: { type: "string", nullable: false, hasDefault: false } }) })
    .add("sqlite", { type: SkillTree, entity: "SkillTree", entityMapper: new DictionaryMapper([["SkillTree", "skills"]]), propertyMappings: new DictionaryMapper([["parentSkillId", "parent_skill_id"]]), ids: ["id"], createProperties: ["uuid", "created", "version", "parentSkillId", "name"], updateProperties: ["uuid", "created", "version", "parentSkillId", "name"], readProperties: ["id", "uuid", "created", "updated", "version", "parentSkillId", "name"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, parentSkillId: { type: "biginteger", nullable: true, hasDefault: false }, name: { type: "string", nullable: false, hasDefault: false } }) })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<SkillTree>()
    .add({ type: SkillTree, entity: "SkillTree", entityMapper: new DictionaryMapper([["SkillTree", "skills"]]), propertyMappings: new DictionaryMapper([["parentSkillId", "parent_skill_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "parentSkillId", "name"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, parentSkillId: { type: "biginteger", nullable: true, hasDefault: false }, name: { type: "string", nullable: false, hasDefault: false } }) })
    .add("sqlite", { type: SkillTree, entity: "SkillTree", entityMapper: new DictionaryMapper([["SkillTree", "skills"]]), propertyMappings: new DictionaryMapper([["parentSkillId", "parent_skill_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "parentSkillId", "name"], schema: new Schema({ id: { type: "biginteger", nullable: false, hasDefault: false, isId: true }, uuid: { type: "uuid", nullable: false, hasDefault: true }, created: { type: "datetime", nullable: false, hasDefault: true }, updated: { type: "datetime", nullable: false, hasDefault: true }, version: { type: "binary", nullable: false, hasDefault: true }, parentSkillId: { type: "biginteger", nullable: true, hasDefault: false }, name: { type: "string", nullable: false, hasDefault: false } }) })
    .for(connection);

export class SkillTreesRepository extends CRUDRepository<SkillTree> {
  constructor(connection: IConnection) {
    super(connection, { ...options(connection), write: new DefaultWriteEnrichmentStrategy(undefined, undefined, new TreeCreateEnrichmentStrategy()) });
    this.addEnriched({ destinationProperty: "children", sourceProperty: "id", relatedKey: "parentSkillId", type: "list" }, new CRUDRepository<SkillTree>(connection, options(connection)), new TreeReadonlyEnrichmentStrategy());
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<SkillTree>(connection, readonlyOptions(connection));
    repo.addEnriched({ destinationProperty: "children", sourceProperty: "id", relatedKey: "parentSkillId", type: "list" }, new ReadonlyRepository<SkillTree>(connection, readonlyOptions(connection)), new TreeReadonlyEnrichmentStrategy());
    return repo;
  }
}
