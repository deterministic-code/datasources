use super::not_implemented::fail;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::command_middleware_host::CommandMiddlewareHost;
use crate::connection::Connection;
use crate::native::Native;
use crate::parameter::Parameter;
use crate::reader::Reader;
use crate::response::Response;
use crate::result::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct StubCommand {
    provider: String,
    connection: Arc<dyn Connection>,
    host: CommandMiddlewareHost,
}

impl StubCommand {
    pub fn new(provider: impl Into<String>, connection: Arc<dyn Connection>) -> Self {
        Self {
            provider: provider.into(),
            connection,
            host: CommandMiddlewareHost::new(),
        }
    }
}

#[async_trait]
impl Command for StubCommand {
    fn connection(&self) -> Arc<dyn Connection> {
        Arc::clone(&self.connection)
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.add_middleware(middleware);
    }

    async fn execute_reader_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        self.host
            .run(text.text(), parameters, |_query, _params| async {
                Err(fail(&self.provider))
            })
            .await
    }

    async fn execute_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        self.host
            .run(text.text(), parameters, |_query, _params| async {
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
                    inner: $crate::StubCommand::new($provider, std::sync::Arc::clone(&connection)),
                    _connection: connection,
                }
            }
        }

        #[async_trait::async_trait]
        impl $crate::Command for $name {
            fn connection(&self) -> std::sync::Arc<dyn $crate::Connection> {
                std::sync::Arc::clone(&self._connection)
            }

            fn add_middleware(&self, middleware: std::sync::Arc<dyn $crate::CommandMiddleware>) {
                self.inner.add_middleware(middleware);
            }

            async fn execute_reader_native(
                &self,
                text: &$crate::Native,
                parameters: Option<&[$crate::Parameter]>,
            ) -> $crate::Result<Box<dyn $crate::Reader>> {
                self.inner.execute_reader_native(text, parameters).await
            }

            async fn execute_native(
                &self,
                text: &$crate::Native,
                parameters: Option<&[$crate::Parameter]>,
            ) -> $crate::Result<$crate::Response> {
                self.inner.execute_native(text, parameters).await
            }
        }
    };
}
