namespace Datasources.Tests;

public sealed class FirstResultReaderTests
{
    [Fact]
    public async Task ReplaysTheFirstSetThenAdvances()
    {
        var sets = new[] { new[] { new Row { ["id"] = 1 }, new Row { ["id"] = 2 } }, new[] { new Row { ["id"] = 3 } } };
        var set = 0;
        var row = 0;
        var closed = false;
        var live = new StreamingReader(
            () =>
            {
                if (row >= sets[set].Length) return Task.FromResult<Row?>(null);
                return Task.FromResult<Row?>(sets[set][row++]);
            },
            () =>
            {
                set += 1;
                row = 0;
                return Task.FromResult(set < sets.Length);
            },
            () =>
            {
                closed = true;
                return Task.CompletedTask;
            });
        var (reader, rows) = await FirstResultReader.Capture(live);
        Assert.Equal(2, rows.Count);
        Assert.Equal(new Value.Int(1), rows[0]["id"]);
        Assert.True(await reader.Next());
        Assert.Equal(new Value.Int(1), reader.Current!["id"]);
        Assert.True(await reader.Next());
        Assert.Equal(new Value.Int(2), reader.Current!["id"]);
        Assert.False(await reader.Next());
        Assert.True(await reader.NextResult());
        Assert.True(await reader.Next());
        Assert.Equal(new Value.Int(3), reader.Current!["id"]);
        await reader.DisposeAsync();
        Assert.True(closed);
    }
}
