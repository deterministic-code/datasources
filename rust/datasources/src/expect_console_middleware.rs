use crate::command_middleware::CommandMiddleware;
use crate::console_command_middleware::ConsoleCommandMiddleware;
use crate::datasource_error::DatasourceError;
use crate::parameter::Parameter;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ExpectConsoleMiddleware {
    inner: ConsoleCommandMiddleware,
    expected: Arc<Vec<String>>,
    index: Arc<Mutex<usize>>,
    error: Arc<Mutex<Option<String>>>,
}

impl ExpectConsoleMiddleware {
    pub fn new(expected: Vec<String>, now: impl Fn() -> String + Send + Sync + 'static) -> Self {
        Self::with_normalize(expected, now, signature)
    }

    pub fn with_normalize(
        expected: Vec<String>,
        now: impl Fn() -> String + Send + Sync + 'static,
        normalize: impl Fn(&str) -> String + Send + Sync + 'static,
    ) -> Self {
        let expected = Arc::new(expected);
        let index = Arc::new(Mutex::new(0usize));
        let error = Arc::new(Mutex::new(None));
        let expected_write = expected.clone();
        let index_write = index.clone();
        let error_write = error.clone();
        let inner = ConsoleCommandMiddleware::with_sink(
            move |line| {
                if error_write.lock().unwrap().is_some() {
                    return;
                }
                let actual = normalize(line);
                let i = *index_write.lock().unwrap();
                let want = expected_write
                    .get(i)
                    .map(String::as_str)
                    .unwrap_or("<missing>");
                if expected_write.get(i).map(String::as_str) != Some(actual.as_str()) {
                    *error_write.lock().unwrap() = Some(format!(
                        "console mismatch at {i}\nexpected: {want}\nactual:   {actual}"
                    ));
                    return;
                }
                *index_write.lock().unwrap() += 1;
            },
            now,
        )
        .options(HashMap::from([("isVerbose".into(), "true".into())]));
        Self {
            inner,
            expected,
            index,
            error,
        }
    }

    pub fn complete(&self) -> Result<()> {
        self.fail()?;
        let index = *self.index.lock().unwrap();
        if index != self.expected.len() {
            return Err(DatasourceError::message(format!(
                "console mismatch at {index}\nexpected: {}\nactual:   <missing>",
                self.expected[index]
            )));
        }
        Ok(())
    }

    fn fail(&self) -> Result<()> {
        match self.error.lock().unwrap().clone() {
            Some(message) => Err(DatasourceError::message(message)),
            None => Ok(()),
        }
    }
}

pub fn signature(line: &str) -> String {
    match line.rfind("] ") {
        Some(duration) if line.ends_with("ms") || line.ends_with("μs") => {
            line[..=duration].to_string()
        }
        _ => line.to_string(),
    }
}

#[async_trait]
impl CommandMiddleware for ExpectConsoleMiddleware {
    async fn before_command(
        &self,
        text: &str,
        params: Option<&[Parameter]>,
    ) -> Result<Option<(String, Option<Vec<Parameter>>)>> {
        self.inner.before_command(text, params).await?;
        self.fail()?;
        Ok(None)
    }

    async fn after_command(
        &self,
        text: &str,
        params: Option<&[Parameter]>,
        results: &[Row],
        elapsed_ms: f64,
        error: Option<&DatasourceError>,
    ) -> Result<()> {
        self.inner
            .after_command(text, params, results, elapsed_ms, error)
            .await?;
        self.fail()
    }
}
