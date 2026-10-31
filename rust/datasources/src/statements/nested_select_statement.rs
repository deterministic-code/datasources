use crate::mappers::TypeMap;
use crate::SelectStatement;

pub struct NestedSelectStatement {
    pub select: SelectStatement,
    pub map: TypeMap,
}
