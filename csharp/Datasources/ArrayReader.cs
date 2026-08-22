namespace Datasources;

public sealed class ArrayReader : IReader
{
    private readonly IReadOnlyList<object?> _rows;
    private int _index = -1;

    public ArrayReader(IReadOnlyList<object?> rows) => _rows = rows;

    public object? Current { get; private set; }

    public Task<bool> Next()
    {
        _index += 1;
        if (_index < _rows.Count)
        {
            Current = _rows[_index];
            return Task.FromResult(true);
        }

        Current = null;
        return Task.FromResult(false);
    }
}
