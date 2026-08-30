use crate::command_middleware::CommandMiddleware;
use crate::parameter::Parameter;
use crate::row::Row;
use crate::result::Result;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Clone, Default)]
pub struct CommandMiddlewareHost {
    middlewares: Arc<Mutex<Vec<Arc<dyn CommandMiddleware>>>>,
}

impl CommandMiddlewareHost {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn use_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.middlewares.lock().expect("middleware").push(middleware);
    }

    pub async fn run<T, F, Fut>(
        &self,
        query: &str,
        params: Option<&[Parameter]>,
        fn_op: F,
    ) -> Result<T>
    where
        F: FnOnce(String, Option<Vec<Parameter>>) -> Fut,
        Fut: Future<Output = Result<(T, Vec<Row>)>>,
    {
        let mut query = query.trim().to_string();
        let mut params = params.map(ToOwned::to_owned);
        let middlewares = self.middlewares.lock().expect("middleware").clone();
        for middleware in &middlewares {
            if let Some(next) = middleware.before_query(&query, params.as_deref()).await? {
                (query, params) = next;
            }
        }

        let start = Instant::now();
        let outcome = fn_op(query.clone(), params.clone()).await;
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        let (rows, error) = match &outcome {
            Ok((_, rows)) => (rows.as_slice(), None),
            Err(error) => (&[] as &[Row], Some(error)),
        };
        for middleware in &middlewares {
            middleware
                .after_query(&query, params.as_deref(), rows, elapsed_ms, error)
                .await?;
        }
        outcome.map(|(value, _)| value)
    }
}
