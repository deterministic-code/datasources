using System.Reflection;

namespace Datasources;

internal static class DriverLoader
{
    internal static IDriver Load(ProviderCatalog catalog)
    {
        try
        {
            var type = Type.GetType($"{catalog.Module}, {catalog.Location}", throwOnError: false)
                ?? Assembly.Load(catalog.Location).GetType(catalog.Module);
            return Activator.CreateInstance(type!) is IDriver driver
                ? driver
                : throw new ProviderNotInstalledException(catalog.Name);
        }
        catch (ProviderNotInstalledException)
        {
            throw;
        }
        catch (Exception)
        {
            throw new ProviderNotInstalledException(catalog.Name);
        }
    }
}
