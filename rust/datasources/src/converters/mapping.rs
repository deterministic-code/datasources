use crate::result::Result;

pub trait Mapping<TFrom, TTo> {
    fn convert(&self, from: &TFrom) -> Result<TTo>;
    fn reverse_convert(&self, to: &TTo) -> Result<TFrom>;
}
