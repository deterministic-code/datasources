using Datasources;

namespace UserRolesSimpleCli;

public sealed class App
{
    readonly Func<DateTime> now;
    readonly Func<string, string> hash;
    readonly Func<byte[]> token;

    public App(
        ICommand command,
        Func<DateTime>? now = null,
        Func<string, string>? hash = null,
        Func<byte[]>? token = null,
        bool procedures = true)
    {
        Command = command;
        Procedures = procedures;
        this.now = now ?? (() => DateTime.UtcNow);
        this.hash = hash ?? PasswordHash.Hash;
        this.token = token ?? (() => Guid.NewGuid().ToByteArray());
    }

    public ICommand Command { get; }

    public bool Procedures { get; }

    public Task<string> Run(CliCommand parsed) => parsed.Run(this);

    public DateTime Now() => now();

    public string Hash(string password) => hash(password);

    public byte[] Token() => token();
}
