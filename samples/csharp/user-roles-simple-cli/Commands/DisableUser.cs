namespace UserRolesSimpleCli;

public sealed record DisableUser(string Username) : CliCommand
{
    public override Task<string> Run(App app) => SetUserEnabled.Execute(app, Username, false);
}
