use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct DatasourceUrl {
    pub provider: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub path: String,
    pub schema: Option<String>,
    pub options: HashMap<String, String>,
}

impl DatasourceUrl {
    pub fn datasource(&self) -> Option<&str> {
        match self.path.as_str() {
            "" => None,
            path if path.starts_with('/') => {
                let rest = &path[1..];
                if rest.is_empty() { None } else { Some(rest) }
            }
            path => Some(path),
        }
    }
}
