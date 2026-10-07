import type { LegacyMigrationResult } from "../types/generated/views/legacyMigrationResult.ts";
import { CRUDRepository, DictionaryMapper, type IConnection } from "@datasources/core";

export class LegacyMigrationResultRepository extends CRUDRepository<LegacyMigrationResult> {
  constructor(connection: IConnection) {
    super(connection, { entity: "legacy_migration_result", entityMapper: new DictionaryMapper([["legacy_migration_result", "legacy_migration_results"]]), ids: [], readProperties: ["migrated_count", "skipped_count", "completed_at"] });
  }
}
