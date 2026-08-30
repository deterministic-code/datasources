use crate::command_middleware::CommandMiddleware;
use crate::parameter::Parameter;
use crate::reader::Reader;
use crate::response::Response;
use crate::result::Result;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Command: Send + Sync {
    fn use_middleware(&self, middleware: Arc<dyn CommandMiddleware>);
    async fn execute_reader(
        &self,
        text: &str,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>>;
    async fn execute(
        &self,
        text: &str,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response>;
}
