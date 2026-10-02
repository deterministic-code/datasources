#[derive(Clone)]
pub struct Information {
    pub descriptions: Vec<String>,
    pub overrides: Vec<String>,
    pub change_logs: Vec<String>,
}

impl Information {
    pub fn new(
        descriptions: impl IntoIterator<Item = impl Into<String>>,
        overrides: impl IntoIterator<Item = impl Into<String>>,
        change_logs: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            descriptions: descriptions.into_iter().map(Into::into).collect(),
            overrides: overrides.into_iter().map(Into::into).collect(),
            change_logs: change_logs.into_iter().map(Into::into).collect(),
        }
    }
}
