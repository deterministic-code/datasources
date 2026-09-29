namespace Datasources;

public static class ProviderFromUrl
{
    public static string? From(string? url)
    {
        if (string.IsNullOrWhiteSpace(url)) return null;
        var colon = url.IndexOf(':');
        if (colon <= 0) return null;
        return url[..colon].ToLowerInvariant();
    }
}
