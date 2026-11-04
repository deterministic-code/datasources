namespace Datasources;

public sealed class FirstResultReader : IReader<Row>
{
    readonly IReadOnlyList<IReadOnlyList<Row>> sets;
    int set;
    int index = -1;

    FirstResultReader(IReadOnlyList<IReadOnlyList<Row>> sets) => this.sets = sets;

    public static async Task<(IReader<Row> Reader, IReadOnlyList<Row> Rows)> Capture(IReader<Row> live)
    {
        var sets = new List<IReadOnlyList<Row>>();
        try
        {
            while (true)
            {
                var rows = new List<Row>();
                while (await live.Next() && live.Current is { } row)
                    rows.Add(row);
                sets.Add(rows);
                if (!await live.NextResult())
                    break;
            }
        }
        finally
        {
            await live.DisposeAsync();
        }
        return (new FirstResultReader(sets), sets[0]);
    }

    public Row? Current { get; private set; }

    public Task<bool> Next()
    {
        if (set >= sets.Count)
        {
            Current = null;
            return Task.FromResult(false);
        }
        index += 1;
        var rows = sets[set];
        if (index < rows.Count)
        {
            Current = rows[index];
            return Task.FromResult(true);
        }
        Current = null;
        return Task.FromResult(false);
    }

    public Task<bool> NextResult()
    {
        Current = null;
        index = -1;
        set += 1;
        return Task.FromResult(set < sets.Count);
    }
}
