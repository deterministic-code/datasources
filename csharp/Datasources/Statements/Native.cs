namespace Datasources;

public readonly record struct Native(string Text)
{
    public static Native N(string text) => new(text);
}
