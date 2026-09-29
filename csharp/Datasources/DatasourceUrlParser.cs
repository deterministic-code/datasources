namespace Datasources;

public static class DatasourceUrlParser
{
    public static DatasourceUrl? Parse(string? url)
    {
        if (string.IsNullOrWhiteSpace(url) || SchemeColon(url) is not int colon) return null;
        var rest = url[(colon + 1)..];
        var hash = rest.IndexOf('#');
        if (hash >= 0) rest = rest[..hash];
        var queryAt = rest.IndexOf('?');
        var query = queryAt >= 0 ? rest[(queryAt + 1)..] : "";
        if (queryAt >= 0) rest = rest[..queryAt];
        if (!Parts(rest, out var username, out var password, out var host, out var port, out var path))
            return null;
        if (!Query(query, out var schema, out var options)) return null;
        return new DatasourceUrl(
            url[..colon].ToLowerInvariant(),
            username,
            password,
            host,
            port,
            path,
            schema,
            options);
    }

    static bool Parts(
        string rest,
        out string? username,
        out string? password,
        out string? host,
        out int? port,
        out string path)
    {
        username = password = host = null;
        port = null;
        path = rest;
        if (!rest.StartsWith("//", StringComparison.Ordinal)) return true;
        var body = rest[2..];
        var slash = body.IndexOf('/');
        path = slash >= 0 ? body[slash..] : "";
        if (!Authority(slash >= 0 ? body[..slash] : body, slash >= 0, out username, out password, out host, out port, out var file))
            return false;
        if (file is not null) path = file;
        return true;
    }

    static bool Authority(
        string authority,
        bool hasPath,
        out string? username,
        out string? password,
        out string? host,
        out int? port,
        out string? file)
    {
        username = password = host = file = null;
        port = null;
        if (authority.Length == 0) return true;
        var at = authority.LastIndexOf('@');
        var hostport = authority;
        if (at >= 0)
        {
            if (!User(authority[..at], out username, out password)) return false;
            hostport = authority[(at + 1)..];
        }
        if (hostport.Length == 0) return true;
        if (hostport[0] == '[') return HostLiteral(hostport, out host, out port);
        var colon = hostport.LastIndexOf(':');
        if (colon >= 0 && Port(hostport[(colon + 1)..]) is int parsed)
        {
            port = parsed;
            var name = hostport[..colon];
            if (name.Length == 0) return true;
            host = Decode(name);
            return host is not null;
        }
        if (!hasPath && at < 0 && hostport[0] != ':')
        {
            file = Decode(hostport);
            return file is not null;
        }
        host = Decode(hostport);
        return host is not null;
    }

    static bool User(string user, out string? username, out string? password)
    {
        username = password = null;
        var colon = user.IndexOf(':');
        username = Decode(colon < 0 ? user : user[..colon]);
        if (username is null) return false;
        if (username is "") username = null;
        if (colon < 0) return true;
        password = Decode(user[(colon + 1)..]);
        return password is not null;
    }

    static bool HostLiteral(string hostport, out string? host, out int? port)
    {
        host = null;
        port = null;
        var end = hostport.IndexOf(']');
        if (end < 1) return false;
        host = Decode(hostport[1..end]);
        if (host is null) return false;
        if (end + 1 == hostport.Length) return true;
        if (hostport[end + 1] != ':' || Port(hostport[(end + 2)..]) is not int parsed) return false;
        port = parsed;
        return true;
    }

    static bool Query(string query, out string? schema, out IReadOnlyDictionary<string, string> options)
    {
        schema = null;
        if (query.Length == 0)
        {
            options = Empty;
            return true;
        }
        var parsed = new Dictionary<string, string>();
        foreach (var part in query.Split('&'))
        {
            if (part.Length == 0) continue;
            var eq = part.IndexOf('=');
            if (Decode(eq < 0 ? part : part[..eq]) is not string key || Decode(eq < 0 ? "" : part[(eq + 1)..]) is not string value)
            {
                options = Empty;
                return false;
            }
            if (key == "schema") schema = value;
            else parsed[key] = value;
        }
        options = parsed;
        return true;
    }

    static int? SchemeColon(string url)
    {
        if (url.Length == 0 || !SchemeStart(url[0])) return null;
        var i = 1;
        while (i < url.Length && Scheme(url[i])) i++;
        return i < url.Length && url[i] == ':' ? i : null;
    }

    static bool SchemeStart(char c) => c is >= 'A' and <= 'Z' or >= 'a' and <= 'z';

    static bool Scheme(char c) => SchemeStart(c) || c is >= '0' and <= '9' or '+' or '.' or '-';

    static int? Port(string text) =>
        text.Length > 0 && text.All(c => c is >= '0' and <= '9') && int.TryParse(text, out var port) ? port : null;

    static string? Decode(string text)
    {
        var bytes = new List<byte>();
        var decoded = new System.Text.StringBuilder();
        void Flush()
        {
            if (bytes.Count == 0) return;
            decoded.Append(System.Text.Encoding.UTF8.GetString(bytes.ToArray()));
            bytes.Clear();
        }
        for (var i = 0; i < text.Length; i++)
        {
            if (text[i] != '%')
            {
                if (text[i] <= 127)
                    bytes.Add((byte)text[i]);
                else
                {
                    Flush();
                    decoded.Append(text[i]);
                }
                continue;
            }
            if (i + 2 >= text.Length || !byte.TryParse(text.AsSpan(i + 1, 2), System.Globalization.NumberStyles.HexNumber, null, out var b))
                return null;
            bytes.Add(b);
            i += 2;
        }
        Flush();
        return decoded.ToString();
    }

    static readonly IReadOnlyDictionary<string, string> Empty = new Dictionary<string, string>();
}
