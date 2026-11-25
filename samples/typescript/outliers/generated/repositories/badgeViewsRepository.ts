import { BadgeView } from "../types/view/badgeView";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
import { BadgeAliasesRepository } from "./badgeAliasesRepository";
import { BadgeSettingsRepository } from "./badgeSettingsRepository";
import { BadgeNodesRepository } from "./badgeNodesRepository";
export class BadgeViewsRepository extends CRUDRepository<BadgeView> {
  readonly badgeAlias: BadgeAliasesRepository;
  readonly badgeSetting: BadgeSettingsRepository;
  readonly badgeNode: BadgeNodesRepository;
  constructor(connection: IConnection) {
    super(connection, { type: BadgeView, entity: "BadgeView", entityMapper: new DictionaryMapper([["BadgeView", "badges"]]), propertyMappings: new DictionaryMapper([["badgeId", "badge_id"]]), ids: ["badgeId", "code"], createProperties: ["badgeId", "code", "name"], updateProperties: ["badgeId", "code", "name"], readProperties: ["badgeId", "code", "name"], schema: new Schema({ badgeId: { type: "uuid", nullable: false, hasDefault: false, isId: true }, code: { type: "string", nullable: false, hasDefault: false, isId: true }, name: { type: "string", nullable: false, hasDefault: false } }) });
    this.badgeAlias = new BadgeAliasesRepository(connection);
    this.badgeSetting = new BadgeSettingsRepository(connection);
    this.badgeNode = new BadgeNodesRepository(connection);
    this.addEnriched({ destinationProperty: "aliases", sourceProperty: "badgeId", relatedKey: "parentBadgeId", keys: [{ sourceProperty: "badgeId", relatedKey: "parentBadgeId" }, { sourceProperty: "code", relatedKey: "parentCode" }], type: "list" }, this.badgeAlias);
    this.addEnriched({ destinationProperty: "settings", sourceProperty: "badgeId", relatedKey: "parentBadgeId", relatedProperty: "value", dictionaryKey: "name", keys: [{ sourceProperty: "badgeId", relatedKey: "parentBadgeId" }, { sourceProperty: "code", relatedKey: "parentCode" }], type: "dictionary" }, this.badgeSetting);
    this.addEnriched({ destinationProperty: "nodes", sourceProperty: "badgeId", relatedKey: "parentBadgeId", keys: [{ sourceProperty: "badgeId", relatedKey: "parentBadgeId" }, { sourceProperty: "code", relatedKey: "parentCode" }], type: "list" }, this.badgeNode);
  }
  static createReadonly(connection: IConnection) {
    const repo = new ReadonlyRepository<BadgeView>(connection, { type: BadgeView, entity: "BadgeView", entityMapper: new DictionaryMapper([["BadgeView", "badges"]]), propertyMappings: new DictionaryMapper([["badgeId", "badge_id"]]), readProperties: ["badgeId", "code", "name"], schema: new Schema({ badgeId: { type: "uuid", nullable: false, hasDefault: false, isId: true }, code: { type: "string", nullable: false, hasDefault: false, isId: true }, name: { type: "string", nullable: false, hasDefault: false } }) });
    const badgeAlias = new BadgeAliasesRepository(connection);
    const badgeSetting = new BadgeSettingsRepository(connection);
    const badgeNode = new BadgeNodesRepository(connection);
    repo.addEnriched({ destinationProperty: "aliases", sourceProperty: "badgeId", relatedKey: "parentBadgeId", keys: [{ sourceProperty: "badgeId", relatedKey: "parentBadgeId" }, { sourceProperty: "code", relatedKey: "parentCode" }], type: "list" }, badgeAlias);
    repo.addEnriched({ destinationProperty: "settings", sourceProperty: "badgeId", relatedKey: "parentBadgeId", relatedProperty: "value", dictionaryKey: "name", keys: [{ sourceProperty: "badgeId", relatedKey: "parentBadgeId" }, { sourceProperty: "code", relatedKey: "parentCode" }], type: "dictionary" }, badgeSetting);
    repo.addEnriched({ destinationProperty: "nodes", sourceProperty: "badgeId", relatedKey: "parentBadgeId", keys: [{ sourceProperty: "badgeId", relatedKey: "parentBadgeId" }, { sourceProperty: "code", relatedKey: "parentCode" }], type: "list" }, badgeNode);
    return repo;
  }
}
