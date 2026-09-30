use crate::array_reader::ArrayReader;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::command_middleware_host::CommandMiddlewareHost;
use crate::datasource_error::DatasourceError;
use crate::native::Native;
use crate::parameter::Parameter;
use crate::reader::Reader;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::statement::Statement;
use crate::statements::SelectInsertedIdStatement;
use crate::value::Value;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

pub struct MockCommand {
    host: CommandMiddlewareHost,
    pub response: Mutex<Response>,
    pub reader_rows: Mutex<Vec<Row>>,
    execute_error: Mutex<Option<String>>,
    reader_error: Mutex<Option<String>>,
    pub execute_calls: Mutex<usize>,
    pub execute_logs: Mutex<Vec<(String, Vec<Parameter>)>>,
    pub reader_logs: Mutex<Vec<(String, Vec<Parameter>)>>,
}

impl MockCommand {
    pub fn new() -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            response: Mutex::new(Response::new(Value::Null, 0, None)),
            reader_rows: Mutex::new(Vec::new()),
            execute_error: Mutex::new(None),
            reader_error: Mutex::new(None),
            execute_calls: Mutex::new(0),
            execute_logs: Mutex::new(Vec::new()),
            reader_logs: Mutex::new(Vec::new()),
        }
    }

    pub fn set_execute_error(&self, message: impl Into<String>) {
        *self.execute_error.lock().expect("mock") = Some(message.into());
    }

    pub fn set_reader_error(&self, message: impl Into<String>) {
        *self.reader_error.lock().expect("mock") = Some(message.into());
    }
}

#[async_trait]
impl Command for MockCommand {
    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.add_middleware(middleware);
    }

    async fn execute_reader_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        self.host
            .run(text.text(), parameters, |query, params| async {
                self.reader_logs
                    .lock()
                    .expect("mock")
                    .push((query, params.unwrap_or_default()));
                if let Some(error) = &*self.reader_error.lock().expect("mock") {
                    return Err(DatasourceError::message(error.clone()));
                }
                let rows = self.reader_rows.lock().expect("mock").clone();
                Ok((
                    Box::new(ArrayReader::new(rows.clone())) as Box<dyn Reader>,
                    rows,
                ))
            })
            .await
    }

    async fn execute_reader_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let text = match statement
            .as_any()
            .downcast_ref::<SelectInsertedIdStatement>()
        {
            Some(inserted) => format!("{}.{}", inserted.table, inserted.column),
            _ => statement.get_text(),
        };
        self.execute_reader_native(&Native::new(text), parameters).await
    }

    async fn execute_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        self.host
            .run(text.text(), parameters, |query, params| async {
                self.execute_logs
                    .lock()
                    .expect("mock")
                    .push((query, params.unwrap_or_default()));
                *self.execute_calls.lock().expect("mock") += 1;
                if let Some(error) = &*self.execute_error.lock().expect("mock") {
                    return Err(DatasourceError::message(error.clone()));
                }
                Ok((self.response.lock().expect("mock").clone(), vec![]))
            })
            .await
    }
}
