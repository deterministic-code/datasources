import type { LegacyMigrationResult } from "../types/generated/views/legacyMigrationResult.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class LegacyMigrationResultRepository extends CRUDRepository<LegacyMigrationResult> {
  constructor(connection: IConnection) {
    super(connection, { entity: "LegacyMigrationResult", entityMapper: new DictionaryMapper([["LegacyMigrationResult", "legacy_migration_results"]]), propertyMappings: new DictionaryMapper([["migratedCount", "migrated_count"], ["skippedCount", "skipped_count"], ["completedAt", "completed_at"]]), ids: [], readProperties: ["migratedCount", "skippedCount", "completedAt"] });
  }
}
