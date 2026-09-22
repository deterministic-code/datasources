using Datasources;

namespace UserRolesSimpleCli;

public static class Columns
{
    public static readonly TransformMapper<string, string> Names = new(
        name => name switch
        {
            "name" => "ugly_name",
            "role_id" => "bad_role_id",
            _ => name
        },
        name => name switch
        {
            "ugly_name" => "name",
            "bad_role_id" => "role_id",
            _ => name
        });
}
