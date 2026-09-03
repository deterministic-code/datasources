namespace Datasources.Tests.Contact;

internal sealed class LegacyMigrationResult
{
    public int migrated_count { get; set; }
    public int skipped_count { get; set; }
    public DateTime? completed_at { get; set; }
}
