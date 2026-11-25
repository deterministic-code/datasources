import { WardView } from "../types/view/wardView";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { WardAliasesRepository } from "./wardAliasesRepository";
import { WardSettingsRepository } from "./wardSettingsRepository";
import { WardNodesRepository } from "./wardNodesRepository";
export class WardViewsRepository extends CRUDRepository<WardView> {
  readonly wardAlias: WardAliasesRepository;
  readonly wardSetting: WardSettingsRepository;
  readonly wardNode: WardNodesRepository;
  constructor(connection: IConnection) {
    super(connection, { type: WardView, entity: "WardView", entityMapper: new DictionaryMapper([["WardView", "wards"]]), propertyMappings: new DictionaryMapper([["regionId", "region_id"], ["wardId", "ward_id"]]), ids: ["regionId", "wardId"], createProperties: ["regionId", "wardId", "name"], updateProperties: ["regionId", "wardId", "name"], readProperties: ["regionId", "wardId", "name"], schema: new Schema({ regionId: { type: "integer", nullable: false, hasDefault: false, isId: true }, wardId: { type: "integer", nullable: false, hasDefault: false, isId: true }, name: { type: "string", nullable: false, hasDefault: false } }) });
    this.wardAlias = new WardAliasesRepository(connection);
    this.wardSetting = new WardSettingsRepository(connection);
    this.wardNode = new WardNodesRepository(connection);
    this.addEnriched({ destinationProperty: "aliases", sourceProperty: "regionId", relatedKey: "parentRegionId", keys: [{ sourceProperty: "regionId", relatedKey: "parentRegionId" }, { sourceProperty: "wardId", relatedKey: "parentWardId" }], type: "list" }, this.wardAlias);
    this.addEnriched({ destinationProperty: "settings", sourceProperty: "regionId", relatedKey: "parentRegionId", relatedProperty: "value", dictionaryKey: "name", keys: [{ sourceProperty: "regionId", relatedKey: "parentRegionId" }, { sourceProperty: "wardId", relatedKey: "parentWardId" }], type: "dictionary" }, this.wardSetting);
    this.addEnriched({ destinationProperty: "nodes", sourceProperty: "regionId", relatedKey: "parentRegionId", keys: [{ sourceProperty: "regionId", relatedKey: "parentRegionId" }, { sourceProperty: "wardId", relatedKey: "parentWardId" }], type: "list" }, this.wardNode);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<WardView>(connection, { type: WardView, entity: "WardView", entityMapper: new DictionaryMapper([["WardView", "wards"]]), propertyMappings: new DictionaryMapper([["regionId", "region_id"], ["wardId", "ward_id"]]), readProperties: ["regionId", "wardId", "name"], schema: new Schema({ regionId: { type: "integer", nullable: false, hasDefault: false, isId: true }, wardId: { type: "integer", nullable: false, hasDefault: false, isId: true }, name: { type: "string", nullable: false, hasDefault: false } }) });
    const wardAlias = new WardAliasesRepository(connection);
    const wardSetting = new WardSettingsRepository(connection);
    const wardNode = new WardNodesRepository(connection);
    repo.addEnriched({ destinationProperty: "aliases", sourceProperty: "regionId", relatedKey: "parentRegionId", keys: [{ sourceProperty: "regionId", relatedKey: "parentRegionId" }, { sourceProperty: "wardId", relatedKey: "parentWardId" }], type: "list" }, wardAlias);
    repo.addEnriched({ destinationProperty: "settings", sourceProperty: "regionId", relatedKey: "parentRegionId", relatedProperty: "value", dictionaryKey: "name", keys: [{ sourceProperty: "regionId", relatedKey: "parentRegionId" }, { sourceProperty: "wardId", relatedKey: "parentWardId" }], type: "dictionary" }, wardSetting);
    repo.addEnriched({ destinationProperty: "nodes", sourceProperty: "regionId", relatedKey: "parentRegionId", keys: [{ sourceProperty: "regionId", relatedKey: "parentRegionId" }, { sourceProperty: "wardId", relatedKey: "parentWardId" }], type: "list" }, wardNode);
    return repo;
  }
}
