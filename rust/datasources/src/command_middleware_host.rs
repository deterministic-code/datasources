use crate::command_middleware::CommandMiddleware;
use crate::parameter::Parameter;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use std::any::Any;
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
        self.middlewares
            .lock()
            .expect("middleware")
            .push(middleware);
    }

    pub async fn run<T: 'static, F, Fut>(
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
        let execute_rows = match &outcome {
            Ok((value, rows)) if rows.is_empty() => response_row(value),
            _ => Vec::new(),
        };
        let (rows, error) = match &outcome {
            Ok((_, rows)) if !rows.is_empty() => (rows.as_slice(), None),
            Ok((_, rows)) => (
                if execute_rows.is_empty() {
                    rows.as_slice()
                } else {
                    execute_rows.as_slice()
                },
                None,
            ),
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

fn response_row<T: 'static>(value: &T) -> Vec<Row> {
    (value as &dyn Any)
        .downcast_ref::<Response>()
        .map(|response| {
            vec![[
                (
                    "result".into(),
                    match &response.result {
                        Value::Null => Value::Text("null".into()),
                        other => other.clone(),
                    },
                ),
                ("rowsAffected".into(), Value::Int(response.rows_affected)),
            ]
            .into_iter()
            .collect()]
        })
        .unwrap_or_default()
}
