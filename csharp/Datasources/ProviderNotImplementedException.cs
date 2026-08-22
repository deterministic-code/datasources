namespace Datasources;

public sealed class ProviderNotImplementedException : DatasourceException
{
    public string Provider { get; }

    public ProviderNotImplementedException(string provider)
        : base($"Provider \"{provider}\" is not implemented")
    {
        Provider = provider;
    }
}
