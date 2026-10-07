export interface LegacyMigrationResult {
  migratedCount: number;
  skippedCount: number;
  completedAt: Date | string | null;
}
