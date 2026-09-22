namespace UserRolesSimpleCli;

public abstract record CliCommand
{
    public abstract Task<string> Run(App app);
}
