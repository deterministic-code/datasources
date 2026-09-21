#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ParameterDirection {
    #[default]
    Input,
    Output,
    InputOutput,
    ReturnValue,
}
