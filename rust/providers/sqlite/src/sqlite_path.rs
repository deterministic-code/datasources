#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JournalMode {
    Wal,
    Delete,
    Truncate,
    Persist,
    Memory,
    Off,
}

impl JournalMode {
    fn parse(value: &str) -> Option<Self> {
        Some(match value.to_ascii_lowercase().as_str() {
            "wal" => Self::Wal,
            "delete" => Self::Delete,
            "truncate" => Self::Truncate,
            "persist" => Self::Persist,
            "memory" => Self::Memory,
            "off" => Self::Off,
            _ => return None,
        })
    }

    pub(crate) fn as_pragma(self) -> &'static str {
        match self {
            Self::Wal => "WAL",
            Self::Delete => "DELETE",
            Self::Truncate => "TRUNCATE",
            Self::Persist => "PERSIST",
            Self::Memory => "MEMORY",
            Self::Off => "OFF",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Synchronous {
    Off,
    Normal,
    Full,
    Extra,
}

impl Synchronous {
    fn parse(value: &str) -> Option<Self> {
        Some(match value.to_ascii_lowercase().as_str() {
            "off" => Self::Off,
            "normal" => Self::Normal,
            "full" => Self::Full,
            "extra" => Self::Extra,
            _ => return None,
        })
    }

    pub(crate) fn as_pragma(self) -> &'static str {
        match self {
            Self::Off => "OFF",
            Self::Normal => "NORMAL",
            Self::Full => "FULL",
            Self::Extra => "EXTRA",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SqliteOptions {
    pub journal_mode: Option<JournalMode>,
    pub busy_timeout: Option<u64>,
    pub pool_size: Option<u32>,
    pub synchronous: Option<Synchronous>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ResolvedSqliteOptions {
    pub journal_mode: Option<JournalMode>,
    pub busy_timeout: u64,
    pub pool_size: u32,
    pub synchronous: Option<Synchronous>,
}

const KNOWN: &[&str] = &["journal_mode", "busy_timeout", "pool_size", "synchronous"];

pub fn sqlite_path(url: &str) -> Option<(String, Result<SqliteOptions, String>)> {
    let rest = url
        .get(7..)
        .filter(|_| url.len() >= 7 && url[..7].eq_ignore_ascii_case("sqlite:"))?;
    let path = rest
        .strip_prefix("///")
        .or_else(|| rest.strip_prefix("//"))
        .unwrap_or(rest);
    let path = if path.is_empty() || path == ":memory" || path == ":memory:" {
        ":memory:"
    } else {
        path
    };
    Some(parse_filename_query(path))
}

pub fn parse_filename_query(path: &str) -> (String, Result<SqliteOptions, String>) {
    let (filename, pairs) = split_query(path);
    (filename, SqliteOptions::from_pairs(&pairs))
}

pub(crate) fn is_memory_filename(filename: &str) -> bool {
    let base = filename.split('?').next().unwrap_or(filename);
    base == ":memory:" || base == ":memory" || filename.to_ascii_lowercase().contains("mode=memory")
}

impl SqliteOptions {
    fn from_pairs(pairs: &[(String, String)]) -> Result<Self, String> {
        let mut options = Self::default();
        for (key, value) in pairs {
            match key.to_ascii_lowercase().as_str() {
                "journal_mode" => {
                    options.journal_mode = Some(JournalMode::parse(value).ok_or_else(|| {
                        format!("Invalid sqlite option journal_mode={value}")
                    })?);
                }
                "busy_timeout" => {
                    options.busy_timeout = Some(parse_busy_timeout(value)?);
                }
                "pool_size" => {
                    options.pool_size = Some(parse_pool_size(value)?);
                }
                "synchronous" => {
                    options.synchronous = Some(Synchronous::parse(value).ok_or_else(|| {
                        format!("Invalid sqlite option synchronous={value}")
                    })?);
                }
                _ => {}
            }
        }
        Ok(options)
    }

    pub(crate) fn resolved(&self, filename: &str) -> ResolvedSqliteOptions {
        let memory = is_memory_filename(filename);
        let journal_mode = if memory {
            None
        } else {
            Some(self.journal_mode.unwrap_or(JournalMode::Wal))
        };
        ResolvedSqliteOptions {
            journal_mode,
            busy_timeout: self.busy_timeout.unwrap_or(5000),
            pool_size: if memory {
                0
            } else {
                self.pool_size.unwrap_or(if journal_mode == Some(JournalMode::Wal) {
                    8
                } else {
                    0
                })
            },
            synchronous: self.synchronous,
        }
    }
}

fn parse_busy_timeout(value: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|_| format!("Invalid sqlite option busy_timeout={value}"))
}

fn parse_pool_size(value: &str) -> Result<u32, String> {
    let size = value
        .parse::<u32>()
        .map_err(|_| format!("Invalid sqlite option pool_size={value}"))?;
    if size > 64 {
        return Err(format!("Invalid sqlite option pool_size={value}"));
    }
    Ok(size)
}

fn split_query(path: &str) -> (String, Vec<(String, String)>) {
    let Some((file, query)) = path.split_once('?') else {
        return (path.to_string(), Vec::new());
    };
    let mut kept = Vec::new();
    let mut known = Vec::new();
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        if known_key(key) {
            known.push((key.to_string(), value.to_string()));
        } else {
            kept.push(pair);
        }
    }
    let filename = if kept.is_empty() {
        file.to_string()
    } else {
        format!("{file}?{}", kept.join("&"))
    };
    (filename, known)
}

fn known_key(key: &str) -> bool {
    KNOWN.iter().any(|name| name.eq_ignore_ascii_case(key))
}

#[cfg(test)]
mod tests {
    use super::{
        is_memory_filename, sqlite_path, JournalMode, SqliteOptions, Synchronous,
    };

    #[test]
    fn parses_sqlalchemy_sqlite_urls() {
        assert_eq!(
            sqlite_path("sqlite:///:memory:").as_ref().map(|(f, _)| f.as_str()),
            Some(":memory:")
        );
        assert_eq!(
            sqlite_path("sqlite://").as_ref().map(|(f, _)| f.as_str()),
            Some(":memory:")
        );
        assert_eq!(
            sqlite_path("sqlite://:memory:").as_ref().map(|(f, _)| f.as_str()),
            Some(":memory:")
        );
        assert_eq!(
            sqlite_path("sqlite:///app.db").as_ref().map(|(f, _)| f.as_str()),
            Some("app.db")
        );
        assert_eq!(
            sqlite_path("sqlite:////tmp/app.db").as_ref().map(|(f, _)| f.as_str()),
            Some("/tmp/app.db")
        );
        assert_eq!(sqlite_path("postgres://localhost/db"), None);
    }

    #[test]
    fn file_defaults_are_wal_timeout_and_pool() {
        let options = SqliteOptions::default().resolved("app.sqlite");
        assert_eq!(options.journal_mode, Some(JournalMode::Wal));
        assert_eq!(options.busy_timeout, 5000);
        assert_eq!(options.pool_size, 8);
        assert_eq!(options.synchronous, None);
    }

    #[test]
    fn memory_ignores_journal_and_pool() {
        let parsed = sqlite_path("sqlite:///:memory:?journal_mode=wal&pool_size=8").unwrap();
        assert_eq!(parsed.0, ":memory:");
        let options = parsed.1.unwrap().resolved(":memory:");
        assert_eq!(options.journal_mode, None);
        assert_eq!(options.pool_size, 0);
        assert_eq!(options.busy_timeout, 5000);
        assert!(is_memory_filename("file:iso?mode=memory&cache=shared"));
        let memory = SqliteOptions {
            journal_mode: Some(JournalMode::Wal),
            pool_size: Some(8),
            ..SqliteOptions::default()
        }
        .resolved("file:iso?mode=memory&cache=shared");
        assert_eq!(memory.journal_mode, None);
        assert_eq!(memory.pool_size, 0);
    }

    #[test]
    fn parses_each_option() {
        let (filename, options) = sqlite_path(
            "sqlite:///./app.sqlite?journal_mode=delete&busy_timeout=1000&pool_size=4&synchronous=normal",
        )
        .unwrap();
        assert_eq!(filename, "./app.sqlite");
        let options = options.unwrap();
        assert_eq!(options.journal_mode, Some(JournalMode::Delete));
        assert_eq!(options.busy_timeout, Some(1000));
        assert_eq!(options.pool_size, Some(4));
        assert_eq!(options.synchronous, Some(Synchronous::Normal));
        let resolved = options.resolved("./app.sqlite");
        assert_eq!(resolved.pool_size, 4);
        assert_eq!(resolved.journal_mode, Some(JournalMode::Delete));
    }

    #[test]
    fn delete_journal_defaults_pool_to_zero() {
        let options = SqliteOptions {
            journal_mode: Some(JournalMode::Delete),
            ..SqliteOptions::default()
        }
        .resolved("app.sqlite");
        assert_eq!(options.pool_size, 0);
    }

    #[test]
    fn pool_size_zero_disables_pool() {
        let options = sqlite_path("sqlite:///app.db?pool_size=0")
            .unwrap()
            .1
            .unwrap()
            .resolved("app.db");
        assert_eq!(options.pool_size, 0);
        assert_eq!(options.journal_mode, Some(JournalMode::Wal));
    }

    #[test]
    fn preserves_file_uri_params() {
        let (filename, options) = sqlite_path(
            "sqlite:///file:x?mode=memory&journal_mode=delete&cache=shared&pool_size=8",
        )
        .unwrap();
        assert_eq!(filename, "file:x?mode=memory&cache=shared");
        assert!(options.is_ok());
    }

    #[test]
    fn rejects_bad_option_values() {
        let err = sqlite_path("sqlite:///app.db?journal_mode=fast")
            .unwrap()
            .1
            .unwrap_err();
        assert!(err.contains("journal_mode=fast"));
        let err = sqlite_path("sqlite:///app.db?busy_timeout=no")
            .unwrap()
            .1
            .unwrap_err();
        assert!(err.contains("busy_timeout=no"));
        let err = sqlite_path("sqlite:///app.db?pool_size=65")
            .unwrap()
            .1
            .unwrap_err();
        assert!(err.contains("pool_size=65"));
        let err = sqlite_path("sqlite:///app.db?synchronous=fast")
            .unwrap()
            .1
            .unwrap_err();
        assert!(err.contains("synchronous=fast"));
    }
}
