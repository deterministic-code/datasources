namespace Datasources;

public interface IReader
{
    object? Current { get; }
    Task<bool> Next();
}
