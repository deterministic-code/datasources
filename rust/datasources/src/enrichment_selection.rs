use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
pub struct EnrichmentSelection {
    names: HashSet<String>,
}

impl EnrichmentSelection {
    pub fn none() -> Self {
        Self { names: HashSet::new() }
    }

    pub fn only(names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            names: names.into_iter().map(Into::into).collect(),
        }
    }

    pub fn includes(&self, destination: &str) -> bool {
        self.names.contains(destination)
    }
}
