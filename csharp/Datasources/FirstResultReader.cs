namespace Datasources;

public sealed class FirstResultReader : IReader<Row>
{
    readonly IReadOnlyList<Row> first;
    readonly IReader<Row> live;
    int index = -1;
    bool firstSet = true;

    FirstResultReader(IReadOnlyList<Row> first, IReader<Row> live) =>
        (this.first, this.live) = (first, live);

    public static async Task<(IReader<Row> Reader, IReadOnlyList<Row> Rows)> Capture(IReader<Row> live)
    {
        var rows = new List<Row>();
        while (await live.Next() && live.Current is { } row)
            rows.Add(row);
        return (new FirstResultReader(rows, live), rows);
    }

    public Row? Current { get; private set; }

    public Task<bool> Next() =>
        firstSet
            ? Task.FromResult(Replay())
            : Live();

    public async Task<bool> NextResult()
    {
        firstSet = false;
        Current = null;
        return await live.NextResult();
    }

    public ValueTask DisposeAsync() => live.DisposeAsync();

    bool Replay()
    {
        index += 1;
        if (index < first.Count)
        {
            Current = first[index];
            return true;
        }
        Current = null;
        return false;
    }

    async Task<bool> Live()
    {
        var moved = await live.Next();
        Current = live.Current;
        return moved;
    }
}
