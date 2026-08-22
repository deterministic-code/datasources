namespace Datasources;

public interface IMiddlewareHost
{
    IMiddlewareHost Use(MiddlewareHook? before = null, MiddlewareHook? after = null);
}
