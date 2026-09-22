namespace UserRolesSimpleCli;

public sealed record EnableUser(string Username) : CliCommand
{
    public override Task<string> Run(App app) => SetUserEnabled.Execute(app, Username, true);
}
