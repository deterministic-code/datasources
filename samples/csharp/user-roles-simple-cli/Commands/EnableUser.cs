namespace UserRolesSimpleCli;

public sealed record EnableUser(string Username) : CliCommand
{
    public override Task<string> Run(App app) => app.SetEnabled(Username, true);
}
