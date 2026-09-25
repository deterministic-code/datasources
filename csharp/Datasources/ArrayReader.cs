namespace Datasources;

public class ArrayReader<T> : IReader<T>
{
    private readonly IReadOnlyList<T?> _rows;
    private int _index = -1;

    public ArrayReader(IReadOnlyList<T?> rows) => _rows = rows;
    public T? Current { get; private set; }

    public Task<bool> Next()
    {
        _index += 1;
        if (_index < _rows.Count)
        {
            Current = _rows[_index];
            return Task.FromResult(true);
        }

        Current = default;
        return Task.FromResult(false);
    }

    public Task<bool> NextResult()
    {
        Current = default;
        return Task.FromResult(false);
    }
}

public sealed class ArrayReader : ArrayReader<object>, IReader
{
    public ArrayReader(IReadOnlyList<object?> rows) : base(rows) { }
}
