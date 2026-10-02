namespace Datasources;

public interface ICondition
{
    string Accept(IConditionVisitor visitor);
}
