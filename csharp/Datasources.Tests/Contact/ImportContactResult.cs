namespace Datasources.Tests.Contact;

internal sealed class ImportContactResult
{
    public int imported_count { get; set; }
    public int skipped_count { get; set; }
    public int failed_count { get; set; }
}
