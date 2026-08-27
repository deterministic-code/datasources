namespace Datasources;

public sealed class MockReader : IReader
{
    private readonly IReadOnlyList<object?> _rows;
    private int _index = -1;
    public int NextCalls { get; private set; }
    public Exception? NextError { get; set; }
    public MockReader(IReadOnlyList<object?>? rows = null) => _rows = rows ?? Array.Empty<object?>();
    public object? Current { get; private set; }

    public Task<bool> Next()
    {
        NextCalls++;
        if (NextError is not null) throw NextError;
        _index++;
        if (_index < _rows.Count)
        {
            Current = _rows[_index];
            return Task.FromResult(true);
        }

        Current = null;
        return Task.FromResult(false);
    }
}
