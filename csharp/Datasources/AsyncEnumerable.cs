namespace Datasources;

public static class AsyncEnumerable
{
    public static async IAsyncEnumerable<T> FromReader<T>(IReader reader)
    {
        while (await reader.Next())
        {
            if (reader.Current is T item)
            {
                yield return item;
            }
        }
    }

    public static async Task<List<T>> Collect<T>(IAsyncEnumerable<T> items)
    {
        var list = new List<T>();
        await foreach (var item in items)
        {
            list.Add(item);
        }

        return list;
    }
}
