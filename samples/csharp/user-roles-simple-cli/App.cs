using Datasources;

namespace UserRolesSimpleCli;

public sealed class App
{
    readonly Func<string, string> hash;

    public App(ICommand command, Func<string, string>? hash = null, bool procedures = true)
    {
        Command = command;
        Procedures = procedures;
        this.hash = hash ?? PasswordHash.Hash;
    }

    public ICommand Command { get; }

    public bool Procedures { get; }

    public Task<string> Run(CliCommand parsed) => parsed.Run(this);

    public string Hash(string password) => hash(password);
}
