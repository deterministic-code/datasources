namespace Datasources;

public class MockReader<T> : IReader<T>
{
    private readonly IReadOnlyList<T?> _rows;
    private int _index = -1;
    public int NextCalls { get; private set; }
    public Exception? NextError { get; set; }
    public MockReader(IReadOnlyList<T?>? rows = null) => _rows = rows ?? Array.Empty<T?>();
    public T? Current { get; private set; }

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

        Current = default;
        return Task.FromResult(false);
    }
}

public sealed class MockReader : MockReader<object>, IReader
{
    public MockReader(IReadOnlyList<object?>? rows = null) : base(rows) { }
}
