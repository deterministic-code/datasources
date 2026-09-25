#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WriteMode {
    Create,
    Update,
    Delete,
}
