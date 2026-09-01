use crate::command_middleware::CommandMiddleware;
use crate::datasource_error::DatasourceError;
use crate::parameter::Parameter;
use crate::row::Row;
use crate::result::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_elapsed(elapsed_ms: f64) -> String {
    if elapsed_ms < 1.0 {
        format!("{}μs", (elapsed_ms * 1000.0).round() as i64)
    } else {
        format!("{}ms", elapsed_ms.round() as i64)
    }
}

pub fn utc_iso_millis(now: SystemTime) -> String {
    let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let days = (duration.as_secs() / 86_400) as i64;
    let rem = duration.as_secs() % 86_400;
    let hour = rem / 3600;
    let min = (rem % 3600) / 60;
    let sec = rem % 60;
    let millis = duration.subsec_millis();
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{min:02}:{sec:02}.{millis:03}Z")
}

fn civil_from_days(days: i64) -> (i64, u64, u64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

pub struct ConsoleCommandMiddleware {
    write: Arc<dyn Fn(&str) + Send + Sync>,
    now: Arc<dyn Fn() -> String + Send + Sync>,
    options: HashMap<String, String>,
}

impl ConsoleCommandMiddleware {
    pub fn new() -> Self {
        Self {
            write: Arc::new(|line| println!("{line}")),
            now: Arc::new(|| utc_iso_millis(SystemTime::now())),
            options: HashMap::new(),
        }
    }

    pub fn with_sink(
        write: impl Fn(&str) + Send + Sync + 'static,
        now: impl Fn() -> String + Send + Sync + 'static,
    ) -> Self {
        Self {
            write: Arc::new(write),
            now: Arc::new(now),
            options: HashMap::new(),
        }
    }

    pub fn options(mut self, options: HashMap<String, String>) -> Self {
        self.options = options;
        self
    }

    fn verbose(&self) -> bool {
        matches!(
            self.options.get("isVerbose").map(String::as_str),
            Some("true") | Some("1")
        )
    }

    fn line(&self, phase: &str, query: &str) -> String {
        format!("[{}]-[datasource][{phase}]-[{query}]", (self.now)())
    }
}

impl Default for ConsoleCommandMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CommandMiddleware for ConsoleCommandMiddleware {
    async fn before_query(
        &self,
        query: &str,
        params: Option<&[Parameter]>,
    ) -> Result<Option<(String, Option<Vec<Parameter>>)>> {
        (self.write)(&self.line("Start", query));
        if self.verbose() {
            (self.write)(&self.line("Request", &params_of(params)));
        }
        Ok(None)
    }

    async fn after_query(
        &self,
        query: &str,
        _params: Option<&[Parameter]>,
        results: &[Row],
        elapsed_ms: f64,
        _error: Option<&DatasourceError>,
    ) -> Result<()> {
        let mut options = self.options.clone();
        options.insert("elapsedMs".into(), format_elapsed(elapsed_ms));
        (self.write)(&format!(
            "{} {}",
            self.line("Finish", query),
            options["elapsedMs"]
        ));
        if self.verbose() {
            (self.write)(&self.line("Response", &rows_of(results)));
        }
        Ok(())
    }
}

fn params_of(params: Option<&[Parameter]>) -> String {
    let body = params
        .unwrap_or(&[])
        .iter()
        .map(|param| format!("{}: {}", param.name, param.value.wire_text()))
        .collect::<Vec<_>>()
        .join(", ");
    format!("{{{body}}}")
}

fn rows_of(results: &[Row]) -> String {
    results
        .iter()
        .map(|row| {
            let cells = row
                .iter()
                .map(|(key, value)| format!("{}: {}", key, value.wire_text()))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{cells}}}")
        })
        .collect::<Vec<_>>()
        .join(", ")
}
