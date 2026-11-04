use datasources::IsolationLevel;

pub struct IsolationSql;

impl IsolationSql {
    pub fn of(isolation: IsolationLevel) -> &'static str {
        match isolation {
            IsolationLevel::ReadUncommitted => "READ UNCOMMITTED",
            IsolationLevel::ReadCommitted => "READ COMMITTED",
            IsolationLevel::RepeatableRead | IsolationLevel::Serializable => "REPEATABLE READ",
        }
    }
}
