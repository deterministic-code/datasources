using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleDropTableStatementRenderer : DropTableStatementTextRenderer
{
    public override string Render(DropTableStatement statement) =>
        $"""
        BEGIN EXECUTE IMMEDIATE 'DROP TABLE {OracleDriver.Quote(statement.Table)} CASCADE CONSTRAINTS'; EXCEPTION WHEN OTHERS THEN IF SQLCODE != -942 THEN RAISE; END IF; END;
        /
        """;
}
