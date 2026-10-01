#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EnrichmentPropertyType {
    #[default]
    Object,
    List,
    Dictionary,
    Custom,
}
