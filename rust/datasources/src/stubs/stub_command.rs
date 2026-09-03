use super::not_implemented::fail;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::command_middleware_host::CommandMiddlewareHost;
use crate::parameter::Parameter;
use crate::reader::Reader;
use crate::response::Response;
use crate::result::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct StubCommand {
    provider: String,
    host: CommandMiddlewareHost,
}

impl StubCommand {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            host: CommandMiddlewareHost::new(),
        }
    }
}

#[async_trait]
impl Command for StubCommand {
    fn use_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.use_middleware(middleware);
    }

    async fn execute_reader(
        &self,
        text: &str,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        self.host
            .run(text, parameters, |_query, _params| async {
                Err(fail(&self.provider))
            })
            .await
    }

    async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response> {
        self.host
            .run(text, parameters, |_query, _params| async {
                Err(fail(&self.provider))
            })
            .await
    }
}

#[macro_export]
macro_rules! named_stub_command {
    ($name:ident, $provider:literal) => {
        pub struct $name {
            inner: $crate::StubCommand,
            #[allow(dead_code)]
            _connection: std::sync::Arc<dyn $crate::Connection>,
        }

        impl $name {
            pub fn new(connection: std::sync::Arc<dyn $crate::Connection>) -> Self {
                Self {
                    inner: $crate::StubCommand::new($provider),
                    _connection: connection,
                }
            }
        }

        #[async_trait::async_trait]
        impl $crate::Command for $name {
            fn use_middleware(&self, middleware: std::sync::Arc<dyn $crate::CommandMiddleware>) {
                self.inner.use_middleware(middleware);
            }

            async fn execute_reader(
                &self,
                text: &str,
                parameters: Option<&[$crate::Parameter]>,
            ) -> $crate::Result<Box<dyn $crate::Reader>> {
                self.inner.execute_reader(text, parameters).await
            }

            async fn execute(
                &self,
                text: &str,
                parameters: Option<&[$crate::Parameter]>,
            ) -> $crate::Result<$crate::Response> {
                self.inner.execute(text, parameters).await
            }
        }
    };
}
