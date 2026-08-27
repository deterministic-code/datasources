namespace Datasources;

public interface IReader<out T>
{
    T? Current { get; }
    Task<bool> Next();
}

public interface IReader : IReader<object>;
