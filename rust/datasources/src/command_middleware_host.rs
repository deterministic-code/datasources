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
        query: Option<&str>,
        params: Option<&[Parameter]>,
        fn_op: F,
    ) -> Result<T>
    where
        F: FnOnce(String, Option<Vec<Parameter>>) -> Fut,
        Fut: Future<Output = Result<(T, Vec<Row>)>>,
    {
        let mut query = query.unwrap_or("").trim().to_string();
        let mut params = params.map(ToOwned::to_owned);
        let middlewares = self.middlewares.lock().expect("middleware").clone();
        for middleware in &middlewares {
            if let Some((next_query, next_params)) =
                middleware.before_query(&query, params.as_deref()).await?
            {
                query = next_query;
                params = next_params;
            }
        }

        let start = Instant::now();
        match fn_op(query.clone(), params.clone()).await {
            Ok((value, rows)) => {
                let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
                for middleware in &middlewares {
                    middleware
                        .after_query(&query, params.as_deref(), &rows, elapsed_ms, None)
                        .await?;
                }
                Ok(value)
            }
            Err(error) => {
                let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
                for middleware in &middlewares {
                    middleware
                        .after_query(&query, params.as_deref(), &[], elapsed_ms, Some(&error))
                        .await?;
                }
                Err(error)
            }
        }
    }
}
