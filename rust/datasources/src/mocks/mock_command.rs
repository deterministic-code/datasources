use crate::array_reader::ArrayReader;
use crate::command::Command;
use crate::datasource_error::DatasourceError;
use crate::middleware_aware::MiddlewareAware;
use crate::middleware_hook::MiddlewareHook;
use crate::middleware_host::MiddlewareHost;
use crate::parameter::Parameter;
use crate::reader::Reader;
use crate::record::Record;
use crate::response::Response;
use crate::result::Result;
use crate::value::Value;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct MockCommand {
    host: MiddlewareHost,
    pub response: Mutex<Response>,
    pub reader_rows: Mutex<Vec<Record>>,
    execute_error: Mutex<Option<String>>,
    reader_error: Mutex<Option<String>>,
    pub execute_calls: Mutex<usize>,
}

impl MockCommand {
    pub fn new() -> Self {
        Self {
            host: MiddlewareHost::new(),
            response: Mutex::new(Response::new(Value::Null, 0)),
            reader_rows: Mutex::new(Vec::new()),
            execute_error: Mutex::new(None),
            reader_error: Mutex::new(None),
            execute_calls: Mutex::new(0),
        }
    }

    pub fn set_execute_error(&self, message: impl Into<String>) {
        *self.execute_error.lock().expect("mock") = Some(message.into());
    }

    pub fn set_reader_error(&self, message: impl Into<String>) {
        *self.reader_error.lock().expect("mock") = Some(message.into());
    }
}

impl MiddlewareAware for MockCommand {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Command for MockCommand {
    async fn execute_reader(
        &self,
        text: Option<&str>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let extras = Some((
            text.map(ToOwned::to_owned),
            parameters.map(|items| items.to_vec()),
        ));
        self.host
            .run("executeReader", extras, || async {
                if let Some(error) = &*self.reader_error.lock().expect("mock") {
                    return Err(DatasourceError::message(error.clone()));
                }
                Ok(Box::new(ArrayReader::new(
                    self.reader_rows.lock().expect("mock").clone(),
                )) as Box<dyn Reader>)
            })
            .await
    }

    async fn execute(
        &self,
        text: Option<&str>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let extras = Some((
            text.map(ToOwned::to_owned),
            parameters.map(|items| items.to_vec()),
        ));
        self.host
            .run("execute", extras, || async {
                *self.execute_calls.lock().expect("mock") += 1;
                if let Some(error) = &*self.execute_error.lock().expect("mock") {
                    return Err(DatasourceError::message(error.clone()));
                }
                Ok(self.response.lock().expect("mock").clone())
            })
            .await
    }
}
