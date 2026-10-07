export interface LegacyMigrationResult {
  migrated_count: number;
  skipped_count: number;
  completed_at: Date | string | null;
}
