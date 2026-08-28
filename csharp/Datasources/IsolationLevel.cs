namespace Datasources;

public enum IsolationLevel
{
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable
}
