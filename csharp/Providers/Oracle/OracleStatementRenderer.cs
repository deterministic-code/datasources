using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(CreateTableStatement create) =>
        new OracleCreateTableStatementRenderer().Render(create);

    public override string Visit(CreateTriggerStatement trigger) =>
        new OracleCreateTriggerStatementRenderer().Render(trigger);

    public override string Visit(DropTableStatement drop) =>
        new OracleDropTableStatementRenderer().Render(drop);
}
