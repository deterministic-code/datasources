using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(CreateTableStatement create) =>
        new OracleCreateTableStatementRenderer().Render(create);

    public override string Visit(AlterTableStatement alter) =>
        new OracleAlterTableStatementRenderer().Render(alter);

    public override string Visit(DropIndexStatement index) =>
        new OracleDropIndexStatementRenderer().Render(index);

    public override string Visit(DropTriggerStatement trigger) =>
        new OracleDropTriggerStatementRenderer().Render(trigger);

    public override string Visit(CreateTriggerStatement trigger) =>
        new OracleCreateTriggerStatementRenderer().Render(trigger);

    public override string Visit(DropTableStatement drop) =>
        new OracleDropTableStatementRenderer().Render(drop);
}
