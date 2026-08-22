use super::not_implemented::fail;
use crate::command::Command;
use crate::middleware_aware::MiddlewareAware;
use crate::middleware_hook::MiddlewareHook;
use crate::middleware_host::MiddlewareHost;
use crate::parameter::Parameter;
use crate::reader::Reader;
use crate::response::Response;
use crate::result::Result;
use async_trait::async_trait;

pub struct StubCommand {
    provider: String,
    host: MiddlewareHost,
}

impl StubCommand {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            host: MiddlewareHost::new(),
        }
    }
}

impl MiddlewareAware for StubCommand {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>) {
        self.host.use_middleware(before, after);
    }
}

#[async_trait]
impl Command for StubCommand {
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
            .run("executeReader", extras, || async { Err(fail(&self.provider)) })
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
            .run("execute", extras, || async { Err(fail(&self.provider)) })
            .await
    }
}
