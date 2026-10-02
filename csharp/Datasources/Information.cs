namespace Datasources;

public sealed record Information(
    IReadOnlyList<string> Descriptions,
    IReadOnlyList<string> Overrides,
    IReadOnlyList<string> ChangeLogs);
