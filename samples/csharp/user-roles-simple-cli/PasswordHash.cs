namespace UserRolesSimpleCli;

public static class PasswordHash
{
    public static string Hash(string password) => BCrypt.Net.BCrypt.HashPassword(password, 4);
}
