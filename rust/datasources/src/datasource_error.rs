use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasourceError {
    ProviderNotInstalled(String),
    ProviderNotImplemented(String),
    Message(String),
}

impl DatasourceError {
    pub fn message(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }

    pub fn not_installed(provider: impl Into<String>) -> Self {
        Self::ProviderNotInstalled(provider.into())
    }

    pub fn not_implemented(provider: impl Into<String>) -> Self {
        Self::ProviderNotImplemented(provider.into())
    }
}

impl fmt::Display for DatasourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProviderNotInstalled(provider) => {
                write!(f, "Provider \"{provider}\" is not installed")
            }
            Self::ProviderNotImplemented(provider) => {
                write!(f, "Provider \"{provider}\" is not implemented")
            }
            Self::Message(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for DatasourceError {}
