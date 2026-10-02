use crate::i_condition::ICondition;
use crate::parameter::Parameter;

pub trait IWhere: ICondition {
    fn parameters(&self) -> Vec<Parameter> {
        Vec::new()
    }
}
