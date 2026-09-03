namespace Datasources;

public sealed record Enrichment(
    string Into,
    string From,
    string By,
    string? Select = null,
    bool Many = false);
