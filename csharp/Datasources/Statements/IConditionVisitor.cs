namespace Datasources;

public interface IConditionVisitor
{
    string Visit(Where where);
    string Visit(WhereText where);
}
