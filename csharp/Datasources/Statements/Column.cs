namespace Datasources;

public sealed record Column(string Name, string Type, ColumnDefault? Default = null, NotNull? NotNull = null, bool Identity = false);
