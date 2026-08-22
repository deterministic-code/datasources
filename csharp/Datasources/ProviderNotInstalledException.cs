namespace Datasources;

public sealed class ProviderNotInstalledException : DatasourceException
{
    public string Provider { get; }

    public ProviderNotInstalledException(string provider)
        : base($"Provider \"{provider}\" is not installed")
    {
        Provider = provider;
    }
}
